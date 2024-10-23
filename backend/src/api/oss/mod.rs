use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};

use tower_http::cors::{Any, CorsLayer};

use crate::{store, AppState};

use super::{auth::Claims, types::ResponsePayload};

pub fn register() -> Router<AppState> {
    Router::new()
        .route("/", post(create_file))
        .route("/:key", get(get_oss))
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OssFile {
    key: String,
    data: Vec<u8>,
}

impl OssFile {
    pub fn from_store_oss(oss_file: store::oss::OssFile) -> Self {
        OssFile {
            key: oss_file.key,
            data: oss_file.data.bytes,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OssFileCreate {
    key: String,
    data: Vec<u8>,
}

async fn create_file(
    State(_app): State<AppState>,
    _claims: Claims,
    mut multipart: axum::extract::Multipart,
) -> (StatusCode, Json<ResponsePayload<()>>) {
    let mut oss_file = store::oss::OssFile {
        id: bson::oid::ObjectId::new().to_hex(),
        key: "".to_string(),
        size: 0,
        _type: "".to_string(),
        data: mongodb::bson::Binary {
            subtype: mongodb::bson::spec::BinarySubtype::Generic,
            bytes: vec![],
        },
    };
    let mut count = 0;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "key" => {
                oss_file.key = String::from_utf8(data.to_vec()).unwrap();
                count += 1;
            }
            "file" => {
                oss_file.data.bytes = data.to_vec();
                count += 1;
            }
            _ => {
                println!("Unknown field: {}", name);
            }
        }
    }

    if count == 2 {
        store::oss::create_file(oss_file, &_app.mongo)
            .await
            .unwrap();
        return (
            StatusCode::CREATED,
            Json(ResponsePayload {
                data: None,
                message: "File created".to_string(),
                status: 2000,
            }),
        );
    }

    (
        StatusCode::BAD_REQUEST,
        Json(ResponsePayload {
            data: None,
            message: "Invalid request".to_string(),
            status: 4000,
        }),
    )
}

async fn get_oss(
    State(app): State<AppState>,
    _claims: Claims,
    Path(key): Path<String>,
) -> (StatusCode, Json<ResponsePayload<OssFile>>) {
    let file = store::oss::get_file_by_key(key, &app.mongo).await;
    match file {
        Ok(file) => (
            StatusCode::OK,
            Json(ResponsePayload {
                data: Some(OssFile::from_store_oss(file)),
                message: "File found".to_string(),
                status: 2000,
            }),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ResponsePayload {
                data: None,
                message: "Not found".to_string(),
                status: 4004,
            }),
        ),
    }
}
