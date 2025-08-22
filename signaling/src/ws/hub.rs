use std::sync::Arc;
use dashmap::DashMap;
use axum::extract::ws::Message;
use tokio::sync::mpsc;
use super::dto::Outbound;
use super::errors::WsError;

#[derive(Clone)]
pub struct WsHub {
    inner: Arc<DashMap<i32, Room>>, // room_id -> Room
}

#[derive(Clone)]
struct Room {
    participants: Arc<DashMap<i32, ClientHandle>>, // participant_id -> handle
    seq: Arc<dashmap::atomic::AtomicU64>,
}

#[derive(Clone)]
struct ClientHandle { tx: mpsc::Sender<Message> }

impl WsHub {
    pub fn new() -> Self { Self { inner: Arc::new(DashMap::new()) } }

    fn room_entry(&self, room_id: i32) -> Room {
        self.inner
            .entry(room_id)
            .or_insert_with(|| Room {
                participants: Arc::new(DashMap::new()),
                seq: Arc::new(dashmap::atomic::AtomicU64::new(0)),
            })
            .clone()
    }

    pub fn add(&self, room_id: i32, participant_id: i32, tx: mpsc::Sender<Message>) {
        let room = self.room_entry(room_id);
        room.participants.insert(participant_id, ClientHandle { tx });
    }

    pub fn remove(&self, room_id: i32, participant_id: i32) {
        if let Some(room) = self.inner.get(&room_id) {
            room.participants.remove(&participant_id);
            if room.participants.is_empty() { drop(room); self.inner.remove(&room_id); }
        }
    }

    pub fn next_seq(&self, room_id: i32) -> u64 {
        self.room_entry(room_id).seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
    }

    pub fn send_to(&self, room_id: i32, to: i32, payload: Outbound) -> Result<(), WsError> {
        let msg = serde_json::to_string(&payload).map_err(|_| WsError::Internal)?;
        if let Some(room) = self.inner.get(&room_id) {
            if let Some(handle) = room.participants.get(&to) {
                return handle.tx.try_send(Message::Text(msg)).map_err(|_| WsError::SlowConsumer);
            }
            return Err(WsError::TargetNotFound);
        }
        Err(WsError::RoomNotFound)
    }

    pub fn broadcast(&self, room_id: i32, payload: Outbound) -> Result<(), WsError> {
        let msg = serde_json::to_string(&payload).map_err(|_| WsError::Internal)?;
        if let Some(room) = self.inner.get(&room_id) {
            for h in room.participants.iter() { let _ = h.tx.try_send(Message::Text(msg.clone())); }
            return Ok(());
        }
        Err(WsError::RoomNotFound)
    }
}