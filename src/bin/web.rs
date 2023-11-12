use std::env;
use std::path::PathBuf;

use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = env::var("ASSET_DIR")
        .map(|v| PathBuf::from(v))
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"));
    let addr = env::var("ADDR").unwrap_or("127.0.0.1:0".to_owned());

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/solve", get(solve))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn solve() -> impl IntoResponse {
    Json("hi")
}
