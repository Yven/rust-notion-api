use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "typecho_contents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cid: i32,
    pub title: String,
    pub slug: String,
    pub created: i64,
    pub modified: i64,
    pub text: String,
    pub order: i32,
    #[sea_orm(column_name = "authorId")]
    pub author_id: i32,
    pub template: String,
    #[sea_orm(column_name = "type")]
    pub ctype: String,
    pub status: String,
    pub password: String,
    #[sea_orm(column_name = "commentsNum")]
    pub comments_num: i32,
    #[sea_orm(column_name = "allowComment")]
    pub allow_comment: String,
    #[sea_orm(column_name = "allowPing")]
    pub allow_ping: String,
    #[sea_orm(column_name = "allowFeed")]
    pub allow_feed: String,
    pub parent: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}