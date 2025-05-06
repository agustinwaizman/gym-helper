use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MembershipKind {
    ThreePerWeek,
    FivePerWeek,
}

#[derive(Serialize, Deserialize)]
pub struct Membership {
    pub id: i32,
    pub client_id: i32,
    pub start_date: String,
    pub end_date: String,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub kind: MembershipKind,
}