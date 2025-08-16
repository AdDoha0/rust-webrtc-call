use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;
use tracing::debug;

use super::dto::{
    input::CreateRoomDto,
    output::RoomResponseDto, 
};


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub public_code: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewRoom {
    pub name: String,
    pub public_code: String,
    pub is_active: bool,
}

impl From<CreateRoomDto> for NewRoom{
    fn from(dto: CreateRoomDto) -> Self {
        debug!("Converting CreateRoomDto to NewRoom: {}", dto.name);
        Self {
            name: dto.name,
            public_code: String::new(),
            is_active: dto.is_active,
        }
    }
}

impl From<Room> for RoomResponseDto  {
    fn from(room: Room) -> Self {
        debug!("Converting Room to RoomResponseDto: {} (id: {})", room.name, room.id);
        Self {
            id: room.id,
            name: room.name,
            public_code: room.public_code,
            is_active: room.is_active,
            created_at: room.created_at
        }
    }
}



