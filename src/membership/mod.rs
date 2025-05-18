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
            .service(services::activate_discipline)
            .service(services::delete_membership)
            .service(services::activate_membership)
    );
}