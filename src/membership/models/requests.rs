use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"name": "CrossFit", "description": "High-intensity functional fitness"}))]
pub struct NewDisciplineRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"name": "Plan Básico", "description": "Plan de entrenamiento básico", "price": 50.0, "discipline_id": 1, "total_classes": 12, "duration_days": 30}))]
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