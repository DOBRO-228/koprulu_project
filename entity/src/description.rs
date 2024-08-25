use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "descriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub description: String,
    pub meta_description: Option<String>,
    pub in_excess: Option<String>,
    pub in_norm: Option<String>,
    pub in_deficiency: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::HormoneEntity")]
    Hormone,
}

impl Related<super::HormoneEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Hormone.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
