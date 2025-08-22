use serde::{Serialize, Deserialize};

// Client -> Server
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Inbound {
    Offer { to: i32, sdp: String },
    Answer { to: i32, sdp: String },
    IceCandidate { to: i32, candidate: String },
    Chat { message: String },
    Ping,
}

// Server -> Client
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Outbound {
    Joined { participant_id: i32, room_id: i32, at: i64 },
    Left { participant_id: i32, room_id: i32, at: i64 },

    Offer { from: i32, sdp: String },
    Answer { from: i32, sdp: String },
    IceCandidate { from: i32, candidate: String },

    Chat { from: i32, message: String, at: i64, seq: u64 },

    Pong { at: i64 },
    Error { code: u16, message: String },
}