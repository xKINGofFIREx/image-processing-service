mod image_transform;

use actix_web::cookie::{Cookie, SameSite};
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use image_transform::Transformation;
use std::collections::HashMap;
use std::io::Result;
use std::sync::{Arc, Mutex};
use serde::Serialize;

struct AppData {
    users: Arc<Mutex<HashMap<String, String>>>,
    sessions: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Serialize)]
struct ImageResponse {
    url: String, 
    file_name: String, 
    file_size: usize,
    content_type: String,
}

#[post("/login")]
async fn login(
    data: web::Data<AppData>,
    form: web::Form<HashMap<String, String>>,
) -> impl Responder {
    let username = form.get("username").unwrap();
    let password = form.get("password").unwrap();

    let users = data.users.lock().unwrap();

    match users.get(username) {
        Some(stored_password) if stored_password == password => {
            let mut sessions = data.sessions.lock().unwrap();
            let session_id = uuid::Uuid::new_v4().to_string();

            sessions.insert(session_id.clone(), username.clone());

            let cookie = Cookie::build("session_id", session_id)
                .same_site(SameSite::Lax)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .body("Wrong username or password")
        }
        _ => HttpResponse::BadRequest().body("Wrong username or password"),
    }
}

#[post("/register")]
async fn register(
    data: web::Data<AppData>,
    form: web::Form<HashMap<String, String>>,
) -> impl Responder {
    let username = form.get("username").unwrap();
    let password = form.get("password").unwrap();

    let mut users = data.users.lock().unwrap();

    if users.contains_key(username) {
        return HttpResponse::BadRequest().body("This username exists");
    }

    users.insert(username.clone(), password.clone());

    HttpResponse::Ok().body("Successfully registered")
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
    let data = web::Data::new({
        AppData {
            users: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(login)
            .service(images)
            .service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
