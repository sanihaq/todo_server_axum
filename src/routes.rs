use axum::{
    middleware,
    routing::{get, patch, post, put},
    Router,
};

mod health_check;
pub mod tasks;
pub mod users;

use self::{
    tasks::{
        create_task::create_task,
        get_all_task::get_all_task,
        get_task::get_task,
        update_task::{mark_done, mark_undone, update_task},
    },
    users::{create_user::create_user, login::login, logout::logout},
};
use crate::{app_state::AppState, middleware::guard::require_authentication};
use health_check::health_check;

pub fn build_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks", get(get_all_task))
        .route("/api/v1/tasks/:task_id", get(get_task))
        .route("/api/v1/tasks/:task_id", patch(update_task))
        .route("/api/v1/tasks/:task_id/done", put(mark_done))
        .route("/api/v1/tasks/:task_id/undone", put(mark_undone))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/health", get(health_check))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}
