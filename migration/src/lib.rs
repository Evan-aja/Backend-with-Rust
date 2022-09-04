pub use sea_orm_migration::prelude::*;

mod m20220903_213818_create_cars_table;
mod m20220903_213851_create_bikes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220903_213818_create_cars_table::Migration),
            Box::new(m20220903_213851_create_bikes_table::Migration),
        ]
    }
}
