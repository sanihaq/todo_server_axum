use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use migration::{drop_database_with_force, run_migration};
use sea_orm::Database;
use std::{borrow::Cow, net::TcpListener};
use todo_server_axum::{app_state::AppState, run, utilities::jwt::TokenWrapper};
use uuid::Uuid;

pub const TEST_USER: TestUser = TestUser {
    username: Cow::Borrowed("tricky_tom"),
    password: Cow::Borrowed("tom-tick88^&"),
};

pub async fn spawn_app() -> (AppState, DbInfo) {
    dotenv().ok();
    let uri: String = dotenv!("API_URI").to_owned();
    let db_info = DbInfo {
        url: dotenv!("DB_CONNECTION").to_owned(),
        name: Uuid::new_v4().to_string(),
    };
    let jwt_secret: String = dotenv!("JWT_SECRET").to_owned();
    let port = get_available_port();
    run_migration(&db_info.url, &db_info.name, true)
        .await
        .unwrap();

    let database_url = format!("{}/{}", &db_info.url, db_info.name);

    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            panic!("Error connecting to the database: {:?}", error);
        }
    };
    let app_state = AppState {
        port,
        uri,
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };
    let state = app_state.clone();
    let _ = tokio::spawn(async move {
        match run(app_state).await {
            Ok(app) => app,
            Err(_) => {
                panic!("Failed to run the server");
            }
        }
    });

    (state, db_info)
}

pub async fn drop_database_after_test(db: sea_orm::DatabaseConnection, db_info: DbInfo) {
    let _ = db.close().await;
    let db = Database::connect(&db_info.url).await.unwrap();
    drop_database_with_force(&db, &db_info.name).await.unwrap();
    let _ = db.close().await;
}

#[derive(Debug)]
pub struct TestUser {
    pub username: Cow<'static, str>,
    pub password: Cow<'static, str>,
}

#[derive(Debug)]
pub struct DbInfo {
    pub url: String,
    pub name: String,
}

fn get_available_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    listener.local_addr().unwrap().port()
}
