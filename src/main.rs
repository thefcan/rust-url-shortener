use std::sync::Arc;

use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use rust_url_shortener::{app, store::MemoryStore, store::Store};

#[tokio::main]
async fn main() {
    // Structured logging; level configurable via RUST_LOG.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let state: Arc<dyn Store> = Arc::new(MemoryStore::new());
    let router = app(state).layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("bind to :8080");
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("serve");

    tracing::info!("server stopped");
}

/// Completes when the process receives Ctrl-C or (on Unix) SIGTERM.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received, draining connections");
}
