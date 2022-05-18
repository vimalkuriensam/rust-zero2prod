use std::io::Result;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello World from Vimal")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
