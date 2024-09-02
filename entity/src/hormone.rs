use sea_orm::entity::prelude::*;

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "hormones")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub hormone_type: HormoneType,
    pub description_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::HormoneTriggerEntity")]
    HormoneTrigger,
    #[sea_orm(
        belongs_to = "super::DescriptionEntity",
        from = "Column::DescriptionId",
        to = "super::description::Column::Id"
    )]
    Description,
}

impl Related<super::TriggerEntity> for Entity {
    fn to() -> RelationDef {
        super::hormone_trigger::Relation::Trigger.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::hormone_trigger::Relation::Hormone.def().rev())
    }
}

impl Related<super::description::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Description.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub const ENTITY_NAME: &'static str = "Hormone";
}