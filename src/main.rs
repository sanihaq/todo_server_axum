use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::Database;
use todo_server_axum::{app_state::AppState, run, utilities::jwt::TokenWrapper};

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    let uri: String = dotenv!("API_URI").to_owned();
    let port: String = dotenv!("API_PORT").to_owned();
    let jwt_secret: String = dotenv!("JWT_SECRET").to_owned();
    let port = port.parse::<u16>().unwrap();
    let database_uri: String = dotenv!("DB_CONNECTION").to_owned();
    let database_name: String = dotenv!("DB_NAME").to_owned();
    let database_url = format!("{}/{}", database_uri, database_name);

    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    let app_state = AppState {
        port,
        uri,
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };

    run(app_state).await
}
