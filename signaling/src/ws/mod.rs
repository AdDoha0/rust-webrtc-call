pub mod dto;
pub mod errors;
pub mod hub;
pub mod connection;
pub mod handler;
pub mod routes;

// Public re-exports for embedding into AppState if you want
pub use hub::WsHub;

// A small WS state you can hold alongside your AppState
#[derive(Clone)]
pub struct WsState {
    pub hub: WsHub,
}
impl WsState { pub fn new() -> Self { Self { hub: WsHub::new() } } }