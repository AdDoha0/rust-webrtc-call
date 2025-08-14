use sqlx::PgPool;
use tokio::sync::RwLock;
use std::sync::Arc;
use super::ws::hub::WsHub;


#[derive(Clone)]
pub struct AppState {
    pub hub: Arc<RwLock<WsHub>>,
    pub db_pool: PgPool,
}