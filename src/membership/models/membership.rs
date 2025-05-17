use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Discipline {
    pub id: i32,
    pub name: String, // "CrossFit", "Yoga", "Funcional", etc.
    pub description: Option<String>, // "A high-intensity workout combining cardio and strength training."
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Membership {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
    pub discipline_id: i32,
    pub total_classes: i32,
    pub active: bool,
    pub duration_days: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientMembership {
    pub id: i32,
    pub client_id: i32,
    pub membership_id: i32,
    pub purchased_at: NaiveDateTime,
    pub remaining_classes: Option<i32>,
    pub expires_at: Option<NaiveDateTime>,
    pub active: bool,
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
