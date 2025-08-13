use axum::{Router, routing::get};
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
        .with_state(hub.clone())
}