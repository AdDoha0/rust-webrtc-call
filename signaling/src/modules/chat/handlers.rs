use axum::{extract::{State, Path, Query}, response::IntoResponse, Json};
use tracing::{info, warn, error, instrument, Span};

use crate::{
    AppState,
    common::error::AppError,
    common::response::ApiResponse,
};
use super::dto::{
    input::{CreateMessageDto, MessageQueryParams},
};

#[instrument(skip(state), fields(room_id = room_id))]
pub async fn create_message_handler(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Json(payload): Json<CreateMessageDto>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating new message for room: {}", room_id);
    
    let result = state.services().chat().create_message(room_id, payload).await?;
    
    info!("Message created successfully with id: {}", result.id);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(room_id = room_id))]
pub async fn get_messages_handler(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(params): Query<MessageQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching messages for room: {} with pagination", room_id);
    
    let result = state.services().chat().get_messages_by_room_id(room_id, params).await?;
    
    info!("Retrieved {} messages for room: {}", result.messages.len(), room_id);
    Ok(ApiResponse::success(result))
} 