use sqlx::PgPool;
use tracing::error;

use crate::common::error::{AppError, InfrastructureError};
use crate::modules::participants::{
    entity::{Participant, NewParticipant},
    dto::input::UpdateParticipantDto,
    repository::repository_trait::ParticipantRepository,
};

pub struct PostgresParticipantRepository {
    pool: PgPool,
}

impl PostgresParticipantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ParticipantRepository for PostgresParticipantRepository {
    async fn insert_participant(&self, dto: NewParticipant) -> Result<Participant, AppError> {
        let result = sqlx::query_as!(
            Participant,
            r#"
            INSERT INTO participants (room_id, client_id, name, is_host)
            VALUES ($1, $2, $3, $4)
            RETURNING id, room_id, client_id, name, is_host
            "#,
            dto.room_id,
            dto.client_id,
            dto.name,
            dto.is_host
        )
        .fetch_one(&self.pool)
        .await?;
       
        Ok(result)
    }

    async fn select_participants_by_room_id(&self, room_id: i32) -> Result<Vec<Participant>, AppError> {
        let result = sqlx::query_as!(
            Participant,
            r#"
            SELECT id, room_id, client_id, name, is_host
            FROM participants
            WHERE room_id = $1
            ORDER BY id
            "#,
            room_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    async fn select_participant_by_id(&self, id: i32) -> Result<Option<Participant>, AppError> {
        let result = sqlx::query_as!(
            Participant,
            r#"
            SELECT id, room_id, client_id, name, is_host
            FROM participants
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?; 

        Ok(result)
    }

    async fn update_participant_by_id(&self, id: i32, dto: UpdateParticipantDto) -> Result<Option<Participant>, AppError> {
        let result = sqlx::query_as!(
            Participant,
            r#"
            UPDATE participants
            SET name = COALESCE($1, name), is_host = COALESCE($2, is_host)
            WHERE id = $3
            RETURNING id, room_id, client_id, name, is_host
            "#,
            dto.name,
            dto.is_host,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete_participant_by_id(&self, id: i32) -> Result<u64, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM participants
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
} 