use actix_web::{get, post, web, HttpResponse};
use sqlx::MySqlPool;
use super::models::{NewSubscriptionRequest, Subscription};
use super::handlers::get_subscription_by_client_id;
use crate::membership::handlers::get_membership_by_id;
use crate::subscription::models::ValidateRequest;

use chrono::{Duration, Utc};


#[post("/")]
pub async fn new_subscription(
    pool: web::Data<MySqlPool>,
    req: web::Json<NewSubscriptionRequest>,
) -> HttpResponse {
    let request = req.into_inner();

    match request.validate(&pool).await {
        Ok(_) => {
            let membership = get_membership_by_id(&pool, request.membership_id).await.unwrap().unwrap();
            let discipline_id = membership.discipline_id;
            match get_subscription_by_client_id(&pool, request.client_id, membership.discipline_id).await {
                Ok(Some(subscription)) => {
                    let remaining_classes = subscription.remaining_classes + membership.total_classes;
                    let now = Utc::now().naive_utc();
                    let expires_at = now + Duration::days(membership.duration_days as i64);
                    let updated_subscription = subscription.update(&pool, remaining_classes, expires_at).await.unwrap();
                    tracing::info!("Subscription updated successfully");
                    HttpResponse::Ok().json(updated_subscription)
                },
                Ok(None) => {
                    let new_subscription = Subscription::create(&pool, &request, &membership).await.unwrap();
                    tracing::info!("Subscription created successfully");
                    HttpResponse::Created().json(new_subscription)
                },
                Err(e) => {
                    tracing::error!("Error fetching subscription: {}", e);
                    HttpResponse::InternalServerError().body("Error fetching subscription")
                }
            }
        },
        Err(e) => {
            tracing::error!("Error creating subscription: {}", e);
            HttpResponse::BadRequest().body(format!("Error creating subscription {}", e))
        }
    }
}

#[get("/{id}")]
pub async fn get_subscription_by_id(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    match Subscription::get_by_id(&pool, id.into_inner()).await {
        Ok(Some(subscription)) => {
            tracing::info!("Subscription fetched successfully");
            HttpResponse::Ok().json(subscription)
        },
        Ok(None) => {
            tracing::info!("Subscription not found");
            HttpResponse::NotFound().body("Subscription not found")
        },
        Err(e) => {
            tracing::error!("Error fetching subscription: {}", e);
            HttpResponse::InternalServerError().body("Error fetching subscription")
        }
    }
}