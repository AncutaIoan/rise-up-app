use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("User not found")]
    NotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Failure: {0}")]
    InternalError(String),
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::DatabaseError(_) | UserError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NotFound => StatusCode::NOT_FOUND,
            UserError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
