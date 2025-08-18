
use sqlx::PgPool;
use tokio::sync::RwLock;
use std::sync::Arc;

use super::ws::hub::WsHub;
use super::container::{ServiceBuilder, Services};



type Hub = Arc<RwLock<WsHub>>;


#[derive(Clone)]
pub struct AppState {
    hub: Hub,
    db_pool: PgPool,
    services: Arc<Services>
}


impl AppState {
    pub fn new(db_pool: PgPool, hub: Hub) -> Self {
        let services = ServiceBuilder::new(db_pool.clone()).build();
        Self {
            hub,
            db_pool,
            services: Arc::new(services),
        }
    }

    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }

    pub fn hub(&self) -> &Hub {
        &self.hub
    }
}