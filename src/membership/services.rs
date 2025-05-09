use sqlx::MySqlPool;
use actix_web::{web, post, HttpResponse};
use super::models::requests::{NewMembershipRequest, NewDisciplineRequest};
use super::handlers::{
    create_discipline_in_db, create_membership_in_db};

#[post("/discipline")]
pub async fn new_discipline(
    pool: web::Data<MySqlPool>,
    req: web::Json<NewDisciplineRequest>
) -> HttpResponse {
    match create_discipline_in_db(&pool, req.into_inner()).await {
        Ok(_) => {
            tracing::info!("Discipline created successfully");
            HttpResponse::Created().body("Discipline created successfully")
        },
        Err(e) => {
            tracing::error!("Error creating discipline: {}", e);
            HttpResponse::InternalServerError().body("Error creating discipline")
        }
    }
}

#[post("/")]
pub async fn new_membership(
    pool: web::Data<MySqlPool>,
    req: web::Json<NewMembershipRequest>
) -> HttpResponse {
    match create_membership_in_db(&pool, req.into_inner()).await {
        Ok(_) => {
            tracing::info!("Membership created successfully");
            HttpResponse::Created().body("Membership created successfully")
        },
        Err(e) => {
            tracing::error!("Error creating membership: {}", e);
            HttpResponse::InternalServerError().body("Error creating membership")
        }
    }
}