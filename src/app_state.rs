use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub port: u16,
    pub uri: String,
    pub db: DatabaseConnection,
}
