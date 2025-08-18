use axum::{Router, routing::{get, post}};
use crate::AppState;

use super::handlers::*;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/rooms/{room_id}/messages",
            get(get_messages_handler)
            .post(create_message_handler)
        )
} 