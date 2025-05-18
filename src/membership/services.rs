use sqlx::MySqlPool;
use actix_web::{delete, post, web, patch, HttpResponse};
use super::models::requests::{NewMembershipRequest, NewDisciplineRequest};
use super::handlers::{
    create_discipline_handler, create_membership_handler,
    delete_discipline_handler, delete_membership_by_discipline_handler,
    activate_discipline_handler, delete_membership_handler,
    activate_membership_handler};

/////////////////////////////////////////////////////////////////////////////////
/////////////////// DISCIPLINE ENDPOINTS //////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

#[post("/discipline")]
pub async fn new_discipline(
    pool: web::Data<MySqlPool>,
    req: web::Json<NewDisciplineRequest>
) -> HttpResponse {
    match create_discipline_handler(&pool, req.into_inner()).await {
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

#[delete("/discipline/{id}")]
pub async fn delete_discipline(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>
) -> HttpResponse {
    let discipline_id = id.into_inner();
    match delete_discipline_handler(&pool, discipline_id).await {
        Ok(_) => {
            delete_membership_by_discipline_handler(&pool, discipline_id).await.unwrap();
            tracing::info!("Discipline deleted successfully");
            HttpResponse::Ok().body("Discipline deleted successfully")
        },
        Err(e) => {
            tracing::error!("Error deleting discipline: {}", e);
            HttpResponse::InternalServerError().body("Error deleting discipline")
        }
    }
}

#[patch("/discipline/{id}")]
pub async fn activate_discipline(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>
) -> HttpResponse {
    match activate_discipline_handler(&pool, id.into_inner()).await {
        Ok(_) => {
            tracing::info!("Discipline activated successfully");
            HttpResponse::Ok().body("Discipline activated successfully")
        },
        Err(e) => {
            tracing::error!("Error activating discipline: {}", e);
            HttpResponse::InternalServerError().body("Error activating discipline")
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////
/////////////////// MEMBERSHIP ENDPOINTS //////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

#[post("/")]
pub async fn new_membership(
    pool: web::Data<MySqlPool>,
    req: web::Json<NewMembershipRequest>
) -> HttpResponse {
    match create_membership_handler(&pool, req.into_inner()).await {
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

#[delete("/{id}")]
pub async fn delete_membership(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>
) -> HttpResponse {
    match delete_membership_handler(&pool, id.into_inner()).await {
        Ok(_) => {
            tracing::info!("Membership deleted successfully");
            HttpResponse::Ok().body("Membership deleted successfully")
        },
        Err(e) => {
            tracing::error!("Error deleting membership: {}", e);
            HttpResponse::InternalServerError().body("Error deleting membership")
        }
    }
}

#[patch("/{id}")]
pub async fn activate_membership(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>
) -> HttpResponse {
    match activate_membership_handler(&pool, id.into_inner()).await {
        Ok(_) => {
            tracing::info!("Membership activated successfully");
            HttpResponse::Ok().body("Membership activated successfully")
        },
        Err(e) => {
            tracing::error!("Error activating membership: {}", e);
            HttpResponse::InternalServerError().body("Error activating membership")
        }
    }
}
