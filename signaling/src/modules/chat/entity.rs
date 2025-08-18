use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;
use tracing::debug;

use super::dto::{
    input::CreateMessageDto,
    output::MessageResponseDto, 
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: i32,
    pub room_id: i32,
    pub sender_id: i32,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewChatMessage {
    pub room_id: i32,
    pub sender_id: i32,
    pub message: String,
}

impl CreateMessageDto {
    pub fn into_new_message(self, room_id: i32) -> NewChatMessage {
        debug!("Converting CreateMessageDto to NewChatMessage for room: {}", room_id);
        NewChatMessage {
            room_id,
            sender_id: self.sender_id,
            message: self.message,
        }
    }
}

impl From<ChatMessage> for MessageResponseDto {
    fn from(message: ChatMessage) -> Self {
        debug!("Converting ChatMessage to MessageResponseDto: {} (id: {})", message.message, message.id);
        Self {
            id: message.id,
            room_id: message.room_id,
            sender_id: message.sender_id,
            message: message.message,
            created_at: message.created_at
        }
    }
} 