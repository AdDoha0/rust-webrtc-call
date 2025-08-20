use axum::{extract::{State, Path}, response::IntoResponse, Json};
use tracing::{info, warn, error, instrument, Span};

use crate::{
    AppState,
    common::error::AppError,
    common::response::ApiResponse,
    common::extractors::resource_id::ResourceId,
};
use super::dto::{
    input::{CreateParticipantDto, UpdateParticipantDto},
};

#[instrument(skip(state), fields(participant_name = payload.name.as_str(), room_id = room_id.value))]
pub async fn create_participant_handler(
    State(state): State<AppState>,
    room_id: ResourceId<i32>,
    Json(payload): Json<CreateParticipantDto>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating new participant with name: {} in room: {}", payload.name, room_id.value);
    
    let result = state.services().participant().create_participant(payload, room_id.value).await?;
    
    info!("Participant created successfully with id: {}", result.id);
    Ok(ApiResponse::created(result))
}

#[instrument(skip(state), fields(room_id = room_id.value))]
pub async fn get_participants_by_room_handler(
    State(state): State<AppState>,
    room_id: ResourceId<i32>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching participants for room: {}", room_id.value);
    
    let result = state.services().participant().get_participants_by_room_id(room_id.value).await?;
    
    info!("Found {} participants for room {}", result.len(), room_id.value);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(participant_id = id.value))]
pub async fn update_participant_handler(
    State(state): State<AppState>,
    id: ResourceId<i32>,
    Json(dto): Json<UpdateParticipantDto>,
) -> Result<impl IntoResponse, AppError> {
    info!("Updating participant with id: {}", id.value);
    
    let result = state.services().participant().update_participant(id.value, dto).await?;
    
    info!("Participant updated successfully: {}", result.name);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(participant_id = id.value))]
pub async fn delete_participant_handler(
    State(state): State<AppState>,
    id: ResourceId<i32>,
) -> Result<impl IntoResponse, AppError> { 
    info!("Deleting participant with id: {}", id.value);
    
    state.services().participant().delete_participant(id.value).await?; 
    
    info!("Participant deleted successfully");
    Ok(ApiResponse::no_content())
} 