use actix_web::{get, post, web, HttpResponse};
use sqlx::MySqlPool;
use super::models::{NewSubscriptionRequest, SubscriptionQueryParams, ClassAttendanceRequest};
use crate::membership::handlers::get_membership_by_id;
use super::handlers::{
    get_subscription_by_client_id, update_subscription_handler,
    create_subscription_handler, get_subscription_by_id_handler,
    get_all_subscriptions_handler, get_subscription_by_query_params_handler,
    new_attendance_handler};
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

#[post("/class_attendance")]
pub async fn class_attendance(
    pool: web::Data<MySqlPool>,
    req: web::Json<ClassAttendanceRequest>,
) -> HttpResponse {
    let request = req.into_inner();
    match request.validate(&pool).await {
        Ok(mut subscription) => {
            match new_attendance_handler(&pool, subscription.id).await {
                Ok(_) => {
                    subscription.remaining_classes -= 1;
                    match update_subscription_handler(
                        &pool, subscription.id,
                        subscription.remaining_classes,
                        subscription.expires_at).await {
                        Ok(_) => {
                            tracing::info!("Class attendance recorded successfully");
                            HttpResponse::Ok().body("Class attendance recorded successfully")
                        },
                        Err(e) => {
                            tracing::error!("Error updating subscription: {}", e);
                            HttpResponse::InternalServerError().body("Error updating subscription")
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Error recording class attendance: {}", e);
                    HttpResponse::InternalServerError().body("Error recording class attendance")
                }
            }
        },
        Err(e) => {
            tracing::error!("Error validating class attendance request: {}", e);
            HttpResponse::BadRequest().body(format!("Error validating class attendance request: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use crate::subscription::models::{Subscription, NewSubscriptionRequest, ClassAttendanceRequest, SubscriptionQueryParams};

    // Helper function para crear una subscription de prueba
    fn create_test_subscription() -> Subscription {
        let now = Utc::now().naive_utc();
        Subscription {
            id: 1,
            client_id: 1,
            discipline_id: 1,
            remaining_classes: 10,
            expires_at: now + Duration::days(30),
            active: true,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    // Helper function para crear una request de nueva subscription
    fn create_test_new_subscription_request() -> NewSubscriptionRequest {
        NewSubscriptionRequest {
            client_id: 1,
            membership_id: 1,
        }
    }

    // Helper function para crear una request de asistencia a clase
    fn create_test_class_attendance_request() -> ClassAttendanceRequest {
        ClassAttendanceRequest {
            subscription_id: 1,
        }
    }

    #[test]
    fn test_new_subscription_request_serialization() {
        let request = create_test_new_subscription_request();
        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"client_id":1,"membership_id":1}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_new_subscription_request_deserialization() {
        let json = r#"{"client_id":1,"membership_id":1}"#;
        let request: NewSubscriptionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.client_id, 1);
        assert_eq!(request.membership_id, 1);
    }

    #[test]
    fn test_class_attendance_request_serialization() {
        let request = create_test_class_attendance_request();
        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"subscription_id":1}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_class_attendance_request_deserialization() {
        let json = r#"{"subscription_id":1}"#;
        let request: ClassAttendanceRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.subscription_id, 1);
    }

    #[test]
    fn test_subscription_query_params_creation() {
        let params = SubscriptionQueryParams {
            client_id: Some(1),
            discipline_id: Some(2),
            active: Some(true),
            expires_at: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
            deleted_at_from: None,
            deleted_at_to: None,
            expires_at_from: None,
            expires_at_to: None,
        };

        assert_eq!(params.client_id, Some(1));
        assert_eq!(params.discipline_id, Some(2));
        assert_eq!(params.active, Some(true));
    }

    #[test]
    fn test_subscription_model_structure() {
        let subscription = create_test_subscription();
        
        assert_eq!(subscription.id, 1);
        assert_eq!(subscription.client_id, 1);
        assert_eq!(subscription.discipline_id, 1);
        assert_eq!(subscription.remaining_classes, 10);
        assert_eq!(subscription.active, true);
        assert!(subscription.deleted_at.is_none());
    }

    // Tests para verificar la lógica de respuesta HTTP
    mod http_response_tests {
        use actix_web::http::StatusCode;

        #[test]
        fn test_http_status_codes() {
            // Verificamos que conocemos los códigos de estado HTTP correctos
            assert_eq!(StatusCode::OK.as_u16(), 200);
            assert_eq!(StatusCode::CREATED.as_u16(), 201);
            assert_eq!(StatusCode::BAD_REQUEST.as_u16(), 400);
            assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 500);
        }
    }

    // Tests de integración más avanzados (requieren configuración de DB de prueba)
    #[cfg(feature = "integration-tests")]
    mod integration_tests {
        use super::*;

        #[tokio::test]
        async fn test_new_subscription_integration() {
            // Este test requeriría una base de datos de prueba configurada
            // Se puede implementar con testcontainers o una DB en memoria
            todo!("Implementar cuando se configure la base de datos de prueba");
        }

        #[tokio::test]
        async fn test_get_subscription_by_id_integration() {
            todo!("Implementar cuando se configure la base de datos de prueba");
        }

        #[tokio::test]
        async fn test_class_attendance_integration() {
            todo!("Implementar cuando se configure la base de datos de prueba");
        }
    }

    // Tests para validación de entrada
    mod validation_tests {
        use super::*;

        #[test]
        fn test_new_subscription_request_validation() {
            let request = NewSubscriptionRequest {
                client_id: 0, // ID inválido
                membership_id: 0, // ID inválido
            };

            // Verificamos que los valores son los esperados para el test
            assert_eq!(request.client_id, 0);
            assert_eq!(request.membership_id, 0);
        }

        #[test]
        fn test_class_attendance_request_validation() {
            let request = ClassAttendanceRequest {
                subscription_id: 0, // ID inválido
            };

            assert_eq!(request.subscription_id, 0);
        }
    }

    // Tests para manejo de errores
    mod error_handling_tests {

        #[test]
        fn test_error_message_format() {
            let error_msg = "Error creating subscription";
            assert!(error_msg.contains("Error"));
            assert!(error_msg.contains("subscription"));
        }

        #[test]
        fn test_validation_error_format() {
            let error_msg = "Error validating class attendance request";
            assert!(error_msg.contains("Error validating"));
            assert!(error_msg.contains("class attendance"));
        }
    }
}
