use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use migration::run_migration;
use sea_orm_migration::prelude::*;
use std::env;

#[async_std::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    let db_url: String = dotenv!("DB_CONNECTION").to_owned();
    let db_name: String = dotenv!("DB_NAME").to_owned();
    let delete_if_exist = if let Some(val) = env::args().nth(1) {
        val == "-d" || val == "--drop-if-exist"
    } else {
        false
    };

    run_migration(&db_url, &db_name, delete_if_exist).await?;

    Ok(())
}
