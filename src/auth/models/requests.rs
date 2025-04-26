use serde::{Serialize, Deserialize};
use super::users::UserRole;

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}