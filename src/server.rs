use crate::dictionary::{Definitions, Dictionary};
use crate::wordsearch;
use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::net::ToSocketAddrs;

pub fn serve<A: ToSocketAddrs + Display>(
    addr: A,
    dict: Dictionary,
    defs: Definitions,
) -> Result<(), Box<dyn Error>> {
    println!("serving on {}", addr);
    async_serve(addr, Data { dict, defs })
}

struct Data {
    dict: Dictionary,
    defs: Definitions,
}

#[actix_web::main]
async fn async_serve<A: ToSocketAddrs>(addr: A, data: Data) -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let st = web::Data::new(data);
    HttpServer::new(move || {
        App::new()
            .app_data(st.clone())
            .wrap(Logger::default())
            .route("/define", web::get().to(lookup_word))
            .route("/boggle", web::get().to(boggle))
    })
    .bind(addr)?
    .run()
    .await?;
    Ok(())
}

#[derive(Deserialize)]
struct LookupWordRequest {
    word: String,
}

#[derive(Serialize)]
struct LookupWordResponse {
    definitions: Vec<String>,
}

async fn lookup_word(st: web::Data<Data>, q: web::Query<LookupWordRequest>) -> impl Responder {
    let definitions = match st.defs.get(&q.word) {
        Some(d) => d.clone(),
        None => vec![],
    };
    let body = serde_json::to_string(&LookupWordResponse { definitions }).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[derive(Deserialize)]
struct BoggleRequest {
    lines: String, // comma-separated
}

#[derive(Serialize)]
struct BoggleResponse {
    total_words: usize,
    total_score: u32,
}

async fn boggle(st: web::Data<Data>, q: web::Query<BoggleRequest>) -> impl Responder {
    let lines: Vec<&str> = q.lines.split(",").collect();
    let words = wordsearch::find_boggle_words(&lines, &st.dict);
    let total_words = words.len();
    let total_score = words.iter().map(|w| w.score).sum();
    let resp = BoggleResponse {
        total_words,
        total_score,
    };
    let body = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}
