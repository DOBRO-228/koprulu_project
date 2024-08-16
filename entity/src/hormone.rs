use sea_orm::entity::prelude::*;

#[derive(Clone, Debug)]
enum HormoneType {
    Sexual,
    Neurotransmitter,
    Corticosteroid,
    Catecholamine,
}

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "hormones")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub hormone_type: HormoneType,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::hormone_trigger::Entity")]
    HormoneTrigger,
    #[sea_orm(
        belongs_to = "super::description::Entity",
        from = "Column::DescriptionId",
        to = "super::description::Column::Id"
    )]
    Description,
}

impl Related<super::trigger::Entity> for Entity {
    fn to() -> RelationDef {
        super::hormone_trigger::Relation::Trigger.def()
    }
    fn to() -> RelationDef {
        Relation::Description.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::hormone_trigger::Relation::Hormone.def().rev())
    }
}
