mod auth;
mod notes;
mod shortnotes;
mod types;
mod user;

use axum::Router;

use crate::AppState;

pub fn register() -> Router<AppState> {
    Router::new()
        .nest("/shortnote", shortnotes::register())
        .nest("/user", user::register())
        .nest("/notes", notes::register())
}
