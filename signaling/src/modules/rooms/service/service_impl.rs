use uuid::Uuid;
use async_trait::async_trait;
use tracing::{debug, warn};

use crate::modules::rooms::{
    dto::{
        input::{CreateRoomDto, UpdateRoomDto},
        output::RoomResponseDto
    },
    entity::NewRoom,
    repository::repository_trait::RoomRepository,
    service::service_trait::RoomService,
    ROOM_SERVICE_LOG_TARGET,
};
use crate::common::error::{DomainError, AppError};

#[derive(Clone)]
pub struct RoomServiceImpl<R>
where
    R: RoomRepository,
{
    repository: R,
}

impl<R> RoomServiceImpl<R>
where
    R: RoomRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> RoomService for RoomServiceImpl<R>
where
    R: RoomRepository + Send + Sync + 'static,
{
    async fn create_room(&self, dto: CreateRoomDto) -> Result<RoomResponseDto, AppError> {
        debug!(target: ROOM_SERVICE_LOG_TARGET, "Creating room: {}", dto.name);

        let mut new_room = NewRoom::from(dto);
        new_room.public_code = Uuid::new_v4().to_string();
        debug!(target: ROOM_SERVICE_LOG_TARGET, "Generated public code: {}", new_room.public_code);

        let room = self.repository.insert_room(new_room).await?;
        let response: RoomResponseDto = room.into();

        debug!(target: ROOM_SERVICE_LOG_TARGET, "Room created successfully: {} (id: {})", response.name, response.id);
        Ok(response)
    }

    async fn get_room_by_id(&self, id: i32) -> Result<RoomResponseDto, AppError> {
        debug!(target: ROOM_SERVICE_LOG_TARGET, "Fetching room by id: {}", id);

        let room = self.repository.select_room_by_id(id).await?;
        match room {
            Some(r) => {
                let response: RoomResponseDto = r.into();
                debug!(target: ROOM_SERVICE_LOG_TARGET, "Room retrieved successfully: {} (id: {})", response.name, response.id);
                Ok(response)
            }
            None => {
                warn!(target: ROOM_SERVICE_LOG_TARGET, "Room not found with id: {}", id);
                Err(AppError::Domain(DomainError::RoomNotFound(format!("Room with id={} not found", id))))
            }
        }
    }

    async fn get_room_by_public_code(&self, public_code: &str) -> Result<RoomResponseDto, AppError> {
        debug!(target: ROOM_SERVICE_LOG_TARGET, "Fetching room by public_code: {}", public_code);

        let room = self.repository.select_room_by_public_code(public_code).await?;
        match room {
            Some(r) => {
                let response: RoomResponseDto = r.into();
                debug!(target: ROOM_SERVICE_LOG_TARGET, "Room retrieved successfully by public_code: {} (id: {})", response.name, response.id);
                Ok(response)
            }
            None => {
                warn!(target: ROOM_SERVICE_LOG_TARGET, "Room not found with public_code: {}", public_code);
                Err(AppError::Domain(DomainError::RoomNotFound(format!("Room with public_code={} not found", public_code))))
            }
        }
    }

    async fn update_room(&self, id: i32, dto: UpdateRoomDto) -> Result<RoomResponseDto, AppError> {
        debug!(target: ROOM_SERVICE_LOG_TARGET, "Updating room with id: {}", id);

        let room = self.repository.update_room_by_id(id, dto).await?;
        match room {
            Some(r) => {
                let response: RoomResponseDto = r.into();
                debug!(target: ROOM_SERVICE_LOG_TARGET, "Room updated successfully: {} (id: {})", response.name, response.id);
                Ok(response)
            }
            None => {
                warn!(target: ROOM_SERVICE_LOG_TARGET, "Room not found for update with id: {}", id);
                Err(AppError::Domain(DomainError::RoomNotFound(format!("Room with id={} not found", id))))
            }
        }
    }

    async fn delete_room(&self, id: i32) -> Result<(), AppError> {
        debug!(target: ROOM_SERVICE_LOG_TARGET, "Deleting room with id: {}", id);

        let deleted = self.repository.delete_room_by_id(id).await?;
        if deleted == 0 {
            warn!(target: ROOM_SERVICE_LOG_TARGET, "No room found to delete with id: {}", id);
            return Err(AppError::Domain(DomainError::RoomNotFound(format!("Room with id={} not found", id))));
        }

        debug!(target: ROOM_SERVICE_LOG_TARGET, "Room deleted successfully (id: {})", id);
        Ok(())
    }
}
