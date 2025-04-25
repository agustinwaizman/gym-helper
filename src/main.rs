mod config;
mod db;
mod logging;
mod auth;
mod clients;

use actix_web::{web, App, HttpServer};
use config::Config;
use db::create_pool;
use actix_web_httpauth::middleware::HttpAuthentication;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("env/env.json");
    let db_pool = create_pool(&config.database_url).await;
    let _log_guard = logging::init();

    tracing::info!("🚀 Gym Helper has been started.");

    let address = config.api_bind.clone();

    HttpServer::new(move || {

        let auth = HttpAuthentication::with_fn(auth::middleware::auth_middleware);

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(auth::routes)
            .service(
                web::scope("/api").wrap(auth)
                    .service(test_of_auth))
    })
    .bind(address)?
    .run()
    .await
}

#[actix_web::get("/test")]
async fn test_of_auth() -> &'static str {
    "Hello, this is a test of auth"
}