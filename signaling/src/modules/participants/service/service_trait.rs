use async_trait::async_trait;

use crate::modules::participants::{
    dto::{
        input::{CreateParticipantDto, UpdateParticipantDto},
        output::ParticipantResponseDto
    },
    repository::repository_trait::ParticipantRepository
};
use crate::common::error::AppError;

#[async_trait]
pub trait ParticipantService: Send + Sync {
    async fn create_participant(&self, dto: CreateParticipantDto, room_id: i32) -> Result<ParticipantResponseDto, AppError>;
    async fn get_participants_by_room_id(&self, room_id: i32) -> Result<Vec<ParticipantResponseDto>, AppError>;
    async fn update_participant(&self, id: i32, dto: UpdateParticipantDto) -> Result<ParticipantResponseDto, AppError>;
    async fn delete_participant(&self, id: i32) -> Result<(), AppError>;
} 