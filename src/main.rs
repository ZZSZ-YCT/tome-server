use actix_web::{App, HttpServer, middleware, web};
use flexi_logger::{Duplicate, FileSpec, Logger};
use log::info;
use tome_server::routes;
use tome_server::db;
use tome_server::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = config::load_settings();

    Logger::try_with_str(settings.log_level.clone())
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs"))
        .duplicate_to_stdout(Duplicate::Info)
        .start()
        .unwrap();

    info!("Logger initialized, starting server…");
    
    let db = db::init_db(settings.database_url.clone()).await;
    let server_port = settings.server_port;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(settings.clone()))
            .configure(routes::init_routes)
    })
        .bind(("0.0.0.0", server_port))?
        .run()
        .await
}