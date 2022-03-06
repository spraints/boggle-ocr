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
            .route("/", web::get().to(index))
        // TODO - add actix_files
        // TODO - Maybe use actix-web-static-files?
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
    min_length: Option<usize>,
    line1: String,
    line2: String,
    line3: String,
    line4: String,
    line5: String,
}

#[derive(Serialize)]
struct BoggleResponse {
    total_words: usize,
    total_score: u32,
    words: Vec<String>,
}

async fn boggle(st: web::Data<Data>, q: web::Query<BoggleRequest>) -> impl Responder {
    let min_length = q.min_length.unwrap_or(3);
    let lines: Vec<&str> = vec![&q.line1, &q.line2, &q.line3, &q.line4, &q.line5];
    let words = wordsearch::find_boggle_words(&lines, &st.dict, min_length);
    let total_words = words.len();
    let total_score = words.iter().map(|w| w.score).sum();
    let resp = BoggleResponse {
        total_words,
        total_score,
        words: words.into_iter().map(|w| w.word).collect(),
    };
    let body = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"
<!DOCTYPE html>
<html>
  <head>
    <title>Boggle solver</title>
  </head>
  <body>
    <h1>Define a word</h1>
    <form action="/define" method="get" spellcheck="false">
      <input type="text" name="word" placeholder="word">
      <input type="submit" value="Define">
    </form>
    <h1>Find all the words</h1>
    <form action="/boggle" method="get" spellcheck="false">
      Shortest word: <input type="number" name="min_length" value="3"><br>
      <input type="text" name="line1" placeholder="abcde"><br>
      <input type="text" name="line2" placeholder="abcde"><br>
      <input type="text" name="line3" placeholder="abcde"><br>
      <input type="text" name="line4" placeholder="abcde"><br>
      <input type="text" name="line5" placeholder="abcde"><br>
      <input type="submit" value="Solve">
    </form>
  </body>
</html>
"#,
    )
}
