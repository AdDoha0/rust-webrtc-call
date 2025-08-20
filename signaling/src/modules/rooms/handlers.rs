use axum::{extract::{State, Path}, response::IntoResponse, Json};
use tracing::{info, warn, error, instrument, Span};

use crate::{
    AppState,
    common::error::AppError,
    common::response::ApiResponse,
    common::extractors::resource_id::ResourceId,
};
use super::dto::{
    input::{CreateRoomDto, UpdateRoomDto},
};


#[instrument(skip(state), fields(room_name = payload.name.as_str()))]
pub async fn create_room_handler (
    State(state): State<AppState>,
    Json(payload): Json<CreateRoomDto>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating new room with name: {}", payload.name);
    
    let result = state.services().room().create_room(payload).await?;
    
    info!("Room created successfully with id: {}", result.id);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(room_id = id.value))]
pub async fn get_room_handler(
    State(state): State<AppState>,
    id: ResourceId<i32>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching room by id: {}", id.value);
    
    let result = state.services().room().get_room_by_id(id.value).await?;
    
    info!("Room retrieved successfully: {}", result.name);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(public_code = public_code))]
pub async fn get_room_by_public_code_handler(
    State(state): State<AppState>,
    Path(public_code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching room by public code: {}", public_code);
    
    let result = state.services().room().get_room_by_public_code(public_code).await?;
    
    info!("Room retrieved successfully by public code: {}", result.name);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(room_id = id.value))]
pub async fn update_room_handler(
    State(state): State<AppState>,
    id: ResourceId<i32>,
    Json(dto): Json<UpdateRoomDto>,
) -> Result<impl IntoResponse, AppError> {
    info!("Updating room with id: {}", id.value);
     
    let result = state.services().room().update_room(id.value, dto).await?;
    
    info!("Room updated successfully: {}", result.name);
    Ok(ApiResponse::success(result))
}

#[instrument(skip(state), fields(room_id = id.value))]
pub async fn delete_room_handler(
    State(state): State<AppState>,
    id: ResourceId<i32>,
) -> Result<impl IntoResponse, AppError> { 
    info!("Deleting room with id: {}", id.value);
    
    state.services().room().delete_room(id.value).await?; 
    
    info!("Room deleted successfully");
    Ok(ApiResponse::message("deleted"))
}

