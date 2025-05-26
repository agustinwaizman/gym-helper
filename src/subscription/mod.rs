pub mod models;
pub mod handlers;
pub mod services;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/subscriptions")
            .service(services::new_subscription)
            .service(services::get_subscription_by_query_params)
            .service(services::get_subscription_by_id)
            .service(services::get_all_subscriptions)
            .service(services::class_attandance)
    );
}