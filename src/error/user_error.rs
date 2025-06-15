use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("User not found")]
    NotFound,

    #[error("Failure")]
    InternalError(String),

}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::DatabaseError(_) => { HttpResponse::InternalServerError().body("A database error occurred") }
            UserError::InternalError(_) => { HttpResponse::InternalServerError().body("An error occurred") }
            UserError::NotFound => HttpResponse::NotFound().body("User not found")
        }
    }
}
