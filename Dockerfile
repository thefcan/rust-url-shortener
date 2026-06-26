# syntax=docker/dockerfile:1

# ---- build stage ----
FROM rust:1-slim AS build
WORKDIR /src
COPY . .
RUN cargo build --release

# ---- run stage ----
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /src/target/release/rust-url-shortener /app
EXPOSE 8080
USER nonroot:nonroot
ENTRYPOINT ["/app"]
