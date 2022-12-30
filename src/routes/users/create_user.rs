use super::{RequestCreateUser, ResponseUser};
use crate::{
    database::{
        tasks::{self, Entity as Tasks},
        users::{self, Entity as Users},
    },
    queries::{task_queries::save_active_task, user_queries::save_active_user},
    utilities::{
        app_error::{general_server_error, AppError},
        hash::hash_password,
        jwt::{create_token, TokenWrapper},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(req_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseUser>, AppError> {
    if (Users::find()
        .filter(users::Column::Username.eq(req_user.username.clone()))
        .one(&db)
        .await
        .map_err(|_| general_server_error())?)
    .is_some()
    {
        return Err(AppError::new(StatusCode::CONFLICT, "User already exists."));
    }
    let mut user = users::ActiveModel {
        ..Default::default()
    };
    user.username = Set(req_user.username.clone());
    user.password = Set(hash_password(&req_user.password)?);
    user.token = Set(Some(create_token(&jwt_secret.0, req_user.username)?));
    let user = save_active_user(&db, user).await?;

    let mut task = tasks::ActiveModel {
        ..Default::default()
    };
    task.is_default = Set(Some(true));
    task.title = Set("This is a task".to_owned());
    save_active_task(&db, task).await?;
    let mut task = tasks::ActiveModel {
        ..Default::default()
    };
    task.title = Set("This is a another task".to_owned());
    save_active_task(&db, task).await?;
    let mut task = tasks::ActiveModel {
        ..Default::default()
    };
    let now = Utc::now();
    task.deleted_at = Set(Some(now.into()));
    task.title = Set("This is a deleted task".to_owned());
    save_active_task(&db, task).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.username,
        token: user.token.unwrap(),
    }))
}
