use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use crate::models::card::CardEntity;
use crate::models::table::NewTableEntity;
use crate::services::table_service::setup_table;

#[derive(serde::Deserialize)]
struct PathParams {
    table_id: i64,
    player_id: i64,
}

#[get("/{oii}")]
async fn hello(oii: web::Path<String>) -> impl Responder {
    info!("{}", oii);
    HttpResponse::Ok().body("Hello world!")
}

#[get("/pardal")]
async fn pardal(data: web::Data<crate::utils::AppState>) -> impl Responder {
    HttpResponse::Ok().json(crate::dao::card_dao::update(5, CardEntity::new(4, crate::models::suit::Suit::Diamonds, 999, false, Some(1)), &data.db).await.expect(""))
}

#[post("/michel_telo/{table_id}/{player_id}")]
async fn michel_telo(params: web::Path<PathParams>, data: web::Data<crate::utils::AppState>) -> impl Responder {
    info!("oiiiiiiiiii");
    HttpResponse::Ok().json(setup_table(params.table_id, params.player_id, &data.db).await.expect("msg"))

}

