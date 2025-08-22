use axum::{extract::{State, WebSocketUpgrade, Path}, response::IntoResponse};
use sqlx::{PgPool, Row};
use tracing::instrument;

use crate::common::error::{AppError, DomainError, InfrastructureError};
use crate::app_state::AppState;
use super::connection::run_connection;

#[instrument(skip(ws, state))]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path((room_id, participant_id)): Path<(i32, i32)>,
) -> Result<impl IntoResponse, AppError> {
    // === Validation against DB ===
    // 1) Room must exist and be active
    let room_exists = sqlx::query("SELECT 1 FROM rooms WHERE id = $1 AND is_active = TRUE")
        .bind(room_id)
        .fetch_optional(state.db_pool())
        .await
        .map_err(InfrastructureError::from)?
        .is_some();
    if !room_exists {
        return Err(DomainError::RoomNotFound(room_id.to_string()).into());
    }

    // 2) Participant must belong to that room
    let participant_ok = sqlx::query("SELECT 1 FROM participants WHERE id = $1 AND room_id = $2")
        .bind(participant_id)
        .bind(room_id)
        .fetch_optional(state.db_pool())
        .await
        .map_err(InfrastructureError::from)?
        .is_some();
    if !participant_ok {
        return Err(DomainError::UserNotFound(participant_id.to_string()).into());
    }

    // At this point you can also verify JWT or room ownership (you'll wire it later)

    Ok(ws.on_upgrade(move |socket| async move {
        run_connection(socket, state.ws_hub().clone(), state.db_pool().clone(), room_id, participant_id).await;
    }))
}
