use std::{clone, collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc};

// use futures_util::{SinkExt, StreamExt};

use axum::{
    extract::ws::{WebSocket, Message, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use uuid::Uuid;

type Tx = mpsc::UnboundedSender<String>;

pub struct WsHub {
    clients: HashMap<String, Tx>
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    // Добавить нового клиента
    pub fn add_client(&mut self, client_id: String, sender: Tx) {
        self.clients.insert(client_id, sender);
    }

    // Удалить клиента 
    pub fn remove_client(&mut self, client_id: String) {
        self.clients.remove(&client_id); 
    }

    // Отправить сообщение по id
    pub fn send_to(&self, client_id: String, msg: &str) {
        if let Some(tx) = self.clients.get(&client_id) {
            let _ = tx.send(msg.to_string());
        } 
    }


    // Рассылка сообщения всем клиентам
    pub fn broadcast(&self, msg: &str) {
        for tx in self.clients.values() {
            let _ = tx.send(msg.to_string());
        }
    }
}

