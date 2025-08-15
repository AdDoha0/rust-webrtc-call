use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRoomDto {
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoomDto {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}

