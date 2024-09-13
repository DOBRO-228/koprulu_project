use super::queries::Queries;
use ::entity::{
    description,
    description::Entity as Description,
    hormone,
    hormone::Entity as Hormone,
};
use sea_orm::*;
use common::traits::HasEntityName;

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

    // Generic method to create an entity
    pub async fn create_entity<E>(db: &DbConn, form_data: E::Model) -> Result<E::ActiveModel, DbErr>
    where
        E: EntityTrait,
        E::ActiveModel: From<E::Model> + ActiveModelBehavior + Send,
        E::Model: IntoActiveModel<E::ActiveModel>,
    {
        let active_model: E::ActiveModel = form_data.into();
        let inserted_entity = active_model.save(db).await?;
        Ok(inserted_entity)
    }

    // Generic method to update an entity by ID
    pub async fn update_entity_by_id<E>(
        db: &DbConn,
        id: i32,
        form_data: E::Model,
    ) -> Result<E::Model, DbErr>
    where
        E: EntityTrait,
        E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
        E::ActiveModel: From<E::Model> + ActiveModelBehavior + Send,
        E::Model: IntoActiveModel<E::ActiveModel> + HasEntityName,
    {
        let entity = Queries::get_entity_by_id::<E>(db, id).await?;
        if let Some(model) = entity {
            let mut active_model = model.into_active_model();
            active_model = E::ActiveModel::from(form_data);
            let updated_entity = active_model.update(db).await?;
            Ok(updated_entity)
        } else {
            Err(DbErr::RecordNotFound(format!(
                "{} with id '{}' not found",
                E::Model::ENTITY_NAME,
                id
            )))
        }
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
    
    pub async fn create_hormone(
        db: &DbConn,
        form_data: hormone::Model,
    ) -> Result<hormone::ActiveModel, DbErr> {
        let updated_hormone = hormone::ActiveModel {
            name: Set(form_data.name.to_owned()),
            hormone_type: Set(form_data.hormone_type.to_owned()),
            description_id: Set(form_data.description_id.to_owned()),
            ..Default::default()
        }
            .save(db)
            .await?;
        Ok(updated_hormone)
    }

    pub async fn update_hormone_by_id(
        db: &DbConn,
        id: i32,
        form_data: hormone::Model,
    ) -> Result<hormone::Model, DbErr> {
        let description = Queries::get_entity_by_id::<Hormone>(db, id).await?;
        if let Some(description) = description {
            let active_model_description = description.into_active_model();
            let updated_description = hormone::ActiveModel {
                id: active_model_description.id,
                name: Set(form_data.name.to_owned()),
                hormone_type: Set(form_data.hormone_type.to_owned()),
                description_id: Set(form_data.description_id.to_owned()),
            }
                .update(db)
                .await?;
            Ok(updated_description)
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Hormone with id '{}' not found",
                id
            )))
        }
    }
}
