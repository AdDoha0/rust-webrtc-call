use axum::{extract::{State, Path, Query}, response::IntoResponse, Json};
use tracing::{info, warn, error, instrument, Span};

use crate::{
    AppState,
    common::error::AppError,
    common::response::ApiResponse,
    common::extractors::resource_id::ResourceId,

};

use super::dto::{
    input::{CreateMessageDto, MessageQueryParams},
};

#[instrument(skip(state), fields(room_id = room_id.value))]
pub async fn create_message_handler(
    State(state): State<AppState>,
    room_id: ResourceId<i32>,
    Json(payload): Json<CreateMessageDto>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating new message for room: {}", room_id.value);
    
    let result = state.services().chat().create_message(room_id.value, payload).await?;
    
    info!("Message created successfully with id: {}", result.id);
    Ok(ApiResponse::created(result))
}

#[instrument(skip(state), fields(room_id = room_id.value))]
pub async fn get_messages_handler(
    State(state): State<AppState>,
    room_id: ResourceId<i32>,
    Query(params): Query<MessageQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching messages for room: {} with pagination", room_id.value);
    
    let result = state.services().chat().get_messages_by_room_id(room_id.value, params).await?;
    
    info!("Retrieved {} messages for room: {}", result.messages.len(), room_id.value);
    Ok(ApiResponse::success(result))
} 