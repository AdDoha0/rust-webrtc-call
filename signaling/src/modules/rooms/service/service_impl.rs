use serde::de;
use uuid::Uuid;
use async_trait::async_trait;
use tracing::{info, warn, error, instrument, debug};

use crate::modules::rooms::{
    dto::{input::{CreateRoomDto, UpdateRoomDto}, output::RoomResponseDto}, entity::{NewRoom, Room}, repository::repository_trait::RoomRepository, service::service_trait::RoomService
};
use crate::common::error::{DomainError, AppError};
// use crate::modules::rooms::repository::repository_trait::RoomRepository; 

#[derive(Clone)]
pub struct RoomServiceImpl<R> 
    where R: RoomRepository
{
    repository: R,
}

impl<R> RoomServiceImpl<R>
where
    R: RoomRepository,
{
    pub fn new(repository: R) -> Self {
        debug!("Initializing RoomServiceImpl");
        Self { repository }
    }
}

#[async_trait]
impl<R> RoomService for RoomServiceImpl<R>
where
    R: RoomRepository + Send + Sync + 'static,
{
    #[instrument(skip(self), fields(room_name = dto.name.as_str()))]
    async fn create_room(&self, dto: CreateRoomDto) -> Result<RoomResponseDto, AppError> {
        info!("Creating new room: {}", dto.name);
        
        let mut new_room = NewRoom::from(dto);
        new_room.public_code = Uuid::new_v4().to_string();
        
        debug!("Generated public code: {}", new_room.public_code);
        
        let room = self.repository.insert_room(new_room).await?;
        
        info!("Room created successfully with id: {} and public_code: {}", room.id, room.public_code);
        Ok(room.into())
    }
    
    #[instrument(skip(self), fields(room_id = id))]
    async fn get_room_by_id(&self, id: i32) -> Result<RoomResponseDto, AppError> {
        debug!("Fetching room by id: {}", id);
        
        let room = self.repository.select_room_by_id(id).await?;
        let room = room.ok_or_else(|| {
            warn!("Room not found with id: {}", id);
            DomainError::RoomNotFound(format!("Room with id={} not found", id))
        })?;
        
        info!("Room retrieved successfully: {} (id: {})", room.name, room.id);
        Ok(room.into())
    }
    
    #[instrument(skip(self), fields(public_code = public_code))]
    async fn get_room_by_public_code(&self, public_code: &str) -> Result<RoomResponseDto, AppError> {
        debug!("Fetching room by public code: {}", public_code);
        
        let room = self.repository.select_room_by_public_code(public_code).await?;
        let room = room.ok_or_else(|| {
            warn!("Room not found with public_code: {}", public_code);
            DomainError::RoomNotFound(format!("Room with public_code={} not found", public_code))
        })?;
        
        info!("Room retrieved successfully by public code: {} (id: {})", room.name, room.id);
        Ok(room.into())
    }
    
    #[instrument(skip(self), fields(room_id = id))]
    async fn update_room(&self, id: i32, dto: UpdateRoomDto) -> Result<RoomResponseDto, AppError> {
        info!("Updating room with id: {}", id);
        
        if let Some(name) = &dto.name {
            debug!("Updating room name to: {}", name);
        }
        if let Some(is_active) = dto.is_active {
            debug!("Updating room active status to: {}", is_active);
        }
        
        let room = self.repository.update_room_by_id(id, dto).await?;
        let room = room.ok_or_else(|| {
            warn!("Room not found for update with id: {}", id);
            DomainError::RoomNotFound(format!("Room with id={} not found", id))
        })?;
        
        info!("Room updated successfully: {} (id: {})", room.name, room.id);
        Ok(room.into())
    }
    
    #[instrument(skip(self), fields(room_id = id))]
    async fn delete_room(&self, id: i32) -> Result<(), AppError> {
        info!("Deleting room with id: {}", id);
        
        let deleted = self.repository.delete_room_by_id(id).await?;
        
        if deleted == 0 {
            warn!("No room found to delete with id: {}", id);
            return Err(DomainError::RoomNotFound(format!("Room with id={} not found", id)))?;
        }
        
        info!("Room deleted successfully (id: {}, rows affected: {})", id, deleted);
        Ok(())
    }
}
