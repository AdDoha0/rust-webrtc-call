use async_trait::async_trait;
use crate::common::error::AppError;


use crate::modules::rooms::entity::{Room, NewRoom };
use crate::modules::rooms::dto::input::UpdateRoomDto;

// use crate::modules::textbooks::query::TextbookQuery;

#[async_trait]
pub trait RoomRepository: Send + Sync {
    async fn insert_room(&self, dto: NewRoom) -> Result<Room, AppError>;
    async fn select_room_by_id(&self, id: i32) -> Result<Option<Room>, AppError>;
    async fn select_room_by_public_code(&self, public_code: &str) -> Result<Option<Room>, AppError>;
    async fn update_room_by_id(&self, id: i32, dto: UpdateRoomDto) -> Result<Option<Room>, AppError>;
    async fn delete_room_by_id(&self, id: i32) -> Result<u64, AppError>;
}




