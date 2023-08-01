#[macro_use]
extern crate diesel;
use actix_web::{ App, HttpServer };

pub mod db;
pub mod schema;
pub mod models;
pub mod services;
pub mod primitives;
pub mod routes;

const SERVER_ADDRESS: &'static str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::config))
        .bind(SERVER_ADDRESS)?
        .workers(1)
        .run()
        .await
}