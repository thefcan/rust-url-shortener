//! HTTP handlers for the shortener.
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use nanoid::nanoid;

use crate::models::{ShortenRequest, ShortenResponse};
use crate::store::Store;

/// Shared application state: any Store implementation behind an Arc.
pub type AppState = Arc<dyn Store>;

/// POST /shorten — create a short code for a URL.
pub async fn shorten(
    State(store): State<AppState>,
    Json(req): Json<ShortenRequest>,
) -> impl IntoResponse {
    let code = nanoid!(7);
    store.save(code.clone(), req.url);
    let short_url = format!("/{code}");
    (
        StatusCode::CREATED,
        Json(ShortenResponse { code, short_url }),
    )
}

/// GET /{code} — permanently redirect to the original URL.
pub async fn redirect(State(store): State<AppState>, Path(code): Path<String>) -> Response {
    match store.resolve(&code) {
        Some(url) => Redirect::permanent(&url).into_response(),
        None => (StatusCode::NOT_FOUND, "unknown short code\n").into_response(),
    }
}
