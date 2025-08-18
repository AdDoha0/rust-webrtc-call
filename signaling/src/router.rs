use axum::{Router, routing::{get, post, patch, delete}};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::ws;
use crate::app_state::AppState;
use crate::modules::{rooms, participants, chat}; 

async fn index() -> &'static str {
    "Signaling server is running" 
}

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/ws", get(ws::handler::ws_handler))
        .nest("/api", api_routes())
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    let api_version = "/v1";

    Router::new()
        .nest(
            &format!("{api_version}"),
            rooms::routes::routes()
        )
        .nest(
            &format!("{api_version}"),
            participants::routes::routes()
        )
        .nest(
            &format!("{api_version}"),
            chat::routes::routes()
        )
}

// fn rooms_routes() -> Router<AppState> {
//     Router::new()
//         .route("/", post(crate::modules::rooms::handler::create_room))
//         .route("/:room_id", get(crate::modules::rooms::handler::get_room_by_id))
//         .route("/code/:public_code", get(crate::modules::rooms::handler::get_room_by_public_code))
//         .route("/:room_id", patch(crate::modules::rooms::handler::update_room))
//         .route("/:room_id", delete(crate::modules::rooms::handler::delete_room))
// }