use async_trait::async_trait;

use crate::modules::chat::{
    dto::{
        input::{CreateMessageDto, MessageQueryParams},
        output::{MessageResponseDto, MessagesListResponseDto}
    },
    entity::NewChatMessage,
    repository::repository_trait::ChatRepository
};
use crate::common::error::AppError;

#[async_trait]
pub trait ChatService: Send + Sync {
    async fn create_message(&self, room_id: i32, dto: CreateMessageDto) -> Result<MessageResponseDto, AppError>;
    async fn get_messages_by_room_id(&self, room_id: i32, params: MessageQueryParams) -> Result<MessagesListResponseDto, AppError>;
} 