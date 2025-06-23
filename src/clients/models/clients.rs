use serde::{Deserialize, Serialize};
use sqlx::{Row, mysql::MySqlRow};
use chrono::NaiveDateTime;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub age: i32,
    pub phone: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Client {
    pub fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            last_name: row.get("last_name"),
            age: row.get("age"),
            phone: row.get("phone"),
            active: row.get::<i8, _>("active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        }
    }
}