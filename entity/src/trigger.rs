use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "triggers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::hormone_trigger::Entity")]
    HormoneTrigger,
}

impl Related<super::hormone::Entity> for Entity {
    fn to() -> RelationDef {
        super::hormone_trigger::Relation::Hormone.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::hormone_trigger::Relation::Trigger.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
