use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;


#[derive(Debug, Serialize)]
pub struct RoomResponseDto {
    pub id: i32,
    pub name: String,
    pub public_code: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}


