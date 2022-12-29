use super::ResponseTask;
use crate::{
    database::users::Model as UserModel, queries::task_queries, utilities::app_error::AppError,
};
use axum::{extract::State, Extension, Json};
use sea_orm::DatabaseConnection;

pub async fn get_all_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, AppError> {
    let tasks = task_queries::get_all_tasks(&db, user.id, false)
        .await?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
            completed_at: db_task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
        })
        .collect::<Vec<ResponseTask>>();
    Ok(Json(tasks))
}
