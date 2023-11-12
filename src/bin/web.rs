use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use boggle_ocr::dictionary;
use boggle_ocr::wordsearch;

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
    let dict = env::var("DICT")
        .map(|v| PathBuf::from(v))
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cached.dict"));
    let defs = env::var("DEFS")
        .map(|v| PathBuf::from(v))
        .unwrap_or(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("DICT.json"));

    let dict = dictionary::read(&dict).unwrap();
    let defs = dictionary::open_defs_path(&defs).unwrap();

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/boggle/solver/solution", get(solve_boggle))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(Data {
            dict: dict.into(),
            defs: defs.into(),
        });

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct Data {
    dict: Arc<dictionary::Dictionary>,
    defs: Arc<dictionary::Definitions>,
}

#[derive(Deserialize)]
struct SolveBoggleRequest {
    board: String,
    best_words_count: Option<usize>,
}

#[derive(Serialize)]
struct SolveBoggleResponse {
    total_words: usize,
    total_score: u32,
    best_words: Vec<ScoredBoggleWord>,
}

#[derive(Serialize)]
struct ScoredBoggleWord {
    word: String,
    score: u32,
    def: Option<String>,
}

async fn solve_boggle(
    Query(query): Query<SolveBoggleRequest>,
    State(data): State<Data>,
) -> impl IntoResponse {
    let SolveBoggleRequest {
        board,
        best_words_count,
    } = query;

    let board = match wordsearch::boggled(board.trim()) {
        Ok(b) => b,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let words = wordsearch::find_words(&data.dict, &board);

    let total_score = words.iter().map(|w| wordsearch::score(w)).sum();

    let mut scored_words: Vec<(u32, &String)> =
        words.iter().map(|w| (wordsearch::score(w), w)).collect();
    scored_words.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());

    let best_words = scored_words
        .into_iter()
        .take(best_words_count.unwrap_or(20))
        .map(|(score, word)| ScoredBoggleWord {
            word: word.clone(),
            score,
            def: data.defs.get(word).cloned(),
        })
        .collect();

    Json(SolveBoggleResponse {
        total_words: words.len(),
        total_score,
        best_words,
    })
    .into_response()
}
