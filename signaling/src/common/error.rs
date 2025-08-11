use thiserror::Error;
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};


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
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Определяем HTTP статус в зависимости от варианта ошибки
        let (status, error_message) = match &self {
            SignalError::WsConnectionError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            SignalError::InvalidMessageFormat => (StatusCode::BAD_REQUEST, self.to_string()),
            SignalError::AuthenticationError => (StatusCode::UNAUTHORIZED, self.to_string()),
            SignalError::Unauthorized => (StatusCode::FORBIDDEN, self.to_string()),
            SignalError::UserNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            SignalError::RoomNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            SignalError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        // Создаём JSON с сообщением об ошибке
        let body = Json(ErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}