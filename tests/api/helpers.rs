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

pub async fn spawn_app() -> AppState {
    dotenv().ok();
    let uri: String = dotenv!("API_URI").to_owned();
    let database_uri: String = dotenv!("DB_CONNECTION").to_owned();
    let database_name: String = Uuid::new_v4().to_string();
    let state;
    if let Some(port) = get_available_port() {
        let database_url = format!("{}/{}", database_uri, database_name);
        run_migration(database_uri, database_name, true)
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
        tokio::spawn(async move { run(app_state).await.expect("Failed to run the server") });
    } else {
        panic!("problem finding a port!")
    }
    state
}
