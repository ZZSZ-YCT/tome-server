use actix_web::{web};
use crate::handlers;

pub fn init(cfg : &mut web::ServiceConfig) {
    cfg.service(handlers::root::health_check);
}