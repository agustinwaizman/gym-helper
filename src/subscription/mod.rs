pub mod models;
pub mod handlers;
pub mod services;
pub mod utils;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/subscriptions")
            .service(services::new_subscription)
            .service(services::get_subscription_by_id)
    );
}