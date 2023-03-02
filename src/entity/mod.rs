pub mod contents;
pub mod metas;
pub mod relationships;

use sea_orm::{TransactionTrait, DatabaseConnection, ActiveModelTrait, Set, EntityTrait, ColumnTrait, QueryFilter};
use crate::error::CommErr;

use super::notion::page;
use chrono::DateTime;


pub async fn new_article(db: &DatabaseConnection, page: page::Page, page_content: String) -> anyhow::Result<()> {
    db.transaction::<_, (), CommErr>(|txn| {
        Box::pin(async move {
            let content_res = contents::ActiveModel {
                title: Set(page.title.clone()),
                slug: Set(page.search_property("Slug")?[0].0.clone()),
                created: Set(DateTime::parse_from_rfc3339(&page.created_time)?.timestamp()),
                modified: Set(DateTime::parse_from_rfc3339(&page.edited_time)?.timestamp()),
                text: Set(page_content),
                author_id: Set(1),
                ctype: Set("post".to_owned()),
                status: Set("publish".to_owned()),
                allow_comment: Set("1".to_owned()),
                allow_ping: Set("0".to_owned()),
                allow_feed: Set("1".to_owned()),
                parent: Set(0),
                ..Default::default()
            }
            .insert(txn)
            .await?;

            let tag_list = page.search_property("Tag")?;
            let mut noexist_tag_list = Vec::new();
            for tag in tag_list.iter() {
                let metas_model = metas::Entity::find().filter(metas::Column::Name.eq(tag.0.clone())).one(txn).await?;

                match metas_model {
                    Some(model) => {
                        let count = model.count;
                        let mut model: metas::ActiveModel = model.into();
                        model.count = Set(count + 1);
                        model.update(txn).await?;
                    },
                    None => noexist_tag_list.push((tag.0.as_str(), tag.1.as_str(), "tag")),
                }
            }

            let category = page.search_property("Category")?;
            let metas_model = metas::Entity::find().filter(metas::Column::Name.eq(category[0].0.clone())).one(txn).await?;
            match metas_model {
                Some(model) => {
                    let count = model.count;
                    let mut model: metas::ActiveModel = model.into();
                    model.count = Set(count + 1);
                    model.update(txn).await?;
                },
                None => noexist_tag_list.push((category[0].0.as_str(), category[0].1.as_str(), "category")),
            }

            for tag in noexist_tag_list {
                let metas_res = metas::ActiveModel {
                    name: Set(tag.0.to_string()),
                    slug: Set(tag.1.to_string()),
                    mtype: Set(tag.2.to_string()),
                    count: Set(1),
                    order: Set(0),
                    parent: Set(0),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                relationships::ActiveModel {
                    cid: Set(content_res.cid),
                    mid: Set(metas_res.mid)
                }
                .insert(txn)
                .await?;
            }

            Ok(())
        })
    })
    .await?;

    Ok(())
}