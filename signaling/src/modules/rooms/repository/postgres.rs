use sqlx::{QueryBuilder, Postgres, PgPool};

use async_trait::async_trait;

use crate::common::error::AppError;
use crate::modules::rooms::entity::{Room, NewRoom};
use crate::modules::rooms::dto::input::UpdateRoomDto;

use super::repository_trait::RoomRepository; 

#[derive(Clone)]
pub struct PostgresRoomRepository {
    pool: PgPool,
}

impl PostgresRoomRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl RoomRepository for PostgresRoomRepository {
    async fn insert_room(&self, dto: NewRoom) -> Result<Room, AppError> {
        let result = sqlx::query_as!(
            Room,
            r#"
            INSERT INTO rooms (name, public_code, is_active)
            VALUES ($1, $2, $3)
            RETURNING id, name, public_code, is_active, created_at, updated_at
            "#,
            dto.name,
            dto.public_code,
            dto.is_active
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn select_room_by_id(&self, id: i32) -> Result<Option<Room>, AppError> {
        let result = sqlx::query_as!(
            Room,
            r#"
            SELECT id, name, public_code, is_active, created_at, updated_at
            FROM rooms
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn select_room_by_public_code(&self, public_code: &str) -> Result<Option<Room>, AppError> {
        let result = sqlx::query_as!(
            Room,
            r#"
            SELECT id, name, public_code, is_active, created_at, updated_at
            FROM rooms
            WHERE public_code = $1
            "#,
            public_code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update_room_by_id(&self, id: i32, dto: UpdateRoomDto) -> Result<Option<Room>, AppError> {

        let result = sqlx::query_as!(
            Room,
            r#"
            UPDATE rooms SET
                name = COALESCE($1, name),
                is_active = COALESCE($2, is_active)
            WHERE id = $3
            RETURNING id, name, public_code, is_active, created_at, updated_at
            "#,
            dto.name,
            dto.is_active,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
    
        Ok(result)
    }

    async fn delete_room_by_id(&self, id: i32) -> Result<u64, AppError> {
        let result = sqlx::query!(
            "DELETE FROM rooms WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}


