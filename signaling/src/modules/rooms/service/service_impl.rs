use serde::de;
use uuid::Uuid;
use async_trait::async_trait;

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
        Self { repository }
    }
}

#[async_trait]
impl<R> RoomService for RoomServiceImpl<R>
where
    R: RoomRepository + Send + Sync + 'static,
{
    async fn create_room(&self, dto: CreateRoomDto) -> Result<RoomResponseDto, AppError> {
        let mut new_room = NewRoom::from(dto);
        new_room.public_code = Uuid::new_v4().to_string();
        let room = self.repository.insert_room(new_room).await?;
        Ok(room.into())
    }
    async fn get_room_by_id(&self, id: i32) -> Result<RoomResponseDto, AppError> {
        let room = self.repository.select_room_by_id(id).await?;
        let room = room.ok_or(DomainError::RoomNotFound(format!("Lesson with id={} not found", id)))?;
        Ok(room.into())
    }
    async fn get_room_by_public_code(&self, public_code: &str) -> Result<RoomResponseDto, AppError> {
        let room = self.repository.select_room_by_public_code(public_code).await?;
        let room = room.ok_or(DomainError::RoomNotFound(format!("Lesson with public_code={} not found", public_code)))?;
        Ok(room.into())
    }
    async fn update_room(&self, id: i32, dto: UpdateRoomDto) -> Result<RoomResponseDto, AppError> {
        let room = self.repository.update_room_by_id(id, dto).await?;
        let room = room.ok_or(DomainError::RoomNotFound(format!("Lesson with id={} not found", id)))?;
        Ok(room.into())
    }
    async fn delete_room(&self, id: i32) -> Result<(), AppError> {
        let deleted = self.repository.delete_room_by_id(id).await?;
        if deleted == 0 {
            return Err(DomainError::RoomNotFound(format!("Lesson with id={} not found", id)))?;
        }
        Ok(())
    }
}
