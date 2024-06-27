use database::DataError;
use socketioxide::SendError;

pub type ApiResult<T> = std::result::Result<T, ChatError>;

#[derive(thiserror::Error, Debug)]
pub enum ChatError {
    #[error("Database error: {0}")]
    Database(#[from] DataError),
    #[error("Error sending message: {0}")]
    SocketEmitError(#[from] SendError),
    #[error("Error parsing request: {0}")]
    BadRequest(String),
    #[error("Conflict error: {0}")]
    ConflictError(String),
    #[error("Not found error: {0}")]
    NotFoundError(String),
}
