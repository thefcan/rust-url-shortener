//! A small, production-style URL shortener built with Axum.
pub mod models;
pub mod routes;
pub mod store;

use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{redirect, shorten, AppState};

/// Build the application router over any Store implementation. Kept separate
/// from `main` so integration tests can exercise it without binding a socket.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/shorten", post(shorten))
        .route("/{code}", get(redirect))
        .with_state(state)
}
