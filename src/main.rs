use std::{
    net::SocketAddr,
    sync::{atomic::AtomicUsize, Arc},
};
use axum::{
    extract::{Extension},
    routing::get,
    AddExtensionLayer,
    Router,
};
use hyper::{HeaderMap, Server, StatusCode};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let app = Router::new()
        .route("/", get(home))
        .layer(AddExtensionLayer::new(AppState::default()));

    let server = Server::bind(&addr).serve(app.into_make_service());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[derive(Clone, Debug, Default)]
struct AppState {
    counter: Arc<AtomicUsize>,
}

async fn home(
    Extension(state): Extension<AppState>
) -> (StatusCode, HeaderMap, String) {
    let counter = state
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/plain; charset=utf-8".parse().unwrap());
    let body = format!("Counter is at: {}", counter);
    (StatusCode::OK, headers, body)
}
