use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "hormone_triggers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub hormone_id: i32,
    pub trigger_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::HormoneEntity",
        from = "Column::HormoneId",
        to = "super::hormone::Column::Id"
    )]
    Hormone,
    #[sea_orm(
        belongs_to = "super::TriggerEntity",
        from = "Column::TriggerId",
        to = "super::trigger::Column::Id"
    )]
    Trigger,
}

impl Related<super::HormoneEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Hormone.def()
    }
}

impl Related<super::TriggerEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Trigger.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
