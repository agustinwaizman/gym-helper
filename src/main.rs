mod config;
mod db;
mod logging;
mod auth;
mod clients;

use actix_web::{web, App, HttpServer};
use config::Config;
use db::create_pool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("env/env.json");
    let db_pool = create_pool(&config.database_url).await;
    let _log_guard = logging::init();

    tracing::info!("🚀 Gym Helper has been started.");

    let address = config.api_bind.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(auth::routes)
    })
    .bind(address)?
    .run()
    .await
}