use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub type ClientId = Uuid; // возвращаем использование Uuid для уникальной идентификации клиентов


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SignalMessage {
    Offer { target: ClientId, sdp: String },
    Answer { target: ClientId, sdp: String },
    IceCandidate { target: ClientId, candidate: String },
    Chat { target: Option<ClientId>, message: String },
}