use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::AppState;

mod user;


pub fn register() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(user::login))
        .route("/registry", post(user::register))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
                .allow_headers(Any),
        )
}