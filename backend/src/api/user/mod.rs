use axum::{http::Method, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::AppState;

mod user;

pub fn register() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(user::login))
        .route("/registry", post(user::register))
        .route("/test", axum::routing::get(test))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
                .allow_headers(Any),
        )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
}

async fn test() -> Json<User> {
    Json(User {
        name: "test".to_string(),
        email: "".to_string(),
    })
}
