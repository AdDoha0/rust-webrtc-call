use axum::{Router, routing::{get, post, patch, delete}};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::ws;
use crate::app_state::AppState; 

async fn index() -> &'static str {
    "Signaling server is running" 
}

pub fn app_router(hub: AppState) -> Router<()> {
    Router::new()
        .route("/", get(index))
        .route("/ws", get(ws::handler::ws_handler))
        // .nest("/rooms", rooms_routes())
        .with_state(hub.clone())
}

// fn rooms_routes() -> Router<AppState> {
//     Router::new()
//         .route("/", post(crate::modules::rooms::handler::create_room))
//         .route("/:room_id", get(crate::modules::rooms::handler::get_room_by_id))
//         .route("/code/:public_code", get(crate::modules::rooms::handler::get_room_by_public_code))
//         .route("/:room_id", patch(crate::modules::rooms::handler::update_room))
//         .route("/:room_id", delete(crate::modules::rooms::handler::delete_room))
// }