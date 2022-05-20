// !lib.rs
use std::io::Error;

use actix_web::{dev::Server, get, App, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<Server, Error> {
    let server: Server = HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8000))?
        .run();
    Ok(server)
}
