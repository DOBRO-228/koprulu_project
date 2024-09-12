use sea_orm::*;

pub struct Queries;

impl Queries {
    pub async fn get_entity_by_id<E>(db: &DbConn, id: i32) -> Result<Option<E::Model>, DbErr>
    where
        E: EntityTrait,
        E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    {
        let entity = E::find_by_id(id).one(db).await?;
        Ok(entity)
    }

    pub async fn get_all_entities<E>(db: &DbConn) -> Result<Vec<E::Model>, DbErr>
    where
        E: EntityTrait,
        E::Model: Send + Sync,
    {
        let entities = E::find().all(db).await?;
        Ok(entities)
    }
}
