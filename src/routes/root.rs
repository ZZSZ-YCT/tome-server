use actix_web::{web};
use crate::handlers;

pub fn init(cfg : &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(handlers::root::health_check))
    );
}