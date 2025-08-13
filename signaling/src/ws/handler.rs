use crate::ws::message::SignalMessage;
use axum::{
    extract::{
        ws::{WebSocketUpgrade, WebSocket, Message as WsMessage},
        Query, State
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::app_state::AppState;
use std::collections::HashMap;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>, // получаем room_id из query-параметров
    State(hub): State<AppState>
) -> impl IntoResponse {
    let room_id = params
        .get("room_id")
        .cloned()
        .unwrap_or_else(|| "default".to_string()); // дефолтная комната, если не передана

    ws.on_upgrade(move |socket| async move {
        handle_socket(socket, hub, room_id).await;
    })
}


async fn handle_socket(socket: WebSocket, hub: AppState, room_id: String) {
    let client_id = Uuid::new_v4();
    tracing::info!("Client {} connected to room {}", client_id, room_id);

    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();

    {
        let mut hub_write = hub.hub.write().await;
        hub_write.add_client(&room_id, client_id, tx);
    }

    let (mut sender, mut receiver) = socket.split();

    // Отправка сообщений клиенту
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Приём сообщений от клиента
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            WsMessage::Text(text) => {
                tracing::info!("Received from {}: {}", client_id, text);

                if let Ok(signal) = serde_json::from_str::<SignalMessage>(&text) {
                    let hub_read = hub.hub.read().await;
                    match signal {
                        SignalMessage::Chat { target, .. } => {
                            if let Some(target_id) = target {
                                hub_read.send_to(&room_id, &target_id, &signal).await;
                            } else {
                                hub_read.broadcast(&room_id, &signal).await;
                            }
                        }
                        SignalMessage::Offer { target, .. }
                        | SignalMessage::Answer { target, .. }
                        | SignalMessage::IceCandidate { target, .. } => {
                            hub_read.send_to(&room_id, &target, &signal).await;
                        }
                    }
                }
            }
            WsMessage::Close(_) => {
                tracing::info!("Client {} disconnected", client_id);
                break;
            }
            _ => {}
        }
    }

    {
        let mut hub_write = hub.hub.write().await;
        hub_write.remove_client(&room_id, &client_id);
    }

    send_task.abort();
    tracing::info!("Connection handler for {} finished", client_id);
}
