# rust-url-shortener

![CI](https://github.com/thefcan/rust-url-shortener/actions/workflows/ci.yml/badge.svg)

A small, production-style **URL shortener** built in **Rust** with **Axum** —
async (Tokio), typed errors, structured logging, a Store trait, tests, Docker
and CI.

## Status — all phases complete
- [x] **Phase 1 — Shorten & redirect** (in-memory Store)
- [x] **Phase 2 — Typed errors, URL validation, stats**
- [x] **Phase 3 — Integration tests, clippy & rustfmt clean**
- [x] **Phase 4 — Tracing + graceful shutdown**
- [x] **Phase 5 — Docker + CI**

## API
| Method | Path          | Description                                  |
|--------|---------------|----------------------------------------------|
| POST   | `/shorten`    | Body `{"url":"https://..."}` → `{code, short_url}`; rejects invalid URLs (400) |
| GET    | `/{code}`     | 308 redirect to the original URL (counts a hit) |
| GET    | `/api/{code}` | Link metadata + hit count (JSON)             |

Errors are JSON (`{"error":"..."}`) with the right status via a typed `AppError`
(`thiserror` → `IntoResponse`). URLs are validated with the `url` crate.

## Run
```bash
cargo run                          # listening on http://0.0.0.0:8080
RUST_LOG=debug cargo run           # verbose request logs

curl -s -X POST localhost:8080/shorten -H 'Content-Type: application/json' \
  -d '{"url":"https://github.com/thefcan"}'
curl -si localhost:8080/<code>     # 308 redirect
curl -s  localhost:8080/api/<code> # {"code","url","hits"}
```
Each request is logged with method, path, status and latency (`tower-http`
TraceLayer); the server shuts down gracefully on Ctrl-C / SIGTERM.

## Docker
A multi-stage build produces a small image on a distroless, non-root base:
```bash
docker build -t rust-url-shortener .
docker run --rm -p 8080:8080 rust-url-shortener
# or: docker compose up --build
```

## Tests, lint & format
```bash
cargo test                                    # in-process integration tests (tower oneshot)
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```
CI runs all of the above plus a release build on every push (GitHub Actions).

## Architecture
```
src/
├── main.rs     # server: tracing, TraceLayer, graceful shutdown
├── lib.rs      # builds the Axum router (testable)
├── routes.rs   # handlers: shorten / redirect / stats
├── store.rs    # Store trait + in-memory implementation
├── models.rs   # serde request/response types
└── error.rs    # typed AppError -> HTTP response
tests/api.rs    # integration tests
```
