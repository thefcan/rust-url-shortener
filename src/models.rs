//! Request and response payloads (JSON via serde).
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub code: String,
    pub short_url: String,
}
