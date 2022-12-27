use axum::{
    middleware,
    routing::{get, post},
    Router,
};

mod health_check;
pub mod users;

use self::users::{create_user::create_user, login::login, logout::logout};
use crate::{app_state::AppState, middleware::guard::require_authentication};
use health_check::health_check;

pub fn build_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/health", get(health_check))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}
