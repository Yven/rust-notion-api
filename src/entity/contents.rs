use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "typecho_contents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cid: u32,
    #[sea_orm(nullable)]
    pub title: Option<String>,
    #[sea_orm(unique, nullable)]
    pub slug: Option<String>,
    pub created: u32,
    pub modified: u32,
    pub text: String,
    pub order: u32,
    #[sea_orm(column_name = "authorId")]
    pub author_id: u32,
    #[sea_orm(nullable)]
    pub template: Option<String>,
    #[sea_orm(column_name = "type")]
    pub ctype: String,
    pub status: String,
    #[sea_orm(nullable)]
    pub password: Option<String>,
    #[sea_orm(column_name = "commentsNum")]
    pub comments_num: u32,
    #[sea_orm(column_name = "allowComment")]
    pub allow_comment: String,
    #[sea_orm(column_name = "allowPing")]
    pub allow_ping: String,
    #[sea_orm(column_name = "allowFeed")]
    pub allow_feed: String,
    pub parent: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::relationships::Entity> for Entity {
    fn to() -> RelationDef {
        super::relationships::Relation::Meta.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::relationships::Relation::Content.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}