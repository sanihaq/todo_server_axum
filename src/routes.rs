use axum::{
    routing::{get, post},
    Router,
};

mod health_check;
pub mod users;

use health_check::health_check;
use users::create_user::create_user;

pub fn build_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/users", post(create_user))
}
