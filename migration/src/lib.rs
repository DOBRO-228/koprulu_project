pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_init_tables;
mod m20240913_153100_create_food_activity_supplement_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_init_tables::Migration),
            Box::new(m20240913_153100_create_food_activity_supplement_tables::Migration),
        ]
    }
}
