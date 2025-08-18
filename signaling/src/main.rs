use std::{net::SocketAddr, sync::Arc};
use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter};
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

fn init_logging() {
    // Настройка логирования с фильтрацией по уровням
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("info")
                .add_directive("rooms=debug".parse().unwrap())
                .add_directive("rooms::handlers=info".parse().unwrap())
                .add_directive("rooms::service=debug".parse().unwrap())
                .add_directive("rooms::repository=debug".parse().unwrap())
                .add_directive("participants=debug".parse().unwrap())
                .add_directive("participants::handlers=info".parse().unwrap())
                .add_directive("participants::service=debug".parse().unwrap())
                .add_directive("participants::repository=debug".parse().unwrap())
                .add_directive("chat=debug".parse().unwrap())
                .add_directive("chat::handlers=info".parse().unwrap())
                .add_directive("chat::service=debug".parse().unwrap())
                .add_directive("chat::repository=debug".parse().unwrap())
                .add_directive("sqlx=warn".parse().unwrap())
        });

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}

#[tokio::main]
async fn main() {
    // Загрузим переменные окружения, если есть .env
    let _ = dotenvy::dotenv();
    
    // Инициализируем логирование
    init_logging();

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

    info!("Connecting to database...");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");
    info!("Database connection established successfully");

    let state = AppState::new(db_pool, hub);
    let app = app_router(state);


    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");

}


