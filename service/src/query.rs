use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_entity_by_id<E>(db: &DbConn, id: i32) -> Result<E::ActiveModel, DbErr>
    where
        E: EntityTrait,
        E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
        E::Model: Into<E::ActiveModel> + Send,
    {
        E::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom(format!("Cannot find 'Description' with id '{id}'.")))
            .map(Into::into)
    }

    pub async fn get_entity_list<E>(db: &DbConn) -> Result<Vec<E::Model>, DbErr>
    where
        E: EntityTrait,
        E::Model: Send + Sync,
    {
        let entities = E::find().all(db).await?;
        Ok(entities)
    }
}
