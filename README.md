# rust-url-shortener

A small, production-style **URL shortener** built in **Rust** with **Axum** —
async (Tokio), typed errors, a Store trait, tests, Docker and CI.

## Status (built in phases)
- [x] **Phase 1 — Shorten & redirect** (in-memory Store)
- [ ] Phase 2 — Typed errors, validation, stats
- [ ] Phase 3 — Tests, clippy & rustfmt clean
- [ ] Phase 4 — Tracing + graceful shutdown
- [ ] Phase 5 — Docker + CI

## API
- `POST /shorten` — body `{"url":"https://..."}` → `{"code":"...","short_url":"/..."}`
- `GET /{code}` — 301 redirect to the original URL

## Run
```bash
cargo run
# listening on http://0.0.0.0:8080
```
