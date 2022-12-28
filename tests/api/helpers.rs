use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use migration::{drop_database_with_force, run_migration};
use sea_orm::{Database, Set};
use std::{borrow::Cow, net::TcpListener};
use todo_server_axum::database::users::{self, ActiveModel as User};
use todo_server_axum::queries::user_queries::save_active_user;
use todo_server_axum::routes::users::RequestCreateUser;
use todo_server_axum::utilities::hash::hash_password;
use todo_server_axum::utilities::jwt::create_token;
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

pub async fn setup_user(state: &AppState, db_info: &DbInfo) -> (RequestCreateUser, User, String) {
    let request_user = RequestCreateUser {
        username: TEST_USER.username.into_owned(),
        password: TEST_USER.password.into_owned(),
    };

    let mut user = users::ActiveModel {
        ..Default::default()
    };

    user.username = Set(request_user.username.clone());
    user.password = Set(hash_password(&request_user.password).expect("error hashing password."));

    let token = create_token(&state.jwt_secret.0, TEST_USER.username.into_owned())
        .expect("error creating token.");

    user.token = Set(Some(token.clone()));

    let _ = save_active_user(&state.db, user.clone())
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Unable to save in database.  port: {}, db: {}",
                state.port, db_info.name
            )
        });

    (request_user, user, token)
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
