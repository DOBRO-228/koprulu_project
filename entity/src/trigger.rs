use sea_orm::entity::prelude::*;
use common::traits::HasEntityName;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "triggers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
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