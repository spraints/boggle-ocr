use crate::dictionary::Dictionary;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fmt::Display;
use std::net::ToSocketAddrs;

type GenericError = Box<dyn std::error::Error>;

pub fn serve<A: ToSocketAddrs + Display>(addr: A, dict: Dictionary) -> Result<(), GenericError> {
    println!("serving on {}", addr);
    async_serve(addr, dict);
    Ok(())
}

#[actix_web::main]
async fn async_serve<A: ToSocketAddrs>(addr: A, dict: Dictionary) {
    HttpServer::new(|| App::new().route("/word", web::get().to(lookup_word)))
        .bind(addr)
        .unwrap()
        .run()
        .await
        .unwrap();
}

async fn lookup_word() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
