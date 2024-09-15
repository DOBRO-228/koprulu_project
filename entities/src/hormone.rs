use common::traits::HasEntityName;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum HormoneType {
    #[sea_orm(string_value = "Sexual")]
    Sexual,
    #[sea_orm(string_value = "Neurotransmitter")]
    Neurotransmitter,
    #[sea_orm(string_value = "Corticosteroid")]
    Corticosteroid,
    #[sea_orm(string_value = "Catecholamine")]
    Catecholamine,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "hormones")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub hormone_type: HormoneType,
    pub description: String,
    pub meta_description: Option<String>,
    pub in_excess: Option<String>,
    pub in_norm: Option<String>,
    pub in_deficiency: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::HormoneTriggerEntity")]
    HormoneTrigger,
}

impl Related<super::TriggerEntity> for Entity {
    fn to() -> RelationDef {
        super::hormone_trigger::Relation::Trigger.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::hormone_trigger::Relation::Hormone.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl HasEntityName for Model {
    const ENTITY_NAME: &'static str = "Hormone";
}
