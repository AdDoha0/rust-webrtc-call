use sqlx::{QueryBuilder, Postgres, PgPool};
use async_trait::async_trait;
use tracing::{info, warn, error, instrument, debug, trace};

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
        debug!("Initializing PostgresRoomRepository");
        Self { pool }
    }
}


#[async_trait]
impl RoomRepository for PostgresRoomRepository {
    #[instrument(skip(self), fields(room_name = dto.name.as_str(), public_code = dto.public_code.as_str()))]
    async fn insert_room(&self, dto: NewRoom) -> Result<Room, AppError> {
        info!("Inserting new room into database: {}", dto.name);
        
        let start = std::time::Instant::now();
        
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

        let duration = start.elapsed();
        info!("Room inserted successfully in {:?} (id: {}, public_code: {})", 
              duration, result.id, result.public_code);
        
        Ok(result)
    }

    #[instrument(skip(self), fields(room_id = id))]
    async fn select_room_by_id(&self, id: i32) -> Result<Option<Room>, AppError> {
        debug!("Querying room by id: {}", id);
        
        let start = std::time::Instant::now();
        
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

        let duration = start.elapsed();
        
        match &result {
            Some(room) => {
                info!("Room found by id in {:?}: {} (id: {})", duration, room.name, room.id);
            }
            None => {
                debug!("No room found by id: {} (query took {:?})", id, duration);
            }
        }

        Ok(result)
    }

    #[instrument(skip(self), fields(public_code = public_code))]
    async fn select_room_by_public_code(&self, public_code: &str) -> Result<Option<Room>, AppError> {
        debug!("Querying room by public code: {}", public_code);
        
        let start = std::time::Instant::now();
        
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

        let duration = start.elapsed();
        
        match &result {
            Some(room) => {
                info!("Room found by public code in {:?}: {} (id: {})", duration, room.name, room.id);
            }
            None => {
                debug!("No room found by public code: {} (query took {:?})", public_code, duration);
            }
        }

        Ok(result)
    }

    #[instrument(skip(self), fields(room_id = id))]
    async fn update_room_by_id(&self, id: i32, dto: UpdateRoomDto) -> Result<Option<Room>, AppError> {
        info!("Updating room by id: {}", id);
        
        if let Some(name) = &dto.name {
            debug!("Updating room name to: {}", name);
        }
        if let Some(is_active) = dto.is_active {
            debug!("Updating room active status to: {}", is_active);
        }

        let start = std::time::Instant::now();

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
    
        let duration = start.elapsed();
        
        match &result {
            Some(room) => {
                info!("Room updated successfully in {:?}: {} (id: {})", duration, room.name, room.id);
            }
            None => {
                warn!("No room found to update with id: {} (query took {:?})", id, duration);
            }
        }

        Ok(result)
    }

    #[instrument(skip(self), fields(room_id = id))]
    async fn delete_room_by_id(&self, id: i32) -> Result<u64, AppError> {
        info!("Deleting room by id: {}", id);
        
        let start = std::time::Instant::now();
        
        let result = sqlx::query!(
            "DELETE FROM rooms WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        let duration = start.elapsed();
        let rows_affected = result.rows_affected();
        
        if rows_affected > 0 {
            info!("Room deleted successfully in {:?} (id: {}, rows affected: {})", 
                  duration, id, rows_affected);
        } else {
            warn!("No room found to delete with id: {} (query took {:?})", id, duration);
        }

        Ok(rows_affected)
    }
}


