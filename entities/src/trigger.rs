use common::traits::HasEntityName;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "triggers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::HormoneTriggerEntity")]
    HormoneTrigger,
}

impl Related<super::HormoneEntity> for Entity {
    fn to() -> RelationDef {
        super::hormone_trigger::Relation::Hormone.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::hormone_trigger::Relation::Trigger.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl HasEntityName for Model {
    const ENTITY_NAME: &'static str = "Trigger";
}
