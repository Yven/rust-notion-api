pub mod contents;
pub mod metas;
pub mod relationships;
pub mod entity_linked;

use sea_orm::{TransactionTrait, DatabaseConnection, ActiveModelTrait, Set, EntityTrait, ColumnTrait, QueryFilter, ModelTrait, DatabaseTransaction};
use chrono::DateTime;
use anyhow::Result;

use crate::error::CommErr;
use super::notion::page;


pub async fn is_exist(db: &DatabaseConnection, slug: String) -> Result<bool> {
    let model = contents::Entity::find().filter(contents::Column::Slug.eq(Some(slug))).one(db).await?;
    match model {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn new_article(db: &DatabaseConnection, page: page::Page) -> Result<()> {
    db.transaction::<_, (), CommErr>(|txn| {
        Box::pin(async move {
            let content_res = contents::ActiveModel {
                title: Set(Some(page.title.clone())),
                slug: Set(Some(page.search_property("Slug").ok_or(CommErr::FormatErr("Slug"))?.to_string())),
                created: Set(DateTime::parse_from_rfc3339(&page.created_time)?.timestamp() as u32),
                modified: Set(DateTime::parse_from_rfc3339(&page.edited_time)?.timestamp() as u32),
                text: Set(format!("<!--markdown-->{}", page.content.to_string())),
                author_id: Set(1),
                ctype: Set("post".to_owned()),
                status: Set("publish".to_owned()),
                allow_comment: Set("1".to_owned()),
                allow_ping: Set("0".to_owned()),
                allow_feed: Set("1".to_owned()),
                ..Default::default()
            }.insert(txn)
            .await?;

            let tag_list = page.search_property("Tag").ok_or(CommErr::FormatErr("Tag"))?.to_array()?;
            for tag in tag_list.into_iter() {
                update_or_create_metas(txn, tag, "tag".to_string(), content_res.cid).await?;
            }

            let category = page.search_property("Category").ok_or(CommErr::FormatErr("Category"))?.to_string();
            update_or_create_metas(txn, category, "category".to_string(), content_res.cid).await?;

            Ok(())
        })
    })
    .await?;

    Ok(())
}

async fn increase_metas(db: &DatabaseTransaction, model: metas::Model) -> Result<()> {
    let count = model.count;
    let mut model: metas::ActiveModel = model.into();
    model.count = Set(count + 1);
    model.update(db).await?;

    Ok(())
}

async fn decrease_metas(db: &DatabaseTransaction, model: metas::Model, cid: u32) -> Result<()> {
    let count = model.count;
    let model = if count > 0 {
        let mut model: metas::ActiveModel = model.into();
        model.count = Set(count - 1);
        model.update(db).await?
    } else { model };

    relationships::ActiveModel {
        cid: Set(cid),
        mid: Set(model.mid)
    }.delete(db)
    .await?;

    Ok(())
}

async fn create_metas(db: &DatabaseTransaction, name: String, mtype: String, cid: u32) -> Result<()> {
    use md5::{Md5, Digest};
    let mut hasher = Md5::new();
    hasher.update(name.clone());
    let metas_res = metas::ActiveModel {
        name: Set(Some(name)),
        slug: Set(Some(format!("{:x}", hasher.finalize()))),
        mtype: Set(Some(mtype)),
        count: Set(1),
        order: Set(0),
        parent: Set(0),
        ..Default::default()
    }.insert(db)
    .await?;

    relationships::ActiveModel {
        cid: Set(cid),
        mid: Set(metas_res.mid)
    }.insert(db)
    .await?;

    Ok(())
}

pub async fn update_or_create_metas(db: &DatabaseTransaction, name: String, mtype: String, cid: u32) -> Result<()> {
    let metas_model = metas::Entity::find()
        .filter(metas::Column::Name.eq(name.clone()))
        .filter(metas::Column::Mtype.eq(mtype.clone()))
        .one(db)
        .await?;
    match metas_model {
        Some(model) => {
            increase_metas(db, model).await?;
        },
        None => {
            create_metas(db, name, mtype, cid).await?;
        },
    }

    Ok(())
}

pub async fn update_article(db: &DatabaseConnection, page: page::Page) -> Result<()> {
    db.transaction::<_, (), CommErr>(|txn| {
        Box::pin(async move {
            let slug = page.search_property("Slug").ok_or(CommErr::FormatErr("Slug"))?.to_string();
            let contents_model = contents::Entity::find().filter(contents::Column::Slug.eq(Some(slug.clone()))).one(txn).await?.ok_or(CommErr::CErr("page do not exist"))?;
            let page_tag_list = page.search_property("Tag").ok_or(CommErr::FormatErr("Tag"))?.to_array()?;
            let category = page.search_property("Category").ok_or(CommErr::FormatErr("Category"))?.to_string();
            let metas_model = contents_model.find_linked(entity_linked::ContentToMeta).all(txn).await?;

            let mut db_tag_list: Vec<String> = Vec::new();
            for m in metas_model.iter() {
                let metas_name = m.name.as_ref().unwrap();
                db_tag_list.push(m.name.as_ref().unwrap().to_string());
                if !page_tag_list.contains(metas_name) {
                    decrease_metas(txn, m.clone(), contents_model.cid).await?;
                }
            }

            if !db_tag_list.contains(&category) {
                update_or_create_metas(txn, category.clone(), "category".to_string(), contents_model.cid).await?;
            }

            for t in page_tag_list.iter() {
                if !db_tag_list.contains(t) {
                    update_or_create_metas(txn, t.to_string(), "tag".to_string(), contents_model.cid).await?;
                }
            }

            let mut contents_model: self::contents::ActiveModel = contents_model.into();
            contents_model.title = Set(Some(page.title.clone()));
            contents_model.modified = Set(DateTime::parse_from_rfc3339(&page.edited_time)?.timestamp() as u32);
            contents_model.text = Set(format!("<!--markdown-->{}", page.content.to_string()));
            contents_model.update(txn).await?;

            Ok(())
        })
    })
    .await?;

    Ok(())
}