pub mod models;
pub mod handlers;
pub mod services;
pub mod membership;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clients")
            .service(services::create_client)
            .service(services::get_clients)
            .service(services::get_clients_by_query_params)
            .service(services::get_client_by_id)
            .service(services::delete_client_by_id)
            .service(services::update_client_by_admin)
            .service(services::alta_client)
    );
}