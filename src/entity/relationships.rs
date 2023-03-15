use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "typecho_relationships"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub cid: u32,
    pub mid: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Cid,
    Mid,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Cid,
    Mid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (u32, u32);

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Content,
    Meta,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Cid => ColumnType::Integer.def(),
            Self::Mid => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Content => Entity::belongs_to(super::contents::Entity)
                .from(Column::Cid)
                .to(super::contents::Column::Cid)
                .into(),
            Self::Meta => Entity::belongs_to(super::metas::Entity)
                .from(Column::Mid)
                .to(super::metas::Column::Mid)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}