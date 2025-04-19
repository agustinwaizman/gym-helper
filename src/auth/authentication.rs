use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};


#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    iss: String,
    sub: String,
    exp: usize,
    iat: usize,
    user_id: usize,
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
