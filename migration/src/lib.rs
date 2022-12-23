pub use sea_orm_migration::prelude::*;

mod m20221223_184859_create_users_table;
mod m20221223_192625_create_tasks_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221223_184859_create_users_table::Migration),
            Box::new(m20221223_192625_create_tasks_table::Migration),
        ]
    }
}
