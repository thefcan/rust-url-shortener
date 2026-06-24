# rust-url-shortener

A small, production-style **URL shortener** built in **Rust** with **Axum** —
async (Tokio), typed errors, a Store trait, tests, Docker and CI.

## Status (built in phases)
- [x] **Phase 1 — Shorten & redirect** (in-memory Store)
- [x] **Phase 2 — Typed errors, URL validation, stats**
- [ ] Phase 3 — Tests, clippy & rustfmt clean
- [ ] Phase 4 — Tracing + graceful shutdown
- [ ] Phase 5 — Docker + CI

## API
| Method | Path          | Description                                  |
|--------|---------------|----------------------------------------------|
| POST   | `/shorten`    | Body `{"url":"https://..."}` → `{code, short_url}`; rejects invalid URLs (400) |
| GET    | `/{code}`     | 308 redirect to the original URL (counts a hit) |
| GET    | `/api/{code}` | Link metadata + hit count (JSON)             |

Errors are returned as JSON (`{"error":"..."}`) with the right status code via a
typed `AppError` (`thiserror` → `IntoResponse`). URLs are validated with the
`url` crate (only `http`/`https`).

## Run
```bash
cargo run
# listening on http://0.0.0.0:8080

curl -s -X POST localhost:8080/shorten -H 'Content-Type: application/json' \
  -d '{"url":"https://github.com/thefcan"}'         # -> {"code":"...","short_url":"/..."}
curl -si localhost:8080/<code>                       # -> 308 redirect
curl -s  localhost:8080/api/<code>                   # -> {"code","url","hits"}
```

## Architecture
```
src/
├── main.rs     # binds the socket, runs the server
├── lib.rs      # builds the Axum router (testable)
├── routes.rs   # handlers: shorten / redirect / stats
├── store.rs    # Store trait + in-memory implementation
├── models.rs   # serde request/response types
└── error.rs    # typed AppError -> HTTP response
```
