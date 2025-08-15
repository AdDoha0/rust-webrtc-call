use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;


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
        Self {
            name: dto.name,
            public_code: String::new(),
            is_active: dto.is_active,
        }
    }
}

impl From<Room> for RoomResponseDto  {
    fn from(tb: Room) -> Self {
        Self {
            id: tb.id,
            name: tb.name,
            public_code: tb.public_code,
            is_active: tb.is_active,
            created_at: tb.created_at
        }
    }
}



