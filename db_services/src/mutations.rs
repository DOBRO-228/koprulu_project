use super::queries::Queries;
use common::traits::HasEntityName;
use sea_orm::*;
use log::warn;

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
            let active_model = E::ActiveModel::from(entity_for_delete);
            let delete_result = active_model.delete(db).await?;
            Ok(delete_result)
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

    pub async fn create_entity<E>(db: &DbConn, form_data: E::ActiveModel) -> Result<E::ActiveModel, DbErr>
    where
        E: EntityTrait,
        E::ActiveModel: From<E::Model> + ActiveModelBehavior + Send,
        E::Model: IntoActiveModel<E::ActiveModel>,
    {
        warn!("Creating entity: {:?}", form_data);
        form_data.save(db).await
    }

    pub async fn update_entity<E>(
        db: &DbConn,
        form_data: E::Model,
    ) -> Result<E::Model, DbErr>
    where
        E: EntityTrait,
        E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
        E::ActiveModel: From<E::Model> + ActiveModelBehavior + Send,
        E::Model: IntoActiveModel<E::ActiveModel> + HasEntityName,
    {
        let active_model = E::ActiveModel::from(form_data);
        active_model.update(db).await
    }
}
