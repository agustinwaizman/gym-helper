use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
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
    pub fn new(
        id: i32, name: String, last_name: String,
        age: i32, phone: String, active: bool,
        created_at: NaiveDateTime, updated_at: NaiveDateTime,
        deleted_at: Option<NaiveDateTime>,
    ) -> Self {

        Self {
            id,
            name,
            last_name,
            age,
            phone,
            active,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}