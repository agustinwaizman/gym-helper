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

impl From<String> for UserRole {
    fn from(role: String) -> Self {
        match role.as_str() {
            "Admin" => UserRole::Admin,
            "Trainer" => UserRole::Trainer,
            _ => UserRole::Trainer,
        }
    }
}