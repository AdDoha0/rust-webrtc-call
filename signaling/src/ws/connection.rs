use std::time::{Duration, Instant};
use axum::extract::ws::{Message, WebSocket};
use futures_util::{StreamExt, SinkExt};
use tokio::{sync::mpsc, time};
use tracing::{instrument, warn};
use sqlx::{PgPool};

use super::{dto, errors::WsError, hub::WsHub};

// Simple token-bucket rate limiter
struct RateLimiter { capacity: u32, tokens: f64, refill_per_sec: f64, last: Instant }
impl RateLimiter {
    fn new(capacity: u32, refill_per_sec: f64) -> Self { Self { capacity, tokens: capacity as f64, refill_per_sec, last: Instant::now() } }
    fn allow(&mut self) -> bool {
        let now = Instant::now();
        let dt = now.duration_since(self.last).as_secs_f64();
        self.last = now;
        self.tokens = (self.tokens + dt * self.refill_per_sec).min(self.capacity as f64);
        if self.tokens >= 1.0 { self.tokens -= 1.0; true } else { false }
    }
}

#[instrument(skip(socket, hub, db), fields(room_id = room_id, participant_id = participant_id))]
pub async fn run_connection(
    mut socket: WebSocket,
    hub: WsHub,
    db: PgPool,
    room_id: i32,
    participant_id: i32,
) {
    let (tx, mut rx) = mpsc::channel::<Message>(64);
    hub.add(room_id, participant_id, tx);

    // Writer task
    let mut writer = socket.clone();
    let writer_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = writer.send(msg).await { warn!(?e, "writer send failed"); break; }
        }
    });

    let now = chrono::Utc::now().timestamp_millis();
    let _ = hub.broadcast(room_id, dto::Outbound::Joined { participant_id, room_id, at: now });

    let mut limiter = RateLimiter::new(20, 10.0); // burst 20, 10 msg/s
    let mut ping_interval = time::interval(Duration::from_secs(20));
    ping_interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
    let mut last_pong = Instant::now();

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                if !limiter.allow() { let _ = hub.send_to(room_id, participant_id, WsError::RateLimited.to_outbound()); continue; }
                match serde_json::from_str::<dto::Inbound>(&text) {
                    Ok(dto::Inbound::Offer { to, sdp }) => {
                        let payload = dto::Outbound::Offer { from: participant_id, sdp };
                        if let Err(e) = hub.send_to(room_id, to, payload) { let _ = hub.send_to(room_id, participant_id, e.to_outbound()); }
                    }
                    Ok(dto::Inbound::Answer { to, sdp }) => {
                        let payload = dto::Outbound::Answer { from: participant_id, sdp };
                        if let Err(e) = hub.send_to(room_id, to, payload) { let _ = hub.send_to(room_id, participant_id, e.to_outbound()); }
                    }
                    Ok(dto::Inbound::IceCandidate { to, candidate }) => {
                        let payload = dto::Outbound::IceCandidate { from: participant_id, candidate };
                        if let Err(e) = hub.send_to(room_id, to, payload) { let _ = hub.send_to(room_id, participant_id, e.to_outbound()); }
                    }
                    Ok(dto::Inbound::Chat { message }) => {
                        // Persist chat in DB
                        if let Err(e) = sqlx::query!(
                            r#"INSERT INTO chat_messages (room_id, sender_id, message) VALUES ($1, $2, $3)"#,
                            room_id, participant_id, message
                        ).execute(&db).await { warn!(?e, "failed to persist chat message"); }

                        let seq = hub.next_seq(room_id);
                        let payload = dto::Outbound::Chat { from: participant_id, message, at: chrono::Utc::now().timestamp_millis(), seq };
                        let _ = hub.broadcast(room_id, payload);
                    }
                    Ok(dto::Inbound::Ping) => {
                        let _ = hub.send_to(room_id, participant_id, dto::Outbound::Pong { at: chrono::Utc::now().timestamp_millis() });
                    }
                    Err(e) => {
                        let _ = hub.send_to(room_id, participant_id, WsError::InvalidMessage(e.to_string()).to_outbound());
                    }
                }
            }
            Message::Binary(_) => { let _ = hub.send_to(room_id, participant_id, WsError::InvalidMessage("binary not supported".into()).to_outbound()); }
            Message::Ping(_) => { /* handled by axum */ }
            Message::Pong(_) => { last_pong = Instant::now(); }
            Message::Close(_) => { break; }
        }

        // Heartbeat close if no pong 60s
        if last_pong.elapsed() > Duration::from_secs(60) { let _ = socket.close().await; break; }
    }

    hub.remove(room_id, participant_id);
    let now = chrono::Utc::now().timestamp_millis();
    let _ = hub.broadcast(room_id, dto::Outbound::Left { participant_id, room_id, at: now });
    let _ = writer_task.await;
}