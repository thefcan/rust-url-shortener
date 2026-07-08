# rust-url-shortener

![CI](https://github.com/thefcan/rust-url-shortener/actions/workflows/ci.yml/badge.svg)

A compact, production-oriented URL shortener implemented in Rust using
Axum and Tokio. It emphasizes strong types for errors, structured logging,
testable router construction, and a small, efficient Docker image for
deployment.

## Features
- Shorten URLs and perform permanent redirects with hit counting
- Typed application errors and JSON error responses
- Pluggable `Store` trait (in-memory implementation included)
- Structured request logging and graceful shutdown
- CI with tests, clippy, and formatting checks
- Docker multi-stage build for small runtime image

## API
Method | Path | Description
---|---|---
POST `/shorten` | Accepts JSON `{ "url": "https://..." }` and returns `{ code, short_url }`. Returns 400 for invalid URLs.
GET `/{code}` | Issues a 308 redirect to the original URL and increments the hit counter.
GET `/api/{code}` | Returns link metadata and hit count as JSON.

Error responses use JSON like `{ "error": "..." }` with appropriate HTTP
status codes. Errors are implemented via a typed `AppError` mapped to
responses.

## Quick start
Run locally with Cargo:

```bash
cargo run
# Server listens on http://0.0.0.0:8080 by default

# Example: shorten a URL
curl -s -X POST localhost:8080/shorten -H 'Content-Type: application/json' \
  -d '{"url":"https://github.com/thefcan"}'

# Redirect
curl -si localhost:8080/<code>

# Fetch metadata
curl -s localhost:8080/api/<code>
```

Run with more verbose logs:

```bash
RUST_LOG=debug cargo run
```

## Docker
Build and run the production image:

```bash
docker build -t rust-url-shortener .
docker run --rm -p 8080:8080 rust-url-shortener
# or: docker compose up --build
```

## Tests, linting & formatting

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

CI executes the above checks and a release build on each push.

## Project layout

```
src/
├── main.rs     # server bootstrap: tracing, TraceLayer, graceful shutdown
├── lib.rs      # router construction (easy to test)
├── routes.rs   # request handlers
├── store.rs    # Store trait and in-memory store
├── models.rs   # serde types for requests/responses
└── error.rs    # AppError -> HTTP responses
tests/api.rs    # integration tests
```

## Contributing
Contributions are welcome. Please open issues or PRs, keep changes small,
and ensure tests, clippy and formatting checks pass.

## License
This project is provided under an open-source license. See the `LICENSE`
file for details.
