pub mod root;
pub mod article;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    root::init(cfg);
    article::init(cfg);
}