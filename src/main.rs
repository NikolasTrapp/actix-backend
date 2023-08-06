pub mod models;
pub mod dao;
pub mod services;
pub mod primitives;
pub mod routes;
pub mod utils;

use std::env;

use actix_cors::Cors;
use actix_web::{ http::header, middleware::Logger, App, HttpServer, web };
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;


const SERVER_ADDRESS: &'static str = "127.0.0.1:8080";


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect(&format!("Couldn't connect to database url '{}'", &database_url));

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(crate::utils::AppState { db: db.clone()}))
        .wrap(Cors::default()
                  .allowed_origin("http://localhost:4200")
                  .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                  .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                  .allowed_header(header::CONTENT_TYPE)
                  .supports_credentials()
                  .max_age(3600),
        )
        .wrap(Logger::default())
        .configure(routes::config))
        .bind(SERVER_ADDRESS)?
        .workers(2)
        .run()
        .await
}