// V2

pub mod types;

use axum::{
    extract::{self, State},
    http::{Method, StatusCode},
    routing::{get, patch, post, put},
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{store, AppState};

use super::{auth::Claims, types::ResponsePayload};

pub fn register() -> Router<AppState> {
    Router::new()
        .route("/", get(get_notes))
        .route("/", put(create_note))
        .route("/:id", get(get_note))
        .route("/:id", patch(update_note))
        .route("/metas", post(get_metas))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                    Method::PATCH,
                ])
                .allow_headers(Any),
        )
}

async fn get_notes(
    State(app): State<AppState>,
    claims: Claims,
) -> (StatusCode, Json<ResponsePayload<Vec<types::Note>>>) {
    let result = store::notes::get_notes_by_user_id(claims.user_id.parse().unwrap(), &app.db).await;

    match result {
        Ok(notes) => (
            StatusCode::OK,
            Json(ResponsePayload {
                status: 200,
                message: "Notes found".to_string(),
                data: Some(
                    notes
                        .iter()
                        .map(|note| types::Note::from_store(note.clone()))
                        .collect(),
                ),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                status: 500,
                message: err.to_string(),
                data: None,
            }),
        ),
    }
}

async fn create_note(
    State(app): State<AppState>,
    claims: Claims,
    extract::Json(note): extract::Json<types::NoteCreate>,
) -> (StatusCode, Json<ResponsePayload<types::Note>>) {
    let result = store::notes::create_note(
        store::notes::types::NoteCreatePayload {
            note_id: nanoid::nanoid!(16),
            user_id: claims.user_id.parse().unwrap(),

            note_type: note.note_type,

            title: "New Note".to_string(),
            tags: sqlx::types::Json(vec![]),
            content: vec![],
        },
        &app.db,
    )
    .await;

    match result {
        Ok(note) => (
            StatusCode::CREATED,
            Json(ResponsePayload {
                status: 201,
                message: "Note created".to_string(),
                data: Some(types::Note::from_store(note.note)),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                status: 500,
                message: err.to_string(),
                data: None,
            }),
        ),
    }
}

async fn get_note(
    State(app): State<AppState>,
    _claims: Claims,
    extract::Path(note_id): extract::Path<String>,
) -> (StatusCode, Json<ResponsePayload<types::Note>>) {
    let result = store::notes::get_note(note_id, &app.db).await;

    match result {
        Ok(note) => (
            StatusCode::OK,
            Json(ResponsePayload {
                status: 200,
                message: "Note found".to_string(),
                data: Some(types::Note::from_store(note.note)),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                status: 500,
                message: err.to_string(),
                data: None,
            }),
        ),
    }
}

async fn update_note(
    State(app): State<AppState>,
    _claims: Claims,
    extract::Path(note_id): extract::Path<String>,
    extract::Json(note): extract::Json<types::NoteUpdate>,
) -> (StatusCode, Json<ResponsePayload<types::Note>>) {
    let result = store::notes::update_note(
        note_id,
        store::notes::types::NoteUpdatePayload {
            title: note.title,
            tags: note.tags.map(|tag| sqlx::types::Json(tag)),
            content: note.content,
        },
        &app.db,
    )
    .await;

    match result {
        Ok(note) => (
            StatusCode::OK,
            Json(ResponsePayload {
                status: 200,
                message: "Note updated".to_string(),
                data: Some(types::Note::from_store(note.note)),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                status: 500,
                message: err.to_string(),
                data: None,
            }),
        ),
    }
}

async fn get_metas(
    State(app): State<AppState>,
    claims: Claims,
) -> (StatusCode, Json<ResponsePayload<Vec<types::NoteMeta>>>) {
    let result = store::notes::get_notes_by_user_id(claims.user_id.parse().unwrap(), &app.db).await;

    match result {
        Ok(metas) => (
            StatusCode::OK,
            Json(ResponsePayload {
                status: 200,
                message: "Note metas found".to_string(),
                data: Some(
                    metas
                        .iter()
                        .map(|note| types::NoteMeta::from_store(note.clone()))
                        .collect(),
                ),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                status: 500,
                message: err.to_string(),
                data: None,
            }),
        ),
    }
}
