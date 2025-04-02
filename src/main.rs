mod user_info;

use actix_web::{App, HttpResponse, HttpServer, Responder, post};
use std::io::Result;

#[post("/login")]
async fn login(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/register")]
async fn register(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/images")]
async fn images(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(login).service(images).service(register))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
