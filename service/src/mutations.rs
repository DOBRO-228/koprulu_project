use super::queries::Queries;
use ::entity::{description, description::Entity as Description};
use sea_orm::*;

pub struct Mutations;

impl Mutations {
    pub async fn delete_entity_by_id<E>(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr>
    where
        E: EntityTrait,
        E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
        E::ActiveModel: From<E::Model> + Send,
    {
        let entity = Queries::get_entity_by_id::<E>(db, id).await?;
        if let Some(entity_for_delete) = entity {
            let active_model: E::ActiveModel = entity_for_delete.into();
            let deleted_entity = active_model.delete(db).await?;
            Ok(deleted_entity)
        } else {
            Ok(DeleteResult { rows_affected: 0 })
        }
    }

    pub async fn delete_all_entities<E>(db: &DbConn) -> Result<DeleteResult, DbErr>
    where
        E: EntityTrait,
    {
        let affected_rows = E::delete_many().exec(db).await?;
        Ok(affected_rows)
    }

    pub async fn create_description(
        db: &DbConn,
        form_data: description::Model,
    ) -> Result<description::ActiveModel, DbErr> {
        let updated_description = description::ActiveModel {
            description: Set(form_data.description.to_owned()),
            meta_description: Set(form_data.meta_description.to_owned()),
            in_excess: Set(form_data.in_excess.to_owned()),
            in_norm: Set(form_data.in_norm.to_owned()),
            in_deficiency: Set(form_data.in_deficiency.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await?;
        Ok(updated_description)
    }

    pub async fn update_description_by_id(
        db: &DbConn,
        id: i32,
        form_data: description::Model,
    ) -> Result<description::Model, DbErr> {
        let description = Queries::get_entity_by_id::<Description>(db, id).await?;
        if let Some(description) = description {
            let active_model_description = description.into_active_model();
            let updated_description = description::ActiveModel {
                id: active_model_description.id,
                description: Set(form_data.description.to_owned()),
                meta_description: Set(form_data.meta_description.to_owned()),
                in_excess: Set(form_data.in_excess.to_owned()),
                in_norm: Set(form_data.in_norm.to_owned()),
                in_deficiency: Set(form_data.in_deficiency.to_owned()),
            }
            .update(db)
            .await?;
            Ok(updated_description)
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Description with id '{}' not found",
                id
            )))
        }
    }
}
