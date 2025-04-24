pub mod jwt;
pub mod models;
pub mod handlers;
pub mod middleware;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(handlers::register)
            .service(handlers::login)
    );
}