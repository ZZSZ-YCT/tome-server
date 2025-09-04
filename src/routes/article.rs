use actix_web::web;
use crate::handlers::article::*;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_articles)
        .service(create_article)
        .service(get_article)
        .service(put_article)
        .service(patch_article)
        .service(delete_article);
}
