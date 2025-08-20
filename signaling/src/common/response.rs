use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing)]
    pub http_status: StatusCode,
    pub status: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    // 200 OK (обычные GET/PATCH)
    pub fn success(data: T) -> Self {
        Self {
            http_status: StatusCode::OK,
            status: "success".to_string(),
            data: Some(data),
        }
    }

    // 201 Created (POST)
    pub fn created(data: T) -> Self {
        Self {
            http_status: StatusCode::CREATED,
            status: "success".to_string(),
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    // 204 No Content (DELETE)
    pub fn no_content() -> Self {
        Self {
            http_status: StatusCode::NO_CONTENT,
            status: "success".to_string(),
            data: None,
        }
    }
}

impl ApiResponse<serde_json::Value> {
    // 200 OK, только сообщение
    pub fn message(msg: &str) -> Self {
        Self {
            http_status: StatusCode::OK,
            status: "success".to_string(),
            data: Some(serde_json::json!({ "message": msg })),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        // Если DELETE (204) → возвращаем пустой ответ без JSON
        if self.http_status == StatusCode::NO_CONTENT {
            return (self.http_status, Json(serde_json::json!({}))).into_response();
        }

        // Для остальных статусов возвращаем JSON
        let status = self.http_status;
        let body = Json(self);
        (status, body).into_response()
    }
}
