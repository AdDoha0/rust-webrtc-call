use async_trait::async_trait;

use crate::modules::rooms::{
    dto::{
        input::{CreateRoomDto, UpdateRoomDto},
        output::RoomResponseDto
    },
    entity::NewRoom,
    repository::repository_trait::RoomRepository
};
use crate::common::error::AppError;


#[async_trait]
pub trait RoomService: Send + Sync {
    async fn create_room(&self, dto: CreateRoomDto) -> Result<RoomResponseDto, AppError>;
    async fn get_room_by_id(&self, id: i32) -> Result<RoomResponseDto, AppError>;
    async fn get_room_by_public_code(&self, public_code: &str) -> Result<RoomResponseDto, AppError>;
    async fn update_room(&self, id: i32, dto: UpdateRoomDto) -> Result<RoomResponseDto, AppError>;
    async fn delete_room(&self, id: i32) -> Result<(), AppError>;
}


