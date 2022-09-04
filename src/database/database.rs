use std::time::Duration;

use sea_orm::{Database,DatabaseConnection, ConnectOptions};

#[allow(unused)]
pub async fn run() -> Result<DatabaseConnection,migration::DbErr>{
    let mut opt = ConnectOptions::new(dotenv::var("DATABASE_URL").unwrap().to_owned());
    opt .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    Database::connect(opt).await
}