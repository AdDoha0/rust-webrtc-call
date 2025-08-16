use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateParticipantDto {
    pub name: String,
    pub is_host: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateParticipantDto {
    pub name: Option<String>,
    pub is_host: Option<bool>,
} 