// use axum::{
//     extract::{Path, State},
//     http::StatusCode,
//     Json,
// };
// use uuid::Uuid;
// use crate::{
//     app_state::AppState,
//     modules::rooms::{
//         service::RoomService,
//         dto::{
//             input::{CreateRoomRequest, UpdateRoomRequest, RoomId, PublicCode},
//             output::{RoomResponse, CreateRoomResponse},
//         },
//     },
//     common::error::AppError,
// };

// pub async fn create_room(
//     State(state): State<AppState>,
//     Json(request): Json<CreateRoomRequest>,
// ) -> Result<Json<CreateRoomResponse>, AppError> {
//     let repository = crate::modules::rooms::repository::RoomRepository::new(state.db_pool);
//     let service = RoomService::new(repository);
    
//     let response = service.create_room(request).await?;
//     Ok(Json(response))
// }

// pub async fn get_room_by_id(
//     State(state): State<AppState>,
//     Path(room_id): Path<Uuid>,
// ) -> Result<Json<RoomResponse>, AppError> {
//     let repository = crate::modules::rooms::repository::RoomRepository::new(state.db_pool);
//     let service = RoomService::new(repository);
    
//     let response = service.get_room_by_id(RoomId(room_id)).await?;
//     Ok(Json(response))
// }

// pub async fn get_room_by_public_code(
//     State(state): State<AppState>,
//     Path(public_code): Path<String>,
// ) -> Result<Json<RoomResponse>, AppError> {
//     let repository = crate::modules::rooms::repository::RoomRepository::new(state.db_pool);
//     let service = RoomService::new(repository);
    
//     let response = service.get_room_by_public_code(PublicCode(public_code)).await?;
//     Ok(Json(response))
// }

// pub async fn update_room(
//     State(state): State<AppState>,
//     Path(room_id): Path<Uuid>,
//     Json(request): Json<UpdateRoomRequest>,
// ) -> Result<Json<RoomResponse>, AppError> {
//     let repository = crate::modules::rooms::repository::RoomRepository::new(state.db_pool);
//     let service = RoomService::new(repository);
    
//     let response = service.update_room(RoomId(room_id), request).await?;
//     Ok(Json(response))
// }

// pub async fn delete_room(
//     State(state): State<AppState>,
//     Path(room_id): Path<Uuid>,
// ) -> Result<StatusCode, AppError> {
//     let repository = crate::modules::rooms::repository::RoomRepository::new(state.db_pool);
//     let service = RoomService::new(repository);
    
//     service.delete_room(RoomId(room_id)).await?;
//     Ok(StatusCode::NO_CONTENT)
// } 