use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{debug, error};

use crate::common::error::{AppError, InfrastructureError};
use crate::modules::chat::{
    entity::{ChatMessage, NewChatMessage},
    dto::input::MessageQueryParams,
    repository::repository_trait::ChatRepository,
};

pub struct PostgresChatRepository {
    pool: PgPool,
}

impl PostgresChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatRepository for PostgresChatRepository {
    async fn insert_message(&self, message: NewChatMessage) -> Result<ChatMessage, AppError> {
        debug!("Inserting new message for room: {}", message.room_id);
        
        let result = sqlx::query_as!(
            ChatMessage,
            r#"
            INSERT INTO chat_messages (room_id, sender_id, message)
            VALUES ($1, $2, $3)
            RETURNING id, room_id, sender_id, message, created_at
            "#,
            message.room_id,
            message.sender_id,
            message.message
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("Message inserted successfully with id: {}", result.id);
        Ok(result)
    }

    async fn select_messages_by_room_id(&self, room_id: i32, params: MessageQueryParams) -> Result<Vec<ChatMessage>, AppError> {
        debug!("Selecting messages for room: {} with limit: {}, offset: {}", room_id, params.limit, params.offset);
        
        let messages = sqlx::query_as!(
            ChatMessage,
            r#"
            SELECT id, room_id, sender_id, message, created_at
            FROM chat_messages
            WHERE room_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            room_id,
            params.limit,
            params.offset
        )
        .fetch_all(&self.pool)
        .await?;

        debug!("Retrieved {} messages for room: {}", messages.len(), room_id);
        Ok(messages)
    }

    async fn count_messages_by_room_id(&self, room_id: i32) -> Result<i64, AppError> {
        debug!("Counting messages for room: {}", room_id);
        
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM chat_messages
            WHERE room_id = $1
            "#,
            room_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to count messages: {}", e);
            InfrastructureError::DatabaseError(e).into()
        })?;

        debug!("Total messages count for room {}: {}", room_id, count);
        Ok(count)
    }
} 