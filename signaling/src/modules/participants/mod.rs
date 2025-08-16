pub mod entity;
pub mod dto;
pub mod repository;
pub mod service;
pub mod handlers;
pub mod routes;

// Константы для логирования
pub const PARTICIPANT_LOG_TARGET: &str = "participants";
pub const PARTICIPANT_HANDLER_LOG_TARGET: &str = "participants::handlers";
pub const PARTICIPANT_SERVICE_LOG_TARGET: &str = "participants::service";
pub const PARTICIPANT_REPOSITORY_LOG_TARGET: &str = "participants::repository"; 