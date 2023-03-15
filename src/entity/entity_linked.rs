use sea_orm::entity::prelude::*;

#[derive(Debug)]
pub struct ContentToMeta ;

impl Linked for ContentToMeta {
    type FromEntity = super::contents::Entity;

    type ToEntity = super::metas::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::relationships::Relation::Content.def().rev(),
            super::relationships::Relation::Meta.def(),
        ]
    }
}
