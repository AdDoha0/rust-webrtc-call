use std::{net::SocketAddr, sync::Arc};
use tracing::info;
use axum::{Router, routing::get};
use tokio::sync::RwLock;
use sqlx::PgPool;
use std::env;

use crate::app_state::AppState;
use crate::router::app_router;

mod ws;
mod router;
mod app_state;
mod common;
mod modules;
mod container;



#[tokio::main]
async fn main() {
    // Загрузим переменные окружения, если есть .env
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

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

    // Инициализируем общий хаб 
    let hub = Arc::new(RwLock::new(ws::hub::WsHub::new()));

    dotenv::dotenv().ok(); 
    let database_url = env::var("DATABASE_URL").expect("Connect database url!");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let app = app_router(AppState::new(db_pool, hub));

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}


