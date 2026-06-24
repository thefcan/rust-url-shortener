//! Integration tests: drive the Axum router in-process (no socket) via
//! tower's `oneshot`, exercising the full request/response path.
use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Router,
};
use http_body_util::BodyExt;
use tower::ServiceExt;

use rust_url_shortener::{app, store::MemoryStore, store::Store};

fn test_app() -> Router {
    let store: Arc<dyn Store> = Arc::new(MemoryStore::new());
    app(store)
}

fn post_json(uri: &str, body: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

fn get(uri: &str) -> Request<Body> {
    Request::builder().uri(uri).body(Body::empty()).unwrap()
}

// Router::clone shares the same Store (Arc), so state persists across requests.
async fn send(app: &Router, req: Request<Body>) -> Response<Body> {
    app.clone().oneshot(req).await.unwrap()
}

async fn json(resp: Response<Body>) -> serde_json::Value {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn shorten_returns_a_code() {
    let app = test_app();
    let resp = send(
        &app,
        post_json("/shorten", r#"{"url":"https://example.com/"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body = json(resp).await;
    assert!(!body["code"].as_str().unwrap().is_empty());
    assert!(body["short_url"].as_str().unwrap().starts_with('/'));
}

#[tokio::test]
async fn invalid_url_is_rejected() {
    let app = test_app();
    let resp = send(&app, post_json("/shorten", r#"{"url":"not a url"}"#)).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert!(json(resp).await["error"].is_string());
}

#[tokio::test]
async fn redirect_follows_and_counts_hits() {
    let app = test_app();

    let created = json(
        send(
            &app,
            post_json("/shorten", r#"{"url":"https://example.com/"}"#),
        )
        .await,
    )
    .await;
    let code = created["code"].as_str().unwrap().to_owned();

    for _ in 0..2 {
        let resp = send(&app, get(&format!("/{code}"))).await;
        assert_eq!(resp.status(), StatusCode::PERMANENT_REDIRECT); // 308
        assert_eq!(
            resp.headers().get("location").unwrap(),
            "https://example.com/"
        );
    }

    let stats = json(send(&app, get(&format!("/api/{code}"))).await).await;
    assert_eq!(stats["hits"], 2);
    assert_eq!(stats["url"], "https://example.com/");
}

#[tokio::test]
async fn unknown_code_is_not_found() {
    let app = test_app();
    let resp = send(&app, get("/api/does-not-exist")).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
