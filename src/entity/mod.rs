pub mod contents;
pub mod metas;
pub mod relationships;

use sea_orm::{TransactionTrait, DatabaseConnection, ActiveModelTrait, Set, EntityTrait, ColumnTrait, QueryFilter};
use md5::{Md5, Digest};
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

            let tag_list = page.search_property("Tag").ok_or(CommErr::FormatErr("Tag"))?.to_string_array()?;
            let mut noexist_tag_list = Vec::new();
            for tag in tag_list.into_iter() {
                let metas_model = metas::Entity::find().filter(metas::Column::Name.eq(tag.clone())).one(txn).await?;

                match metas_model {
                    Some(model) => {
                        let count = model.count;
                        let mut model: metas::ActiveModel = model.into();
                        model.count = Set(count + 1);
                        model.update(txn).await?;
                    },
                    None => noexist_tag_list.push((tag, "tag".to_string())),
                }
            }

            let category = page.search_property("Category").ok_or(CommErr::FormatErr("Category"))?.to_string();
            let metas_model = metas::Entity::find().filter(metas::Column::Name.eq(category.clone())).one(txn).await?;
            match metas_model {
                Some(model) => {
                    let count = model.count;
                    let mut model: metas::ActiveModel = model.into();
                    model.count = Set(count + 1);
                    model.update(txn).await?;
                },
                None => noexist_tag_list.push((category, "category".to_string())),
            }

            for tag in noexist_tag_list {
                let mut hasher = Md5::new();
                hasher.update(tag.0.clone());
                let metas_res = metas::ActiveModel {
                    name: Set(Some(tag.0)),
                    slug: Set(Some(format!("{:?}", hasher.finalize()))),
                    mtype: Set(Some(tag.1)),
                    count: Set(1),
                    order: Set(0),
                    parent: Set(0),
                    ..Default::default()
                }.insert(txn)
                .await?;

                relationships::ActiveModel {
                    cid: Set(content_res.cid),
                    mid: Set(metas_res.mid)
                }.insert(txn)
                .await?;
            }

            Ok(())
        })
    })
    .await?;

    Ok(())
}

pub async fn update_article(db: &DatabaseConnection, page: page::Page) -> Result<()> {
    db.transaction::<_, (), CommErr>(|txn| {
        Box::pin(async move {
            let slug = page.search_property("Slug").ok_or(CommErr::FormatErr("Slug"))?.to_string();
            let contents_model = contents::Entity::find().filter(contents::Column::Slug.eq(Some(slug.clone()))).one(txn).await?.ok_or(CommErr::CErr("page do not exist"))?;

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