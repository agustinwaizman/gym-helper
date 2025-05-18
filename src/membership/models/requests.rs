use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct NewDisciplineRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewMembershipRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
    pub discipline_id: i32,
    pub total_classes: i32,
    pub duration_days: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct NewClientMembershipRequest {
    pub client_id: i32,
    pub membership_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewClassAttendanceRequest {
    pub client_membership_id: i32,
}