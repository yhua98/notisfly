mod auth;
mod mongo;
mod notes;
mod oss;
mod shortnotes;
mod types;
mod user;

use axum::Router;

use crate::AppState;

pub fn register() -> Router<AppState> {
    Router::new()
        .nest("/v1/shortnote", shortnotes::register())
        .nest("/user", user::register())
        .nest("/v1/notes", notes::register())
        .nest("/notes", mongo::register())
        .nest("/oss", oss::register())
}
