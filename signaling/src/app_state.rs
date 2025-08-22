
use sqlx::PgPool;
use std::sync::Arc;

use super::ws::WsHub;
use super::container::{ServiceBuilder, Services};


#[derive(Clone)]
pub struct AppState {
    ws_hub: WsHub,
    db_pool: PgPool,
    services: Arc<Services>
}


impl AppState {
    pub fn new(db_pool: PgPool, ws_hub: WsHub) -> Self {
        let services = ServiceBuilder::new(db_pool.clone()).build();
        Self {
            ws_hub,
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

    pub fn ws_hub(&self) -> &WsHub {
        &self.ws_hub
    }
}