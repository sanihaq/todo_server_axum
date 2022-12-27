use super::{RequestCreateUser, ResponseUser};
use crate::{
    database::users::{self, Entity as Users},
    queries::user_queries::save_active_user,
    utilities::{
        app_error::{general_server_error, AppError},
        hash::hash_password,
        jwt::{create_token, TokenWrapper},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(req_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseUser>, AppError> {
    if let Some(_) = Users::find()
        .filter(users::Column::Username.eq(req_user.username.clone()))
        .one(&db)
        .await
        .map_err(|_| general_server_error())?
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

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.username,
        token: user.token.unwrap(),
    }))
}
