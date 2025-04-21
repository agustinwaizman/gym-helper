use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::{FromRow, Type};


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type)]
#[sqlx(type_name = "ENUM('Admin', 'Trainer')")]
#[sqlx(rename_all = "PascalCase")]
pub enum UserRole {
    Admin,
    Trainer,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash: String,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResult {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub user_id: usize,
    pub role: String,
}

impl From<String> for UserRole {
    fn from(role: String) -> Self {
        match role.as_str() {
            "Admin" => UserRole::Admin,
            "Trainer" => UserRole::Trainer,
            _ => UserRole::Trainer,
        }
    }
}