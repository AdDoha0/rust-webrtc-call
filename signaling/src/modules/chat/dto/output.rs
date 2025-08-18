use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize)]
pub struct MessageResponseDto {
    pub id: i32,
    pub room_id: i32,
    pub sender_id: i32,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct MessagesListResponseDto {
    pub messages: Vec<MessageResponseDto>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
} 