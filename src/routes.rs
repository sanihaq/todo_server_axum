use axum::{
    routing::{get, post},
    Router,
};

mod health_check;
pub mod users;

use self::users::login::login;
use crate::app_state::AppState;
use health_check::health_check;
use users::create_user::create_user;

pub fn build_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}
