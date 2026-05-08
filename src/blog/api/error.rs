use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::error::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    JsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error("Invalid login or password")]
    Unauthorized,
    #[error("{0}")]
    NotFound(String),
    #[error("Resource already exists")]
    Conflict,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    InternalServerError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            Self::JsonRejection(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, Self::Unauthorized.to_string()),
            Self::NotFound(err) => (StatusCode::NOT_FOUND, err),
            Self::Conflict => (StatusCode::CONFLICT, Self::Conflict.to_string()),
            Self::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
            Self::InternalServerError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };

        (status, message).into_response()
    }
}
