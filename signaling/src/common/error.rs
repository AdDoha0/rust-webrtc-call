use thiserror::Error;
use axum::{
    http::StatusCode, response::{IntoResponse, Response}, Json
};
use sqlx::Error as SqlxError;
use serde::Serialize;
use tracing::error; 

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Room not found: {0}")]
    RoomNotFound(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Authentication failed")]
    AuthenticationError,

    #[error("Unauthorized action")]
    Unauthorized,

    #[error("Invalid message format")]
    InvalidMessageFormat,
}

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(SqlxError),

    #[error("WebSocket connection error: {0}")]
    WsConnectionError(String),

    #[error("Internal server error")]
    InternalError,
}

impl From<SqlxError> for InfrastructureError {
    fn from(e: SqlxError) -> Self {
        error!("Database error: {}", e);
        InfrastructureError::DatabaseError(e)
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),
}

#[derive(Serialize)]
struct ErrorResponse {
    pub error: String,
    pub r#type: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Domain(domain) => {
                let (status, r#type) = match domain {
                    DomainError::UserNotFound(_) => (StatusCode::NOT_FOUND, "UserNotFound"),
                    DomainError::RoomNotFound(_) => (StatusCode::NOT_FOUND, "RoomNotFound"),
                    DomainError::NotFound(_) => (StatusCode::NOT_FOUND, "NotFound"),
                    DomainError::AuthenticationError => (StatusCode::UNAUTHORIZED, "AuthenticationError"),
                    DomainError::Unauthorized => (StatusCode::FORBIDDEN, "Unauthorized"),
                    DomainError::InvalidMessageFormat => (StatusCode::BAD_REQUEST, "InvalidMessageFormat"),
                };
                let body = Json(ErrorResponse { error: domain.to_string(), r#type: r#type.to_string() });
                (status, body).into_response()
            }

            AppError::Infrastructure(infra) => {
                let (status, r#type, msg) = match infra {
                    InfrastructureError::DatabaseError(err) => match err {
                        SqlxError::RowNotFound => (StatusCode::NOT_FOUND, "RowNotFound", "Row not found".to_string()),
                        SqlxError::Database(db_err) => match db_err.code().as_deref() {
                            Some("23505") => (StatusCode::CONFLICT, "UniqueViolation", db_err.to_string()),
                            Some("23503") => (StatusCode::CONFLICT, "ForeignKeyViolation", db_err.to_string()),
                            Some("23502") => (StatusCode::BAD_REQUEST, "NotNullViolation", db_err.to_string()),
                            Some("22P02") => (StatusCode::BAD_REQUEST, "InvalidTextRepresentation", db_err.to_string()),
                            _ => (StatusCode::INTERNAL_SERVER_ERROR, "DatabaseError", db_err.to_string()),
                        },
                        _ => (StatusCode::INTERNAL_SERVER_ERROR, "DatabaseError", err.to_string()),
                    },
                    InfrastructureError::WsConnectionError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "WsConnectionError", msg),
                    InfrastructureError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "InternalError", infra.to_string()),
                };
                let body = Json(ErrorResponse { error: msg, r#type: r#type.to_string() });
                (status, body).into_response()
            }
        }
    }
}