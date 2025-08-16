use sqlx::PgPool;
use std::sync::Arc;
use crate::modules::rooms::{
    repository::postgres::PostgresRoomRepository, 
    service::{
        service_impl::RoomServiceImpl, 
        service_trait::RoomService
    }
};


#[derive(Clone)]
pub struct Services {
    pub room: Arc<dyn RoomService + Send + Sync>,
}


impl Services {
    pub fn room(&self) -> &(dyn RoomService + Send + Sync) {
        &*self.room
    }   
}


pub struct ServiceBuilder {
    pub db_pool: PgPool,
    pub room: Option<Arc<dyn RoomService + Send + Sync>>,
}


impl ServiceBuilder {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool,
            room: None
        }
    }


    pub fn with_room(
        mut self,
        room: Arc<dyn RoomService + Send + Sync>
    ) -> Self {
        self.room = Some(room);
        self
    }


    pub fn build(self) -> Services {
        let room = self.room.unwrap_or_else(|| {
            let room_repository = PostgresRoomRepository::new(self.db_pool.clone());
            Arc::new(RoomServiceImpl::new(room_repository))
        });

        Services {
            room,
        }
    }
}