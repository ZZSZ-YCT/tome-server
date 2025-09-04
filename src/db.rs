use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn init_db(database_url: String) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true);

    Database::connect(opt)
        .await
        .expect("Failed to connect to database")
}