use actix_web::{App, HttpServer, middleware, web};
use tome_server::routes;
use tome_server::db;
use tome_server::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let settings = config::load_settings();
    
    let db = db::init_db(settings.database_url).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db.clone()))
            .configure(routes::init_routes)
    })
        .bind(("0.0.0.0", settings.server_port))?
        .run()
        .await
}