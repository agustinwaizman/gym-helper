use actix_web::{get, post, web, HttpResponse};
use sqlx::MySqlPool;
use super::models::{NewSubscriptionRequest, SubscriptionQueryParams};
use crate::membership::handlers::get_membership_by_id;
use super::handlers::{
    get_subscription_by_client_id, update_subscription_handler,
    create_subscription_handler, get_subscription_by_id_handler,
    get_all_subscriptions_handler, get_subscription_by_query_params_handler};

use chrono::{Duration, Utc};


#[post("/")]
pub async fn new_subscription(
    pool: web::Data<MySqlPool>,
    req: web::Json<NewSubscriptionRequest>,
) -> HttpResponse {
    let request = req.into_inner();

    match request.validate(&pool).await {
        Ok(_) => {
            let membership = get_membership_by_id(&pool, request.membership_id)
                .await
                .unwrap()
                .unwrap();
            match get_subscription_by_client_id(&pool, request.client_id, membership.discipline_id).await {
                Ok(Some(subscription)) => {
                    let remaining_classes = subscription.remaining_classes + membership.total_classes;
                    let now = Utc::now().naive_utc();
                    let expires_at = now + Duration::days(membership.duration_days as i64);
                    let updated_subscription = update_subscription_handler(
                        &pool, subscription.id, remaining_classes, expires_at)
                            .await
                            .unwrap();
                    tracing::info!("Subscription updated successfully");
                    HttpResponse::Ok().json(updated_subscription)
                },
                Ok(None) => {
                    let new_subscription = create_subscription_handler(&pool, &request, &membership).await.unwrap();
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
    match get_subscription_by_id_handler(&pool, id.into_inner()).await {
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

#[get("/")]
pub async fn get_all_subscriptions(
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    match get_all_subscriptions_handler(&pool).await {
        Ok(subscriptions) => {
            tracing::info!("Subscriptions fetched successfully");
            HttpResponse::Ok().json(subscriptions)
        },
        Err(e) => {
            tracing::error!("Error fetching subscriptions: {}", e);
            HttpResponse::InternalServerError().body("Error fetching subscriptions")
        }
    }
}

#[get("/filter")]
pub async fn get_subscription_by_query_params(
    pool: web::Data<MySqlPool>,
    query: web::Query<SubscriptionQueryParams>,
) -> HttpResponse {
    match get_subscription_by_query_params_handler(&pool, query.into_inner()).await {
        Ok(subscriptions) => {
            tracing::info!("Subscriptions fetched successfully");
            HttpResponse::Ok().json(subscriptions)
        },
        Err(e) => {
            tracing::error!("Error fetching subscriptions: {}", e);
            HttpResponse::InternalServerError().body("Error fetching subscriptions")
        }
    }
}
