mod types;

use axum::{
    extract::{self, Path, State},
    http::{Method, StatusCode},
    routing::{get, patch, post},
    Json, Router,
};
use futures::StreamExt;
use mongodb::bson::doc;
use serde_json::Value;
use tower_http::cors::{Any, CorsLayer};
use types::{Block, Doc, Meta, TodoDoc};

use crate::{store, AppState};

use super::{auth::Claims, types::ResponsePayload};

pub fn register() -> Router<AppState> {
    Router::new()
        .route("/", post(create_note))
        .route("/", patch(update_note))
        .route("/:id", get(get_note))
        .route("/todos", get(get_todos))
        .route("/meta", get(get_notes_meta))
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

async fn create_note(
    State(app): State<AppState>,
    _claims: Claims,
    extract::Json(doc): extract::Json<types::DocCreate>,
) -> (StatusCode, Json<ResponsePayload<String>>) {
    let top_block_id = doc.blocks.id.clone();
    let result = store::mongo::create_note(
        store::mongo::types::Doc {
            id: bson::oid::ObjectId::new().to_hex(),
            _type: doc._type,
            meta: doc.meta.to_store_meta(),
            blocks: doc.blocks.to_store_blocks(doc.meta.id.clone()),
            top_block_id: top_block_id,
        },
        &app.mongo,
    )
    .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(ResponsePayload {
                data: None,
                message: "Success".to_string(),
                status: 2000,
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                data: None,
                message: "Failed to save note".to_string(),
                status: 500,
            }),
        ),
    }
}

async fn update_note(
    State(app): State<AppState>,
    _claims: Claims,
    extract::Json(doc): extract::Json<types::DocCreate>,
) -> (StatusCode, Json<ResponsePayload<String>>) {
    let top_block_id = doc.blocks.id.clone();
    let result = store::mongo::update_note(
        doc.meta.id.clone(),
        store::mongo::types::Doc {
            id: bson::oid::ObjectId::new().to_hex(),
            _type: doc._type,
            meta: doc.meta.to_store_meta(),
            blocks: doc.blocks.to_store_blocks(doc.meta.id.clone()),
            top_block_id: top_block_id,
        },
        &app.mongo,
    )
    .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(ResponsePayload {
                data: None,
                message: "Update success".to_string(),
                status: 2000,
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponsePayload {
                data: None,
                message: "Failed to save note".to_string(),
                status: 500,
            }),
        ),
    }
}

async fn get_note(
    State(app): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ResponsePayload<Doc>>) {
    let id_clone = id.clone();
    let result = store::mongo::get_note_by_id(id_clone, &app.mongo).await;

    let doc = match result {
        Ok(doc) => {
            let top_id = doc.top_block_id.clone();
            let _doc = Doc {
                id: Some(doc.id),
                _type: doc._type.clone(),
                meta: Meta::from_store_meta(doc.meta),
                blocks: Block::from_store_blocks(top_id, doc.blocks),
            };
            ResponsePayload {
                data: Some(_doc),
                message: "Success".to_string(),
                status: 2000,
            }
        }
        Err(err) => ResponsePayload {
            data: None,
            message: err.to_string(),
            status: 500,
        },
    };

    (StatusCode::OK, Json(doc))
}

async fn get_todos(State(app): State<AppState>) -> (StatusCode, Json<ResponsePayload<Value>>) {
    let todos = app
        .mongo
        .database("notes")
        .collection::<store::mongo::types::Doc>("note");

    let result = todos
        .aggregate([
            doc! {
                "$unwind": "$blocks"
            },
            doc! {
                "$match":doc! {
                    "blocks.props.type": "todo"
                }
            },
        ])
        .with_type::<TodoDoc>()
        .await;

    match result {
        Ok(cursor) => {
            let mut todos = vec![];
            let mut cursor = cursor;
            while let Some(_todo) = cursor.next().await {
                if let Ok(todo) = _todo {
                    todos.push(todo);
                }
            }
            (
                StatusCode::OK,
                Json(ResponsePayload {
                    data: Some(serde_json::to_value(todos).unwrap()),
                    message: "Success".to_string(),
                    status: 2000,
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

async fn get_notes_meta(
    State(app): State<AppState>,
    // _claims: Claims,
) -> (StatusCode, Json<ResponsePayload<Vec<Meta>>>) {
    let result = store::mongo::get_notes_meta(&app.mongo).await;

    match result {
        Ok(meta) => {
            let meta = meta.into_iter().map(Meta::from_store_meta).collect();
            (
                StatusCode::OK,
                Json(ResponsePayload {
                    data: Some(meta),
                    message: "Success".to_string(),
                    status: 2000,
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
