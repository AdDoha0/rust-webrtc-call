use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::common::error::{AppError, DomainError};
use crate::modules::participants::{
    dto::{
        input::{CreateParticipantDto, UpdateParticipantDto},
        output::ParticipantResponseDto
    },
    entity::NewParticipant,
    repository::repository_trait::ParticipantRepository,
    service::service_trait::ParticipantService,
    PARTICIPANT_SERVICE_LOG_TARGET,
};

pub struct ParticipantServiceImpl<R>
where
    R: ParticipantRepository,
{
    repository: R,
}

impl<R> ParticipantServiceImpl<R>
where
    R: ParticipantRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl<R> ParticipantService for ParticipantServiceImpl<R>
where
    R: ParticipantRepository,
{
    async fn create_participant(&self, dto: CreateParticipantDto, room_id: i32) -> Result<ParticipantResponseDto, AppError> {
        debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Creating participant: {} in room {}", dto.name, room_id);
        
        let mut new_participant: NewParticipant = (dto, room_id).into();
        new_participant.client_id = Uuid::new_v4().to_string(); 
        debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Generated public code: {}", new_participant.client_id);

        let participant = self.repository.insert_participant(new_participant).await?;
        let response: ParticipantResponseDto = participant.into();

        debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Participant created successfully: {}", response.name);
        Ok(response)
    }

    async fn get_participants_by_room_id(&self, room_id: i32) -> Result<Vec<ParticipantResponseDto>, AppError> {
        debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Getting participants for room: {}", room_id);
        
        let participants = self.repository.select_participants_by_room_id(room_id).await?;
        let response: Vec<ParticipantResponseDto> = participants.into_iter().map(|p| p.into()).collect();
        
        debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Found {} participants for room {}", response.len(), room_id);
        
        Ok(response)
    }

    async fn update_participant(&self, id: i32, dto: UpdateParticipantDto) -> Result<ParticipantResponseDto, AppError> {
        debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Updating participant with id: {}", id);
        
        let participant = self.repository.update_participant_by_id(id, dto).await?;
        
        match participant {
            Some(p) => {
                let response: ParticipantResponseDto = p.into();
                debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Participant updated successfully: {}", response.name);
                Ok(response)
            }
            None => {
                warn!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Participant not found for update: {}", id);
                Err(AppError::Domain(DomainError::NotFound("Participant not found".to_string())))
            }
        }
    }

    async fn delete_participant(&self, id: i32) -> Result<(), AppError> {
        debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Deleting participant with id: {}", id);
        
        let deleted_count = self.repository.delete_participant_by_id(id).await?;
        
        if deleted_count == 0 {
            warn!(target: PARTICIPANT_SERVICE_LOG_TARGET, "No participant found to delete with id: {}", id);
            return Err(AppError::Domain(DomainError::NotFound("Participant not found".to_string())));
        }
        
        debug!(target: PARTICIPANT_SERVICE_LOG_TARGET, "Participant deleted successfully");
        Ok(())
    }
} 