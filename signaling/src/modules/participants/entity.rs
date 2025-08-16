use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::debug;

use super::dto::{
    input::{CreateParticipantDto, UpdateParticipantDto},
    output::ParticipantResponseDto, 
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Participant {
    pub id: i32,
    pub room_id: i32,
    pub client_id: String,
    pub name: String,
    pub is_host: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewParticipant {
    pub room_id: i32,
    pub client_id: String,
    pub name: String,
    pub is_host: bool,
}

impl From<(CreateParticipantDto, i32)> for NewParticipant {
    fn from((dto, room_id): (CreateParticipantDto, i32)) -> Self {
        debug!("Converting CreateParticipantDto to NewParticipant: {} in room {}", dto.name, room_id);
        Self {
            room_id,
            client_id: String::new(),
            name: dto.name,
            is_host: dto.is_host,
        }
    }
}

impl From<Participant> for ParticipantResponseDto {
    fn from(participant: Participant) -> Self {
        debug!("Converting Participant to ParticipantResponseDto: {} (id: {})", participant.name, participant.id);
        Self {
            id: participant.id,
            room_id: participant.room_id,
            client_id: participant.client_id,
            name: participant.name,
            is_host: participant.is_host,
        }
    }
} 