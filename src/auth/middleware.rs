use actix_web::{web, dev::ServiceRequest, Error, error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::auth::jwt::validate_token;
use tracing::{info, error};

pub async fn auth_middleware(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("Missing token"), req));
    };

    let token = credentials.token();

    let result = validate_token(
        token.to_owned(), 
        req.app_data::<web::Data<crate::config::Config>>().unwrap().clone());

    match result {
        Ok(claims) => {
            info!("Token is valid: {:?}", claims);
            return Ok(req);
        },
        Err(e) => {
            error!("Token validation failed: {}", e);
            return Err((error::ErrorUnauthorized("Invalid token"), req));
        }
    }
}