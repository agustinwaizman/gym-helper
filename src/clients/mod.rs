pub mod models;
pub mod handlers;
pub mod services;
pub mod membership;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clients")
            .service(services::create_client)
    );
}