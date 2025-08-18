use async_trait::async_trait;
use tracing::{debug, error};

use crate::common::error::{AppError, DomainError};
use crate::modules::chat::{
    dto::{
        input::{CreateMessageDto, MessageQueryParams},
        output::{MessageResponseDto, MessagesListResponseDto}
    },
    entity::NewChatMessage,
    repository::repository_trait::ChatRepository,
    service::service_trait::ChatService,
};

pub struct ChatServiceImpl<R>
where
    R: ChatRepository,
{
    repository: R,
}

impl<R> ChatServiceImpl<R>
where
    R: ChatRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> ChatService for ChatServiceImpl<R>
where
    R: ChatRepository,
{
    async fn create_message(&self, room_id: i32, dto: CreateMessageDto) -> Result<MessageResponseDto, AppError> {
        debug!("Creating new message for room: {}", room_id);
        
        // Валидация входных данных
        if dto.message.trim().is_empty() {
            return Err(DomainError::InvalidMessageFormat.into());
        }
        
        let new_message = dto.into_new_message(room_id);
        let message = self.repository.insert_message(new_message).await?;
        
        debug!("Message created successfully with id: {}", message.id);
        Ok(message.into())
    }

    async fn get_messages_by_room_id(&self, room_id: i32, params: MessageQueryParams) -> Result<MessagesListResponseDto, AppError> {
        debug!("Getting messages for room: {} with pagination", room_id);
        
        // Валидация параметров пагинации
        if params.limit <= 0 || params.limit > 100 {
            return Err(DomainError::InvalidMessageFormat.into());
        }
        
        if params.offset < 0 {
            return Err(DomainError::InvalidMessageFormat.into());
        }
        
        let messages = self.repository.select_messages_by_room_id(room_id, params.clone()).await?;
        let total = self.repository.count_messages_by_room_id(room_id).await?;
        
        let response_dto = MessagesListResponseDto {
            messages: messages.into_iter().map(|m| m.into()).collect(),
            total,
            limit: params.limit,
            offset: params.offset,
        };
        
        debug!("Retrieved {} messages for room: {}", response_dto.messages.len(), room_id);
        Ok(response_dto)
    }
} 