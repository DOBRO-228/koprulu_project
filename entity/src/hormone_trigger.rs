use sea_orm::entity::prelude::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "hormone_triggers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub hormone_id: i32,
    pub trigger_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::hormone::Entity")]
    Hormone,
    #[sea_orm(belongs_to = "super::trigger::Entity")]
    Trigger,
}

impl Related<super::hormone::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hormone.def()
    }
}

impl Related<super::trigger::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Trigger.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
