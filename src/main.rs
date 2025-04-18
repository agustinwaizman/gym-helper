mod config;
mod db;
mod logging;

use actix_web::{web, App, HttpServer};
use config::Config;
use db::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("env/env.json");
    let db_pool = create_pool(&config.database_url).await;
    let _log_guard = logging::init();

    tracing::info!("🚀 Gym Helper has been started.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
    })
    .bind(config.api_bind.clone())?
    .run()
    .await
}