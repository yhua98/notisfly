use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{
    api::{auth::Claims, types::ResponsePayload},
    store, AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    id: String,
    title: String,
    tags: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Meta {
    pub fn from_store(store: store::shortnotes::meta::Meta) -> Self {
        Self {
            id: store.snid,
            title: store.title,
            tags: store.tags.to_vec(),
            created_at: store.created_at,
        }
    }
}

pub async fn create_meta(
    State(app_state): State<AppState>,
    claims: Claims,
) -> (StatusCode, Json<ResponsePayload<Meta>>) {
    let user_id = claims.user_id;
    println!("user: {}, create meta", user_id);
    let result = store::shortnotes::meta::create_meta(
        user_id.parse().unwrap(),
        store::shortnotes::meta::MetaCreate {
            snid: nanoid::nanoid!(16),
            title: "New Meta".to_string(),
            tags: sqlx::types::Json(vec![]),
        },
        &app_state.db,
    )
    .await;
    match result {
        Ok(meta) => (
            StatusCode::CREATED,
            Json(ResponsePayload {
                status: 200,
                message: "success".to_string(),
                data: Some(Meta::from_store(meta)),
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

pub async fn get_metas(
    State(app_state): State<AppState>,
    claims: Claims,
) -> (StatusCode, Json<ResponsePayload<Vec<Meta>>>) {
    let result =
        store::shortnotes::meta::get_metas(claims.user_id.parse().unwrap(), &app_state.db).await;
    match result {
        Ok(metas) => (
            StatusCode::OK,
            Json(ResponsePayload {
                status: 200,
                message: "success".to_string(),
                data: Some(
                    metas
                        .into_iter()
                        .map(|meta| Meta::from_store(meta))
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
