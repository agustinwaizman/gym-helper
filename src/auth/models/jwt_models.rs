use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub user_id: usize,
    pub token_type: TokenType,
    pub role: String,
}
