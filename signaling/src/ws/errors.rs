use thiserror::Error;
use super::dto::Outbound;

#[derive(Error, Debug)]
pub enum WsError {
    #[error("room not found")] RoomNotFound,
    #[error("participant not found in room")] ParticipantNotInRoom,
    #[error("target not found in room")] TargetNotFound,
    #[error("slow consumer")] SlowConsumer,
    #[error("rate limited")] RateLimited,
    #[error("invalid message: {0}")] InvalidMessage(String),
    #[error("internal error")] Internal,
}

impl WsError {
    pub fn to_outbound(&self) -> Outbound {
        match self {
            WsError::RoomNotFound => Outbound::Error { code: 404, message: "room not found".into() },
            WsError::ParticipantNotInRoom => Outbound::Error { code: 404, message: "participant not in room".into() },
            WsError::TargetNotFound => Outbound::Error { code: 404, message: "target not found".into() },
            WsError::SlowConsumer => Outbound::Error { code: 429, message: "client is too slow".into() },
            WsError::RateLimited => Outbound::Error { code: 429, message: "rate limited".into() },
            WsError::InvalidMessage(m) => Outbound::Error { code: 400, message: m.clone() },
            WsError::Internal => Outbound::Error { code: 500, message: "internal".into() },
        }
    }
}