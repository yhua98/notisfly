mod shortnotes;
mod user;
mod auth;
mod types;

use axum::Router;

use crate::AppState;

pub fn register() -> Router<AppState> {
    Router::new()
        .nest("/shortnote", shortnotes::register())
        .nest("/user", user::register())
}