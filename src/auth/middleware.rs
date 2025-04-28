use actix_web::{web, dev::ServiceRequest, Error, error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::auth::jwt::validate_token;
use tracing::{info, error};
use super::models::jwt_models::TokenType;

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
        Ok(claims) => match claims.token_type {
            TokenType::Access => {
                info!("Access token is valid");
                return Ok(req);
            },
            TokenType::Refresh => {
                error!("Invalid token type: {:?}", claims.token_type);
                return Err((error::ErrorUnauthorized("Invalid token type"), req));
            }
        },
        Err(e) => {
            error!("Token validation failed: {}", e);
            return Err((error::ErrorUnauthorized("Invalid token"), req));
        }
    }
}