use axum::{Router, routing::get};

async fn index() -> &'static str {
    "Hello, World!"
}

pub fn app_router() -> Router<()> {
    Router::new()
        .route("/", get(index))
}