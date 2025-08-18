use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateMessageDto {
    pub room_id: i32,
    pub sender_id: i32,
    pub message: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageQueryParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

fn default_offset() -> i64 {
    0
}