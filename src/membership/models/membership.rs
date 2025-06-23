use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Discipline {
    pub id: i32,
    pub name: String, // "CrossFit", "Yoga", "Funcional", etc.
    pub description: Option<String>, // "A high-intensity workout combining cardio and strength training."
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Membership {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
    pub discipline_id: i32,
    pub total_classes: i32,
    pub active: bool,
    pub duration_days: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct ClassAttendance {
    pub id: i32,
    pub client_membership_id: i32,
    pub attended_at: NaiveDateTime,
}

impl Membership {
    pub fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            price: row.get("price"),
            discipline_id: row.get("discipline_id"),
            total_classes: row.get("total_classes"),
            active: row.get::<i8, _>("active") != 0,
            duration_days: row.get("duration_days"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        }
    }
}