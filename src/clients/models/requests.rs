use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateClientRequest {
    pub name: String,
    pub last_name: String,
    pub age: i32,
    pub phone: String,
}

#[derive(Deserialize, Debug)]
pub struct ClientQueryParams {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<i32>,
    pub phone: Option<String>,
    pub active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_from: Option<NaiveDateTime>,
    pub created_to: Option<NaiveDateTime>,
    pub updated_from: Option<NaiveDateTime>,
    pub updated_to: Option<NaiveDateTime>,
    pub deleted_from: Option<NaiveDateTime>,
    pub deleted_to: Option<NaiveDateTime>,
}