use axum::{Router, routing::{get, post, patch, delete}};
use crate::AppState;
use super::handlers::*;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/rooms", post(create_room_handler))
        .route("/rooms/id/{id}", get(get_room_handler))    
        .route("/rooms/code/{public_code}", get(get_room_by_public_code_handler))
        .route("/rooms/{id}", patch(update_room_handler))
        .route("/rooms/{id}", delete(delete_room_handler))
}


// pub fn routes() -> Router<AppState> {
//     Router::new()
//         .route("/lessons", 
//         get(list_lessons_handler)
//         .post(create_lesson_handler))
//         .route("/lessons/{id}",
//             get(get_lesson_handler)
//             .delete(delete_lesson_handler)
//             .patch(update_lesson_handler)
//         )
// }