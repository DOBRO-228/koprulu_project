pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_init_tables;
mod m20240913_153100_create_food_activity_supplement_tables;
mod m20240915_105318_remove_description_as_separated_entity;
mod m20240918_142513_add_trigger_name_field;
mod m20240918_150246_add_sport_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_init_tables::Migration),
            Box::new(m20240913_153100_create_food_activity_supplement_tables::Migration),
            Box::new(m20240915_105318_remove_description_as_separated_entity::Migration),
            Box::new(m20240918_142513_add_trigger_name_field::Migration),
            Box::new(m20240918_150246_add_sport_table::Migration),
        ]
    }
}
