use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::{auth::Claims, types::ResponsePayload},
    store, AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryNote {
    pub id: String,
    pub title: String,
    pub content: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i32,
    pub dnid: String,
}

impl DiaryNote {
    pub fn from_store(store: store::shortnotes::diary::DiaryNote) -> Self {
        Self {
            id: store.dnid.clone(),
            title: store.title,
            content: store.content,
            created_at: store.created_at,
            updated_at: store.updated_at,
            user_id: store.user_id,
            dnid: store.dnid,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDiaryNoteCreate {
    pub dnid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDiaryNote {
    pub dnid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDiaryNoteUpdate {
    pub content: Option<Vec<u8>>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub async fn create_diarynote(
    State(app_state): State<AppState>,
    claims: Claims,
    extract::Json(payload): extract::Json<RequestDiaryNoteCreate>,
) -> (StatusCode, Json<ResponsePayload<DiaryNote>>) {
    let user_id: i32 = claims.user_id.parse().unwrap();
    let result =
        store::shortnotes::diary::create_diarynote(payload.dnid, user_id, &app_state.db).await;
    match result {
        Ok(diarynote) => (
            StatusCode::CREATED,
            Json(ResponsePayload {
                status: 200,
                message: "success".to_string(),
                data: Some(DiaryNote::from_store(diarynote)),
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

pub async fn update_diary(
    State(app_state): State<AppState>,
    _claims: Claims,
    Path(dnid): Path<String>,
    extract::Json(payload): extract::Json<RequestDiaryNoteUpdate>,
) -> (StatusCode, Json<ResponsePayload<DiaryNote>>) {
    let result = store::shortnotes::diary::update_diarynote_by_dnid(
        dnid.clone(),
        store::shortnotes::diary::DiaryNoteUpdate {
            title: payload.title,
            content: payload.content,
            tags: payload.tags.map(|tags| sqlx::types::Json(tags)),
        },
        &app_state.db,
    )
    .await;
    match result {
        Ok(diarynote) => {
            let note = DiaryNote::from_store(diarynote);
            (
                StatusCode::OK,
                Json(ResponsePayload {
                    data: Some(note),
                    message: "Diary updated".to_string(),
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

pub async fn get_or_create_diarynote_by_date(
    State(app_state): State<AppState>,
    Path(dnid): Path<String>,
    claims: Claims,
) -> (StatusCode, Json<ResponsePayload<DiaryNote>>) {
    let user_id: i32 = claims.user_id.parse().unwrap();
    let result =
        store::shortnotes::diary::get_diarynote_by_date(user_id, dnid.clone(), &app_state.db).await;
    match result {
        Ok(diarynote) => (
            StatusCode::OK,
            Json(ResponsePayload {
                status: 200,
                message: "success".to_string(),
                data: Some(DiaryNote::from_store(diarynote)),
            }),
        ),
        Err(err) => match err.downcast_ref::<sqlx::Error>() {
            Some(sqlx::Error::RowNotFound) => {
                let result =
                    store::shortnotes::diary::create_diarynote(dnid, user_id, &app_state.db).await;
                match result {
                    Ok(diarynote) => (
                        StatusCode::CREATED,
                        Json(ResponsePayload {
                            status: 200,
                            message: "success".to_string(),
                            data: Some(DiaryNote::from_store(diarynote)),
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
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponsePayload {
                    status: 500,
                    message: err.to_string(),
                    data: None,
                }),
            ),
        },
    }
}
