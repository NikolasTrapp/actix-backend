use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::card::CardEntity;
use crate::models::table::NewTableEntity;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/pardal")]
async fn pardal(data: web::Data<crate::utils::AppState>) -> impl Responder {
    HttpResponse::Ok().json(crate::dao::card_dao::update(5, CardEntity::new(4, crate::models::suit::Suit::Diamonds, 999, false, 1), &data.db).await.expect(""))
}

