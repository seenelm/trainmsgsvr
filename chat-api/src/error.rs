use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub struct ApiError {
    pub status_code: Option<StatusCode>,
    pub error: String,
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
        if let Some(status_code) = self.status_code {
            return error_response(status_code, format!("{}", self.error));
        }
        error_response(StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self.error))
    }
}
