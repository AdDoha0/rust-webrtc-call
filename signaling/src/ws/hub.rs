use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use axum::extract::ws::{Message as WsMessage, Message}; // импортируем Message правильно

use super::message::{ClientId, SignalMessage}; 


pub type Tx = mpsc::UnboundedSender<WsMessage>;

pub struct WsHub {
    pub rooms: HashMap<String, HashMap<ClientId, Tx>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self { rooms: HashMap::new() }
    }

    pub fn add_client(&mut self, room_id: &str, client_id: ClientId, tx: Tx) {
        self.rooms.entry(room_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(client_id, tx);
    }

    pub fn remove_client(&mut self, room_id: &str, client_id: &ClientId) {
        if let Some(room) = self.rooms.get_mut(room_id) {
            room.remove(client_id);
            if room.is_empty() {
                self.rooms.remove(room_id); // удаляем комнату, если она пуста
            }
        }
    }

    pub async fn send_to(&self, room_id: &str, target: &ClientId, msg: &SignalMessage) {
        if let Some(room) = self.rooms.get(room_id) {
            if let Some(tx) = room.get(target) {
                let text = serde_json::to_string(msg).unwrap().into();
                let _ = tx.send(text);
            }
        }
    }

    pub async fn broadcast(&self, room_id: &str, msg: &SignalMessage) {
        if let Some(room) = self.rooms.get(room_id) {
            let text = WsMessage::Text(serde_json::to_string(msg).unwrap().into());
            for tx in room.values() {
                let _ = tx.send(text.clone());
            }
        }
    }
}