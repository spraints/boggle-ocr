use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::dictionary::{self, Definitions, Dictionary};
use crate::options::ServerOptions;
use crate::wordsearch;

pub fn serve(opts: ServerOptions) -> Result<(), Box<dyn std::error::Error>> {
    let ServerOptions {
        addr,
        assets,
        dict,
        defs,
    } = opts;

    let addr = addr.unwrap_or("127.0.0.1:0".to_owned());
    let assets = assets.unwrap_or("assets".to_owned());
    let dict = dict.unwrap_or("cached.dict".to_owned());
    let defs = defs.unwrap_or("DICT.json".to_owned());

    let dict = dictionary::read(&dict)?;
    let defs = dictionary::open_defs_path(&defs)?;

    let rt = Runtime::new()?;
    rt.block_on(async move { async_serve(addr, assets, dict, defs).await });
    Ok(())
}

async fn async_serve(addr: String, assets_dir: String, dict: Dictionary, defs: Definitions) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "boggle_ocr=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/boggle/solver/solution", get(solve_boggle))
        .route("/boggle/dict/words/:word", get(boggle_word))
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

async fn boggle_word(Path(word): Path<String>, State(data): State<Data>) -> impl IntoResponse {
    match data.defs.get(&word.to_lowercase()) {
        Some(def) => (StatusCode::OK, def.to_owned()),
        None => (StatusCode::NOT_FOUND, "".to_owned()),
    }
}
