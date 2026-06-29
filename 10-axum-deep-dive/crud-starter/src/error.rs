use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    #[allow(dead_code)]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(what)  => (StatusCode::NOT_FOUND,   what),
            AppError::BadRequest(why) => (StatusCode::BAD_REQUEST, why),
            AppError::Internal(why)   => (StatusCode::INTERNAL_SERVER_ERROR, why),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
