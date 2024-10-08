use crate::{
    api::{auth::Claims, types::ResponsePayload},
    store::{
        self,
        shortnotes::{
            self,
            note::{ShortNote, ShortNoteCreate},
        },
    },
    AppState,
};
use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

// note type
#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub uid: String,
    pub title: String,
    pub content: Vec<u8>,
    pub tags: Vec<String>,
}

// create note type
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestNoteCreate {
    pub title: String,
    pub content: Vec<u8>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestNoteUpdate {
    pub title: Option<String>,
    pub content: Option<Vec<u8>>,
    pub tags: Option<Vec<String>>,
}

pub async fn save_note(
    State(app_state): State<AppState>,
    extract::Json(note_create): extract::Json<RequestNoteCreate>,
) -> (StatusCode, Json<ResponsePayload<Note>>) {
    let result = shortnotes::note::create_shortnote(
        "".to_string(),
        ShortNoteCreate {
            snid: nanoid::nanoid!(16),
            title: note_create.title.clone(),
            content: note_create.content.clone(),
            tags: sqlx::types::Json(note_create.tags.clone()),
        },
        &app_state.db,
    )
    .await;

    if let Ok(short_note) = result {
        let note = short_note_to_note(short_note);
        return (
            StatusCode::CREATED,
            Json(ResponsePayload {
                data: Some(note),
                message: "Note created".to_string(),
                status: 200,
            }),
        );
    }

    println!("Error creating note: {:?}", result);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ResponsePayload {
            data: None,
            message: "Error creating note".to_string(),
            status: 500,
        }),
    )
}

pub async fn get_note(
    State(app_state): State<AppState>,
    Path(uid): Path<String>,
) -> (StatusCode, Json<ResponsePayload<Note>>) {
    let result = shortnotes::note::get_shortnote(uid, &app_state.db).await;

    match result {
        Ok(short_note) => {
            let note = short_note_to_note(short_note);
            (
                StatusCode::OK,
                Json(ResponsePayload {
                    data: Some(note),
                    message: "Note found".to_string(),
                    status: 200,
                }),
            )
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                data: None,
                message: err.to_string(),
                status: 500,
            }),
        ),
    }
}

pub async fn delete_note() -> impl IntoResponse {}

pub async fn update_note(
    State(app_state): State<AppState>,
    _claims: Claims,
    Path(uid): Path<String>,
    extract::Json(note_create): extract::Json<RequestNoteUpdate>,
) -> (StatusCode, Json<ResponsePayload<Note>>) {
    let result = store::shortnotes::note::update_shortnote(
        uid,
        store::shortnotes::note::ShortNoteUpdate {
            title: note_create.title,
            content: note_create.content,
            tags: note_create.tags.map(|tags| sqlx::types::Json(tags)),
        },
        &app_state.db,
    )
    .await;

    match result {
        Ok(short_note) => {
            let note = short_note_to_note(short_note);
            (
                StatusCode::OK,
                Json(ResponsePayload {
                    data: Some(note),
                    message: "Note updated".to_string(),
                    status: 200,
                }),
            )
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                data: None,
                message: err.to_string(),
                status: 500,
            }),
        ),
    }
}

pub async fn get_notes(
    State(app_state): State<AppState>,
) -> (StatusCode, Json<ResponsePayload<Vec<Note>>>) {
    let result = shortnotes::note::get_all_shortnotes("".to_string(), &app_state.db).await;

    if let Ok(short_notes) = result {
        let notes = short_notes
            .into_iter()
            .map(short_note_to_note)
            .collect::<Vec<Note>>();
        return (
            StatusCode::OK,
            Json(ResponsePayload {
                data: Some(notes),
                message: "Notes found".to_string(),
                status: 200,
            }),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ResponsePayload {
            data: None,
            message: "Error getting notes".to_string(),
            status: 500,
        }),
    )
}

pub async fn get_notes_by_tag() -> impl IntoResponse {}

fn short_note_to_note(short_note: ShortNote) -> Note {
    Note {
        uid: short_note.snid,
        title: short_note.title,
        content: short_note.content,
        tags: short_note.tags.to_vec(),
    }
}
