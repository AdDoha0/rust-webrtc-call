use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ParticipantResponseDto {
    pub id: i32,
    pub room_id: i32,
    pub client_id: String,
    pub name: String,
    pub is_host: bool,
} 