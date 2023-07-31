use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[macro_use]
extern crate diesel;

pub mod db;
pub mod schema;
pub mod models;
pub mod models_dao;
pub mod primitives;

use models::*;


struct AppState;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/pardal")]
async fn pardal() -> impl Responder {
    HttpResponse::Ok().body("🐦 piu piu")
}

#[post("/users")]
async fn create_user(user: web::Json<CardEntity>, _data: web::Data<AppState>) -> impl Responder {
    let user = user.into_inner();
    HttpResponse::Created().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(pardal)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}