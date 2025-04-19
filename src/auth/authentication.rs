use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use tracing::{info, error};


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub user_id: usize,
}

pub fn generate_token(iss: String, sub: String, duration: i64, user_id: usize, key: String) -> String{
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(key.as_ref());

    let exp = (Utc::now() + Duration::minutes(duration)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let claims = Claims {
        iss,
        sub,
        exp,
        iat,
        user_id
    };

    encode(&header, &claims, &encoding_key).unwrap()
}

pub fn validate_token(token: String, key: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS512);
    let decoding_key = DecodingKey::from_secret(key.as_ref());

    let result = decode::<Claims>(
        &token,
        &decoding_key,
        &validation,
    );

    match result {
        Ok(token_data) => {
            info!("token is valid");
            Ok(token_data.claims)
        },
        Err(err) => {
            error!("token is invalid: {}", err);
            Err(err)},
    }
}
