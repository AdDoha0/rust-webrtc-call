use axum::{extract::{State, Path}, response::IntoResponse, Json};
use tracing::{info, warn, error, instrument, Span};

use crate::{
    AppState,
    common::error::AppError,
    common::response::ApiResponse,
};
use super::dto::{
    input::{CreateParticipantDto, UpdateParticipantDto},
};

#[instrument(skip(state), fields(participant_name = payload.name.as_str(), room_id = room_id))]
pub async fn create_participant_handler(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Json(payload): Json<CreateParticipantDto>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating new participant with name: {} in room: {}", payload.name, room_id);
    
    let result = state.services().participant().create_participant(payload, room_id).await?;
    
    info!("Participant created successfully with id: {}", result.id);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(room_id = room_id))]
pub async fn get_participants_by_room_handler(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching participants for room: {}", room_id);
    
    let result = state.services().participant().get_participants_by_room_id(room_id).await?;
    
    info!("Found {} participants for room {}", result.len(), room_id);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(participant_id = id))]
pub async fn update_participant_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateParticipantDto>,
) -> Result<impl IntoResponse, AppError> {
    info!("Updating participant with id: {}", id);
    
    let result = state.services().participant().update_participant(id, dto).await?;
    
    info!("Participant updated successfully: {}", result.name);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(participant_id = id))]
pub async fn delete_participant_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, AppError> { 
    info!("Deleting participant with id: {}", id);
    
    state.services().participant().delete_participant(id).await?; 
    
    info!("Participant deleted successfully");
    Ok(ApiResponse::message("deleted"))
} 