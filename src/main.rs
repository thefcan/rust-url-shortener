use std::sync::Arc;

use rust_url_shortener::{app, store::MemoryStore};

#[tokio::main]
async fn main() {
    let state: Arc<dyn rust_url_shortener::store::Store> = Arc::new(MemoryStore::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("bind to :8080");
    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app(state)).await.expect("serve");
}
