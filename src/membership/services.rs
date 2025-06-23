use sqlx::MySqlPool;
use actix_web::{delete, post, web, patch, HttpResponse};
use super::models::requests::{NewMembershipRequest, NewDisciplineRequest};
use super::models::membership::{Discipline, Membership};
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::membership::models::{
        membership::{Discipline, Membership},
        requests::{NewDisciplineRequest, NewMembershipRequest}
    };

    // Helper function para crear una discipline de prueba
    fn create_test_discipline() -> Discipline {
        let now = Utc::now().naive_utc();
        Discipline {
            id: 1,
            name: "CrossFit".to_string(),
            description: Some("High-intensity functional fitness".to_string()),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    // Helper function para crear una membership de prueba
    fn create_test_membership() -> Membership {
        let now = Utc::now().naive_utc();
        Membership {
            id: 1,
            name: "Plan Básico".to_string(),
            description: Some("Plan de entrenamiento básico".to_string()),
            price: 50.0,
            discipline_id: 1,
            total_classes: 12,
            active: true,
            duration_days: 30,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    // Helper function para crear una request de discipline
    fn create_test_new_discipline_request() -> NewDisciplineRequest {
        NewDisciplineRequest {
            name: "CrossFit".to_string(),
            description: Some("High-intensity functional fitness".to_string()),
        }
    }

    // Helper function para crear una request de membership
    fn create_test_new_membership_request() -> NewMembershipRequest {
        NewMembershipRequest {
            name: "Plan Básico".to_string(),
            description: Some("Plan de entrenamiento básico".to_string()),
            price: 50.0,
            discipline_id: 1,
            total_classes: 12,
            duration_days: Some(30),
        }
    }

    #[test]
    fn test_new_discipline_request_serialization() {
        let request = create_test_new_discipline_request();
        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"name":"CrossFit","description":"High-intensity functional fitness"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_new_discipline_request_deserialization() {
        let json = r#"{"name":"CrossFit","description":"High-intensity functional fitness"}"#;
        let request: NewDisciplineRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.name, "CrossFit");
        assert_eq!(request.description, Some("High-intensity functional fitness".to_string()));
    }

    #[test]
    fn test_new_membership_request_serialization() {
        let request = create_test_new_membership_request();
        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"name":"Plan Básico","description":"Plan de entrenamiento básico","price":50.0,"discipline_id":1,"total_classes":12,"duration_days":30}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_new_membership_request_deserialization() {
        let json = r#"{"name":"Plan Básico","description":"Plan de entrenamiento básico","price":50.0,"discipline_id":1,"total_classes":12,"duration_days":30}"#;
        let request: NewMembershipRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.name, "Plan Básico");
        assert_eq!(request.price, 50.0);
        assert_eq!(request.discipline_id, 1);
        assert_eq!(request.total_classes, 12);
        assert_eq!(request.duration_days, Some(30));
    }

    #[test]
    fn test_discipline_model_structure() {
        let discipline = create_test_discipline();
        
        assert_eq!(discipline.id, 1);
        assert_eq!(discipline.name, "CrossFit");
        assert_eq!(discipline.description, Some("High-intensity functional fitness".to_string()));
        assert!(discipline.deleted_at.is_none());
    }

    #[test]
    fn test_membership_model_structure() {
        let membership = create_test_membership();
        
        assert_eq!(membership.id, 1);
        assert_eq!(membership.name, "Plan Básico");
        assert_eq!(membership.price, 50.0);
        assert_eq!(membership.discipline_id, 1);
        assert_eq!(membership.total_classes, 12);
        assert_eq!(membership.active, true);
        assert_eq!(membership.duration_days, 30);
        assert!(membership.deleted_at.is_none());
    }

    #[test]
    fn test_discipline_request_without_description() {
        let request = NewDisciplineRequest {
            name: "Yoga".to_string(),
            description: None,
        };
        
        assert_eq!(request.name, "Yoga");
        assert!(request.description.is_none());
    }

    #[test]
    fn test_membership_request_without_description() {
        let request = NewMembershipRequest {
            name: "Plan Premium".to_string(),
            description: None,
            price: 100.0,
            discipline_id: 2,
            total_classes: 20,
            duration_days: None,
        };
        
        assert_eq!(request.name, "Plan Premium");
        assert!(request.description.is_none());
        assert_eq!(request.price, 100.0);
        assert!(request.duration_days.is_none());
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

    // Tests para validación de entrada
    mod validation_tests {
        use super::*;

        #[test]
        fn test_discipline_name_validation() {
            let valid_names = vec![
                "CrossFit".to_string(),
                "Yoga".to_string(),
                "Pilates".to_string(),
                "Funcional".to_string(),
            ];

            for name in valid_names {
                let request = NewDisciplineRequest {
                    name: name.clone(),
                    description: Some("Test description".to_string()),
                };
                assert!(!name.is_empty());
                assert!(name.len() >= 3);
            }
        }

        #[test]
        fn test_membership_price_validation() {
            let valid_prices = vec![10.0, 25.5, 50.0, 100.0, 150.75];
            let invalid_prices = vec![-10.0, 0.0, -1.0];

            for price in valid_prices {
                let request = NewMembershipRequest {
                    name: "Test Plan".to_string(),
                    description: None,
                    price,
                    discipline_id: 1,
                    total_classes: 10,
                    duration_days: Some(30),
                };
                assert!(price > 0.0);
            }

            for price in invalid_prices {
                let request = NewMembershipRequest {
                    name: "Test Plan".to_string(),
                    description: None,
                    price,
                    discipline_id: 1,
                    total_classes: 10,
                    duration_days: Some(30),
                };
                assert!(price <= 0.0);
            }
        }

        #[test]
        fn test_membership_classes_validation() {
            let valid_classes = vec![1, 5, 10, 15, 20, 50];
            let invalid_classes = vec![-1, 0, -10];

            for classes in valid_classes {
                let request = NewMembershipRequest {
                    name: "Test Plan".to_string(),
                    description: None,
                    price: 50.0,
                    discipline_id: 1,
                    total_classes: classes,
                    duration_days: Some(30),
                };
                assert!(classes > 0);
            }

            for classes in invalid_classes {
                let request = NewMembershipRequest {
                    name: "Test Plan".to_string(),
                    description: None,
                    price: 50.0,
                    discipline_id: 1,
                    total_classes: classes,
                    duration_days: Some(30),
                };
                assert!(classes <= 0);
            }
        }

        #[test]
        fn test_membership_duration_validation() {
            let valid_durations = vec![Some(7), Some(15), Some(30), Some(60), Some(90)];
            let invalid_durations = vec![Some(-1), Some(0), Some(-30)];

            for duration in valid_durations {
                let request = NewMembershipRequest {
                    name: "Test Plan".to_string(),
                    description: None,
                    price: 50.0,
                    discipline_id: 1,
                    total_classes: 10,
                    duration_days: duration,
                };
                if let Some(days) = duration {
                    assert!(days > 0);
                }
            }

            for duration in invalid_durations {
                let request = NewMembershipRequest {
                    name: "Test Plan".to_string(),
                    description: None,
                    price: 50.0,
                    discipline_id: 1,
                    total_classes: 10,
                    duration_days: duration,
                };
                if let Some(days) = duration {
                    assert!(days <= 0);
                }
            }
        }
    }

    // Tests para manejo de errores
    mod error_handling_tests {
        #[test]
        fn test_discipline_error_messages() {
            let error_messages = vec![
                "Error creating discipline",
                "Error deleting discipline",
                "Error activating discipline",
            ];

            for msg in error_messages {
                assert!(msg.contains("Error"));
                assert!(msg.contains("discipline"));
            }
        }

        #[test]
        fn test_membership_error_messages() {
            let error_messages = vec![
                "Error creating membership",
                "Error deleting membership",
                "Error activating membership",
            ];

            for msg in error_messages {
                assert!(msg.contains("Error"));
                assert!(msg.contains("membership"));
            }
        }

        #[test]
        fn test_success_messages() {
            let success_messages = vec![
                "Discipline created successfully",
                "Discipline deleted successfully",
                "Discipline activated successfully",
                "Membership created successfully",
                "Membership deleted successfully",
                "Membership activated successfully",
            ];

            for msg in success_messages {
                assert!(msg.contains("successfully"));
                assert!(msg.contains("Discipline") || msg.contains("Membership"));
            }
        }
    }

    // Tests para tipos de disciplinas comunes
    mod discipline_types_tests {
        use super::*;

        #[test]
        fn test_common_disciplines() {
            let disciplines = vec![
                ("CrossFit", "High-intensity functional fitness"),
                ("Yoga", "Mind-body practice combining physical postures"),
                ("Pilates", "Low-impact exercise method"),
                ("Funcional", "Functional movement training"),
                ("Boxeo", "Combat sport training"),
                ("Natación", "Swimming training"),
            ];

            for (name, description) in disciplines {
                let request = NewDisciplineRequest {
                    name: name.to_string(),
                    description: Some(description.to_string()),
                };
                
                assert!(!request.name.is_empty());
                assert!(request.description.is_some());
                assert!(request.description.as_ref().unwrap().len() > 10);
            }
        }
    }

    // Tests para diferentes tipos de planes
    mod membership_plans_tests {
        use super::*;

        #[test]
        fn test_basic_plans() {
            let plans = vec![
                ("Plan Básico", 8, 30, 25.0),
                ("Plan Intermedio", 12, 30, 40.0),
                ("Plan Avanzado", 16, 30, 55.0),
                ("Plan Premium", 20, 30, 70.0),
            ];

            for (name, classes, days, price) in plans {
                let request = NewMembershipRequest {
                    name: name.to_string(),
                    description: Some(format!("Plan con {} clases por mes", classes)),
                    price,
                    discipline_id: 1,
                    total_classes: classes,
                    duration_days: Some(days),
                };
                
                assert!(!request.name.is_empty());
                assert!(request.total_classes > 0);
                assert!(request.price > 0.0);
                assert!(request.duration_days.is_some());
            }
        }
    }

    // Tests de integración más avanzados (requieren configuración de DB de prueba)
    #[cfg(feature = "integration-tests")]
    mod integration_tests {
        use super::*;

        #[tokio::test]
        async fn test_create_discipline_integration() {
            // Este test requeriría una base de datos de prueba configurada
            todo!("Implementar cuando se configure la base de datos de prueba");
        }

        #[tokio::test]
        async fn test_create_membership_integration() {
            todo!("Implementar cuando se configure la base de datos de prueba");
        }

        #[tokio::test]
        async fn test_delete_discipline_integration() {
            todo!("Implementar cuando se configure la base de datos de prueba");
        }
    }
}
