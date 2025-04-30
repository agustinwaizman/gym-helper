use actix_web::web;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use crate::auth::models::jwt_models::{Claims, TokenType};


pub fn generate_token(
        iss: String, sub: String, duration: i64,
        user_id: usize, role: String, 
        token_type: TokenType, key: String) -> String{
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(key.as_ref());

    let exp = (Utc::now() + Duration::minutes(duration)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let claims = Claims {
        iss,
        sub,
        exp,
        iat,
        token_type,
        user_id,
        role
    };

    encode(&header, &claims, &encoding_key).unwrap()
}

pub fn validate_token(
        token: String,
        data: web::Data<crate::config::Config>) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS512);
    let decoding_key = DecodingKey::from_secret(data.jwt_secret.as_ref());

    match decode::<Claims>(&token, &decoding_key, &validation) {
        Ok(token_data) => {
            tracing::info!("Token is valid");
            Ok(token_data.claims)
        },
        Err(err) => {
            tracing::error!("Error decoding token: {}", err);
            Err(err)
        }
    }
}