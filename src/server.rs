use crate::dictionary::{Definitions, Dictionary};
use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;
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
            .route("/word", web::get().to(lookup_word))
    })
    .bind(addr)?
    .run()
    .await?;
    Ok(())
}

#[derive(Serialize)]
struct LookupWordResult {
    definitions: Vec<String>,
}

async fn lookup_word(st: web::Data<Data>, req: HttpRequest) -> impl Responder {
    let word = "whatever";
    let definitions = match st.defs.get(word) {
        Some(d) => d.clone(),
        None => vec![],
    };
    let body = serde_json::to_string(&LookupWordResult { definitions }).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}
