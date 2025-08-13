use thiserror::Error;
use axum::{
    http::StatusCode, response::{IntoResponse, Response}, Json
};

use serde::Serialize; 




#[derive(Error, Debug)]
pub enum AppError {
    #[error("WebSocket connection error: {0}")]
    WsConnectionError(String),

    #[error("Invalid message format")]
    InvalidMessageFormat,

    #[error("Authentication failed")]
    AuthenticationError,

    #[error("Unauthorized action")]
    Unauthorized,

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Room not found: {0}")]
    RoomNotFound(String),

    #[error("Internal server error")]
    InternalError,
}


#[derive(Serialize)]
struct ErrorResponse {
    pub error: String,
    pub r#type: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_type) = match &self {
            AppError::WsConnectionError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), "WsConnectionError"),
            AppError::InvalidMessageFormat => (StatusCode::BAD_REQUEST, self.to_string(), "InvalidMessageFormat"),
            AppError::AuthenticationError => (StatusCode::UNAUTHORIZED, self.to_string(), "AuthenticationError"),
            AppError::Unauthorized => (StatusCode::FORBIDDEN, self.to_string(), "Unauthorized"),
            AppError::UserNotFound(_) => (StatusCode::NOT_FOUND, self.to_string(), "UserNotFound"),
            AppError::RoomNotFound(_) => (StatusCode::NOT_FOUND, self.to_string(), "RoomNotFound"),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), "InternalError"),
        };

        let body = Json(ErrorResponse {
            error: error_message,
            r#type: error_type.to_string(),
        });

        (status, body).into_response()
    }
}
