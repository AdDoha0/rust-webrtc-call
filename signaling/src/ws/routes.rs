use axum::{Router, routing::get};
use crate::app_state::AppState;
use super::handler::ws_handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ws/rooms/:room_id/participants/:participant_id", get(ws_handler))
}