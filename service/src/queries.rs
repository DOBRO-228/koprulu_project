use sea_orm::*;

pub struct Queries;

impl Queries {
    pub async fn find_entity_by_id<E>(db: &DbConn, id: i32) -> Option<E::Model>
    where
        E: EntityTrait,
        E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    {
        E::find_by_id(id)
            .one(db)
            .await.ok()?
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
