use axum::{Router, routing::{get, post, patch, delete}};
use crate::AppState;

use super::handlers::*;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/rooms/{room_id}/participants",
            get(get_participants_by_room_handler)
            .post(create_participant_handler)
        )
        .route("/participants/{id}",
            patch(update_participant_handler)
            .delete(delete_participant_handler)
        )
} 