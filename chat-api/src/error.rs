use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use database::DataError;
use serde::{Deserialize, Serialize};

pub type ApiResult<T> = std::result::Result<T, ApiError>;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] DataError),
    #[error("Resource not found")]
    ResourceNotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
}

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[error("{message}")]
pub struct RequestError {
    pub message: String,
}

pub fn error_response<T: Into<String>>(status_code: StatusCode, message: T) -> Response {
    (
        status_code,
        Json(RequestError {
            message: message.into(),
        }),
    )
        .into_response()
}

// Convert ApiError to Response to be sent to Client
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            ApiError::Database(db_err) => match db_err {
                DataError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
                DataError::QueryError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
                DataError::InsertError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
                DataError::NotFoundError(err) => (StatusCode::NOT_FOUND, err),
            },
            ApiError::ResourceNotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            ApiError::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
        };
        error_response(status_code, message)
    }
}
