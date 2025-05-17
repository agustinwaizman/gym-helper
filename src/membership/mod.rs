pub mod models;
pub mod handlers;
pub mod services;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/membership")
            .service(services::new_discipline)
            .service(services::new_membership)
            .service(services::delete_discipline)
    );
}