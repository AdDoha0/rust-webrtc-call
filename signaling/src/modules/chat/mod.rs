pub mod entity;
pub mod dto;
pub mod repository;
pub mod service;
pub mod handlers;
pub mod routes;

// Константы для логирования
pub const CHAT_LOG_TARGET: &str = "chat";
pub const CHAT_HANDLER_LOG_TARGET: &str = "chat::handlers";
pub const CHAT_SERVICE_LOG_TARGET: &str = "chat::service";
pub const CHAT_REPOSITORY_LOG_TARGET: &str = "chat::repository"; 