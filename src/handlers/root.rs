use actix_web::{get, HttpResponse, Responder};

use crate::models;
use crate::config;

#[get("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(models::health_check::HealthCheckResult {
        status: String::from("ok"),
        message: String::from("API is running."),
        version: config::VERSION.to_string(),
    })
}
