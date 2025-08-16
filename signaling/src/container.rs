use sqlx::PgPool;
use std::sync::Arc;
use crate::modules::rooms::{
    repository::postgres::PostgresRoomRepository, 
    service::{
        service_impl::RoomServiceImpl, 
        service_trait::RoomService
    }
};
use crate::modules::participants::{
    repository::postgres::PostgresParticipantRepository,
    service::{
        service_impl::ParticipantServiceImpl,
        service_trait::ParticipantService
    }
};


#[derive(Clone)]
pub struct Services {
    pub room: Arc<dyn RoomService + Send + Sync>,
    pub participant: Arc<dyn ParticipantService + Send + Sync>,
}


impl Services {
    pub fn room(&self) -> &(dyn RoomService + Send + Sync) {
        &*self.room
    }
    
    pub fn participant(&self) -> &(dyn ParticipantService + Send + Sync) {
        &*self.participant
    }
}


pub struct ServiceBuilder {
    pub db_pool: PgPool,
    pub room: Option<Arc<dyn RoomService + Send + Sync>>,
    pub participant: Option<Arc<dyn ParticipantService + Send + Sync>>,
}


impl ServiceBuilder {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool,
            room: None,
            participant: None
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

        let participant = self.participant.unwrap_or_else(|| {
            let participant_repository = PostgresParticipantRepository::new(self.db_pool.clone());
            Arc::new(ParticipantServiceImpl::new(participant_repository))
        });

        Services {
            room,
            participant,
        }
    }
}