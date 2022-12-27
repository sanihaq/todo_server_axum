#![allow(clippy::clone_on_copy)]
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use crate::utilities::jwt::TokenWrapper;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub port: u16,
    pub uri: String,
    pub db: DatabaseConnection,
    pub jwt_secret: TokenWrapper,
}
