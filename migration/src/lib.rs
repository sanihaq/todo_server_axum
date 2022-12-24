use crate::sea_orm::{ConnectionTrait, Statement};
pub use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{Database, DatabaseConnection};

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

/// set `drop_if_exist` to `true` to create database for the first time or to recreate and reset otherwise leave it to `false`
pub async fn run_migration(
    db_url: String,
    db_name: String,
    drop_if_exist: bool,
) -> Result<(), DbErr> {
    let db = Database::connect(&db_url).await?;

    let _ = if let sea_orm::DatabaseBackend::Postgres = db.get_database_backend() {
        if drop_if_exist {
            create_database(&db, &db_name, drop_if_exist).await?;
        }
    } else {
        panic!("database supposed to be Postgres!!")
    };

    let db = Database::connect(format!("{}/{}", db_url, db_name)).await?;

    match Migrator::refresh(&db).await {
        Ok(_) => println!("migration via API complete"),
        Err(err) => return Err(err),
    };

    let schema_manager = SchemaManager::new(&db); // To investigate the schema
    assert!(schema_manager.has_table("users").await?);
    assert!(schema_manager.has_table("tasks").await?);

    Ok(())
}

pub async fn create_database(
    db: &DatabaseConnection,
    db_name: &String,
    drop_if_exist: bool,
) -> Result<(), DbErr> {
    if drop_if_exist {
        drop_database(&db, db_name).await?
    }
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", db_name),
    ))
    .await?;
    Ok(())
}

pub async fn drop_database(db: &DatabaseConnection, db_name: &String) -> Result<(), DbErr> {
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
    ))
    .await?;
    Ok(())
}
