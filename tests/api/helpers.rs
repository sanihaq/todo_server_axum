use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use migration::{drop_database, run_migration};
use sea_orm::Database;
use std::net::TcpListener;
use todo_server_axum::{app_state::AppState, run};
use uuid::Uuid;

fn get_available_port() -> Option<u16> {
    (8000..9000).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Debug)]
pub struct DbInfo {
    pub url: String,
    pub name: String,
}

pub async fn spawn_app() -> (AppState, DbInfo) {
    dotenv().ok();
    let uri: String = dotenv!("API_URI").to_owned();
    let db_info = DbInfo {
        url: dotenv!("DB_CONNECTION").to_owned(),
        name: Uuid::new_v4().to_string(),
    };
    let state;
    if let Some(port) = get_available_port() {
        let database_url = format!("{}/{}", &db_info.url, db_info.name);
        run_migration(&db_info.url, &db_info.name, true)
            .await
            .unwrap();

        let db = match Database::connect(database_url).await {
            Ok(db) => db,
            Err(error) => {
                eprintln!("Error connecting to the database: {:?}", error);
                panic!();
            }
        };
        let app_state = AppState { port, uri, db };
        state = app_state.clone();
        let _ =
            tokio::spawn(async move { run(app_state).await.expect("Failed to run the server") });
    } else {
        panic!("problem finding a port!")
    }
    (state, db_info)
}

pub async fn drop_database_after_test(db: sea_orm::DatabaseConnection, db_info: DbInfo) {
    let _ = db.close().await.map_err(|e| e);
    let db = Database::connect(&db_info.url).await.unwrap();
    drop_database(&db, &db_info.name).await.unwrap();
    let _ = db.close().await.map_err(|e| e);
}
