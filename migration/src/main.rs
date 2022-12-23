use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use migration::sea_orm::{ConnectionTrait, Database, Statement};
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    let db_url: String = dotenv!("DB_CONNECTION").to_owned();
    let db_name: String = dotenv!("DB_NAME").to_owned();
    let db = Database::connect(&db_url).await?;

    let _ = if let sea_orm::DatabaseBackend::Postgres = db.get_database_backend() {
        let url = format!("{}/{}", db_url, db_name);
        match Database::connect(&url).await {
            Ok(_) => println!("\"{}\" database found!", db_name),
            Err(_) => {
                println!("\"{}\" database not found!!", db_name);
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", db_name),
                ))
                .await?;
                println!("\"{}\" database is created.", db_name);
            }
        }
    } else {
        panic!("databese supposed to be Postgres!!")
    };

    cli::run_cli(migration::Migrator).await;

    Ok(())
}
