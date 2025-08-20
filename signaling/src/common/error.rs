use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    extract::rejection::{PathRejection, JsonRejection},
};
use sqlx::Error as SqlxError;
use serde::Serialize;
use tracing::error;

// ===== Errors =====

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Invalid ID: {0}")]
    InvalidId(String),
}

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

    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
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

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Infrastructure(InfrastructureError),

    #[error(transparent)]
    PathError(PathError),
}

// ===== From conversions with logging =====

impl From<SqlxError> for InfrastructureError {
    fn from(e: SqlxError) -> Self {
        error!("Database error: {}", e);
        InfrastructureError::DatabaseError(e)
    }
}

impl From<InfrastructureError> for AppError {
    fn from(err: InfrastructureError) -> Self {
        error!("Infrastructure error: {:?}", err);
        AppError::Infrastructure(err)
    }
}

impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        error!("Database error converted to AppError: {}", e);
        AppError::Infrastructure(InfrastructureError::from(e))
    }
}

impl From<PathRejection> for AppError {
    fn from(err: PathRejection) -> Self {
        error!("Path parsing error: {}", err);
        AppError::PathError(PathError::InvalidId(err.to_string()))
    }
}

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self {
        // Получаем сообщение об ошибке в виде строки
        let msg = err.to_string();
        error!("JSON deserialization error: {}", msg);
        AppError::Domain(DomainError::InvalidJson(msg))
    }
}

// ===== Error Response =====

#[derive(Serialize)]
struct ErrorResponse {
    pub error: String,
    pub r#type: String,
}

// ===== IntoResponse =====

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Domain(domain) => {
                let (status, r#type, msg) = match domain {
                    DomainError::UserNotFound(s) => (StatusCode::NOT_FOUND, "UserNotFound", s),
                    DomainError::RoomNotFound(s) => (StatusCode::NOT_FOUND, "RoomNotFound", s),
                    DomainError::NotFound(s) => (StatusCode::NOT_FOUND, "NotFound", s),
                    DomainError::AuthenticationError => (StatusCode::UNAUTHORIZED, "AuthenticationError", domain.to_string()),
                    DomainError::Unauthorized => (StatusCode::FORBIDDEN, "Unauthorized", domain.to_string()),
                    DomainError::InvalidMessageFormat => (StatusCode::UNPROCESSABLE_ENTITY, "InvalidMessageFormat", domain.to_string()),
                    DomainError::InvalidJson(s) => (StatusCode::UNPROCESSABLE_ENTITY, "InvalidJson", s),
                };
                let body = Json(ErrorResponse { error: msg, r#type: r#type.to_string() });
                (status, body).into_response()
            }

            AppError::PathError(path_err) => {
                let (status, r#type, msg) = match path_err {
                    PathError::InvalidId(s) => (StatusCode::BAD_REQUEST, "BadRequest", s),
                };
                let body = Json(ErrorResponse { error: msg, r#type: r#type.to_string() });
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
