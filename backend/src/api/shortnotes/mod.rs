use axum::{
    http::{HeaderValue, Method},
    routing::{delete, get, post, put},
    Router,
};

use crate::AppState;

use tower_http::cors::{Any, CorsLayer};

mod diary;
mod meta;
mod note;
mod ws;

pub fn register() -> Router<AppState> {
    Router::new()
        .route("/note", post(note::save_note))
        .route("/note/:uid", get(note::get_note))
        .route("/note/:uid", put(note::update_note))
        .route("/note", delete(note::delete_note))
        .route("/note/all", get(note::get_notes))
        .route("/note/tag/:tag", get(note::get_notes_by_tag))
        .route("/ws/:room", get(ws::ws_handler))
        .route("/metas", post(meta::get_metas))
        .route("/meta", post(meta::create_meta))
        .route("/diary", post(diary::create_diarynote))
        .route("/diary/:dnid", get(diary::get_or_create_diarynote_by_date))
        .route("/diary/:dnid", put(diary::update_diary))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
                .allow_headers(Any),
        )
}
