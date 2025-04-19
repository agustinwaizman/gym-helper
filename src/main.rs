mod config;
mod db;
mod logging;
mod auth;

use std::usize;

use actix_web::{web, App, HttpServer};
use config::Config;
use db::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("env/env.json");
    let db_pool = create_pool(&config.database_url).await;
    let _log_guard = logging::init();
    let iss = "Gym Helper".to_owned();
    let sub = "Gym Helper API".to_owned();
    let duration: i64 = 60;
    let user_id: usize = 1;
    let jwt_secret = config.jwt_secret.clone();
    let token = auth::authentication::generate_token(iss, sub, duration, user_id, jwt_secret);
    tracing::info!("Generated JWT Token: {}", token);

    tracing::info!("🚀 Gym Helper has been started.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
    })
    .bind(config.api_bind.clone())?
    .run()
    .await
}