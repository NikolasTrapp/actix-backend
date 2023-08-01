use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::*;

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