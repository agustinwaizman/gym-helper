pub mod jwt;
pub mod models;
pub mod handlers;
pub mod middleware;
pub mod services;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(services::register)
            .service(services::login)
            .service(services::refresh)
    );
}