use async_trait::async_trait;
use crate::common::error::AppError;

use crate::modules::chat::entity::{ChatMessage, NewChatMessage};
use crate::modules::chat::dto::input::MessageQueryParams;

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn insert_message(&self, message: NewChatMessage) -> Result<ChatMessage, AppError>;
    async fn select_messages_by_room_id(&self, room_id: i32, params: MessageQueryParams) -> Result<Vec<ChatMessage>, AppError>;
    async fn count_messages_by_room_id(&self, room_id: i32) -> Result<i64, AppError>;
} 