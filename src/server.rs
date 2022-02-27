use crate::dictionary::{Definitions, Dictionary};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
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

async fn lookup_word(st: web::Data<Data>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hey there! {:?}", st.defs.get("oversize")))
}
