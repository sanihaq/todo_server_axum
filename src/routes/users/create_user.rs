use super::{RequestCreateUser, ResponseUser};
use crate::{database::users, utilities::app_error::AppError};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(req_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseUser>, AppError> {
    let mut user = users::ActiveModel {
        ..Default::default()
    };
    user.username = Set(req_user.username);
    user.password = Set(req_user.password);
    let user = user
        .save(&db)
        .await
        .map_err(|error| {
            eprintln!("Error creating user: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .try_into_model()
        .map_err(|error| {
            eprintln!("Erro converting user back into model: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating user")
        })?;
    Ok(Json(ResponseUser {
        id: user.id,
        username: user.username,
    }))
}
