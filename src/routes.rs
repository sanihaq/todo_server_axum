use axum::{
    routing::{get, post},
    Router,
};

mod health_check;
pub mod users;

use health_check::health_check;
use users::create_user::create_user;

use crate::app_state::AppState;

pub fn build_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/users", post(create_user))
        .with_state(app_state)
}
