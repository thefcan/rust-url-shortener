//! HTTP handlers for the shortener.
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use nanoid::nanoid;

use crate::error::AppError;
use crate::models::{ShortenRequest, ShortenResponse, StatsResponse};
use crate::store::Store;

/// Shared application state: any Store implementation behind an Arc.
pub type AppState = Arc<dyn Store>;

/// POST /shorten — validate a URL and mint a short code for it.
pub async fn shorten(
    State(store): State<AppState>,
    Json(req): Json<ShortenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let url = validate_url(&req.url)?;
    let code = nanoid!(7);
    store.save(code.clone(), url);
    let short_url = format!("/{code}");
    Ok((
        StatusCode::CREATED,
        Json(ShortenResponse { code, short_url }),
    ))
}

/// GET /{code} — permanently redirect to the original URL (counts a hit).
pub async fn redirect(
    State(store): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, AppError> {
    store
        .resolve(&code)
        .map(|url| Redirect::permanent(&url))
        .ok_or(AppError::NotFound)
}

/// GET /api/{code} — link metadata and hit count (does not count a hit).
pub async fn stats(
    State(store): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<StatsResponse>, AppError> {
    let link = store.get(&code).ok_or(AppError::NotFound)?;
    Ok(Json(StatsResponse {
        code,
        url: link.url,
        hits: link.hits,
    }))
}

/// validate_url accepts only well-formed http/https URLs, returning the
/// canonical form.
fn validate_url(raw: &str) -> Result<String, AppError> {
    let parsed = url::Url::parse(raw).map_err(|e| AppError::InvalidUrl(e.to_string()))?;
    match parsed.scheme() {
        "http" | "https" => Ok(parsed.to_string()),
        other => Err(AppError::InvalidUrl(format!(
            "unsupported scheme '{other}'"
        ))),
    }
}
