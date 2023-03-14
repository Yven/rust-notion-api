use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "typecho_metas")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub mid: u32,
    #[sea_orm(nullable)]
    pub name: Option<String>,
    #[sea_orm(nullable)]
    pub slug: Option<String>,
    #[sea_orm(column_name = "type", nullable)]
    pub mtype: Option<String>,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    pub count: u32,
    pub order: u32,
    pub parent: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::relationships::Entity> for Entity {
    fn to() -> RelationDef {
        super::relationships::Relation::Content.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::relationships::Relation::Meta.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}