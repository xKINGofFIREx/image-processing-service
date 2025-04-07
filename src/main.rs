mod image_transform;
mod user_info;

use actix_web::{App, HttpResponse, HttpServer, Responder, http::header::ContentType, post, web};
use image_transform::Transformation;
use std::io::Result;
use user_info::User;

#[post("/login")]
async fn login(req: web::Json<User>) -> impl Responder {
    let _user = User {
        username: String::from(&req.username),
        password: String::from(&req.password),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("Login sucessful")
}

#[post("/register")]
async fn register(req: web::Json<User>) -> impl Responder {
    let _new_user = User {
        username: String::from(&req.username),
        password: String::from(&req.password),
    };

    HttpResponse::Created()
        .content_type(ContentType::json())
        .body("Sucessfully registered")
}

#[post("/images")]
async fn images(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/images/:id/transform")]
async fn transform_image(req: web::Json<Transformation>) -> impl Responder {
    let transformation: Transformation = req.0;
    HttpResponse::Ok().body(transformation.to_string())
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(login).service(images).service(register))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
