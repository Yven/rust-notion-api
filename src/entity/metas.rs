use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "typecho_metas")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub mid: i32,
    pub name: String,
    pub slug: String,
    #[sea_orm(column_name = "type")]
    pub mtype: String,
    pub description: String,
    pub count: i32,
    pub order: i32,
    pub parent: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}