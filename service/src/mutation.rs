use super::query::Query;
use ::entity::{description, description::Entity as Description};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_description(
        db: &DbConn,
        form_data: description::Model,
    ) -> Result<description::ActiveModel, DbErr> {
        description::ActiveModel {
            description: Set(form_data.description.to_owned()),
            meta_description: Set(form_data.meta_description.to_owned()),
            in_excess: Set(form_data.in_excess.to_owned()),
            in_norm: Set(form_data.in_norm.to_owned()),
            in_deficiency: Set(form_data.in_deficiency.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_description_by_id(
        db: &DbConn,
        id: i32,
        form_data: description::Model,
    ) -> Result<description::Model, DbErr> {
        let description = Query::find_entity_by_id::<Description>(db, id).await?;

        description::ActiveModel {
            id: description.id,
            description: Set(form_data.description.to_owned()),
            meta_description: Set(form_data.meta_description.to_owned()),
            in_excess: Set(form_data.in_excess.to_owned()),
            in_norm: Set(form_data.in_norm.to_owned()),
            in_deficiency: Set(form_data.in_deficiency.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_entity_by_id<E>(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr>
    where
        E: EntityTrait,
        E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    {
        let entity = Query::find_entity_by_id::<Description>(db, id).await?;
        entity.delete(db).await
    }

    pub async fn delete_all_descriptions(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Description::delete_many().exec(db).await
    }
}
