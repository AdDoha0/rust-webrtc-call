use std::{net::SocketAddr, sync::Arc};
use tracing::info;
use axum::{Router, routing::get};
use tokio::sync::RwLock;

use crate::app_state::AppState;

mod ws;
mod router;
mod app_state;



#[tokio::main]
async fn main() {
    // Загрузим переменные окружения, если есть .env
    let _ = dotenvy::dotenv();

    info!("Starting signaling server...");

    // Хост/порт можно переопределить через переменные окружения HOST/PORT
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000);

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid HOST/PORT provided");

    // Инициализируем общий хаб (в дальнейшем для WS)
    let hub = Arc::new(RwLock::new(ws::hub::WsHub::new()));
    let state = AppState { hub };

    // Базовый роутер c health-check
    let app = Router::new()
        .route("/", get(|| async { "Signaling server is running" }))
        .with_state(state);

    info!("Listening on {}", addr);

    // Запуск сервера (Axum 0.8)
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}


