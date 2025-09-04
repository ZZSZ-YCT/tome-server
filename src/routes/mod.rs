pub mod root;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    root::init(cfg);
}