use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                error_message: self.message,
            }),
        )
            .into_response()
    }
}

pub fn general_server_error() -> AppError {
    AppError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong, please try again.",
    )
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error_message: String,
}
