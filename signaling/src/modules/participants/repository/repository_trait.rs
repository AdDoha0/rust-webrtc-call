use async_trait::async_trait;
use crate::common::error::AppError;

use crate::modules::participants::entity::{Participant, NewParticipant};
use crate::modules::participants::dto::input::UpdateParticipantDto;

#[async_trait]
pub trait ParticipantRepository: Send + Sync {
    async fn insert_participant(&self, dto: NewParticipant) -> Result<Participant, AppError>;
    async fn select_participants_by_room_id(&self, room_id: i32) -> Result<Vec<Participant>, AppError>;
    async fn select_participant_by_id(&self, id: i32) -> Result<Option<Participant>, AppError>;
    async fn update_participant_by_id(&self, id: i32, dto: UpdateParticipantDto) -> Result<Option<Participant>, AppError>;
    async fn delete_participant_by_id(&self, id: i32) -> Result<u64, AppError>;
} 