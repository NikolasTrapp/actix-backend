use actix_web::web;

use crate::primitives;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(primitives::hello);
    cfg.service(primitives::pardal);
}