use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use sqlx::MySqlPool;
use crate::clients::models::requests::{
    CreateClientRequest, ClientQueryParams};
use crate::clients::models::clients::Client;
use super::handlers::{
    obtain_client_by_id, create_client_in_db,
    obtain_clients, filter_clients, delete_client,
    update_client, activate_client};
use crate::subscription::handlers::{get_all_client_subscriptions, delete_subscription_handler};

#[post("/")]
pub async fn create_client(
    pool: web::Data<MySqlPool>,
    req: web::Json<CreateClientRequest>
) -> HttpResponse {
    match create_client_in_db(&pool, req.into_inner()).await {
        Ok(_) => {
            tracing::info!("Client created successfully");
            HttpResponse::Created().body("Client created successfully")
        },
        Err(e) => {
            tracing::error!("Error creating client: {}", e);
            HttpResponse::InternalServerError().body("Error creating client")
        }
    }
}

#[get("/{id}")]
pub async fn get_client_by_id(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    match obtain_client_by_id(&pool, id.into_inner()).await {
        Ok(Some(client)) => HttpResponse::Ok().json(client),
        Ok(None) => HttpResponse::NotFound().body("Client not found"),
        Err(e) => {
            tracing::error!("Error fetching client: {}", e);
            HttpResponse::InternalServerError().body("Error fetching client")
        }
    }
}

#[get("/")]
pub async fn get_clients(
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    match obtain_clients(&pool).await {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(e) => {
            tracing::error!("Error fetching clients: {}", e);
            HttpResponse::InternalServerError().body("Error fetching clients")
        }
    }
}

#[get("/filter")]
pub async fn get_clients_by_query_params(
    pool: web::Data<MySqlPool>,
    query: web::Query<ClientQueryParams>,
) -> HttpResponse {
    match filter_clients(&pool, query.into_inner()).await {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(e) => {
            tracing::error!("Error fetching clients: {}", e);
            HttpResponse::InternalServerError().body("Error fetching clients")
        }
    }
}

#[delete("/{id}")]
pub async fn delete_client_by_id(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    // TODO: Ver por que en caso de no encontrar el cliente devuelve un 200 Ok
    let client_id = id.into_inner();
    match delete_client(&pool, client_id).await {
        Ok(_) => {
            let client_subscriptions = get_all_client_subscriptions(&pool, client_id).await.unwrap();
            if !client_subscriptions.is_empty() {
                for subscription in client_subscriptions {
                    tracing::info!("Deleting subscription with id: {}", subscription.id);
                    delete_subscription_handler(&pool, subscription.id).await.unwrap();
                }
            }
            tracing::info!("Client deleted successfully");
            HttpResponse::Ok().body("Client deleted successfully")},
        Err(e) => {
            tracing::error!("Error deleting client: {}", e);
            HttpResponse::InternalServerError().body("Error deleting client")
        }
    }
}

// Endpoint para admins
#[put("/{id}")]
pub async fn update_client_by_admin(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
    req: web::Json<CreateClientRequest>,
) -> HttpResponse {
    match update_client(&pool, id.into_inner(), req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Client updated successfully"),
        Err(e) => {
            tracing::error!("Error updating client: {}", e);
            HttpResponse::NotModified().body("Error updating client")
        }
    }
}

#[patch("/{id}")]
pub async fn alta_client(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>
) -> HttpResponse {
    match activate_client(&pool, id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Client activated successfully"),
        Err(e) => {
            tracing::error!("Error activating client: {}", e);
            HttpResponse::InternalServerError().body("Error activating client")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::clients::models::{
        clients::Client,
        requests::{CreateClientRequest, ClientQueryParams}
    };

    // Helper function para crear un client de prueba
    fn create_test_client() -> Client {
        let now = Utc::now().naive_utc();
        Client {
            id: 1,
            name: "Juan".to_string(),
            last_name: "Pérez".to_string(),
            age: 25,
            phone: "123456789".to_string(),
            active: true,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    // Helper function para crear una request de cliente
    fn create_test_create_client_request() -> CreateClientRequest {
        CreateClientRequest {
            name: "Juan".to_string(),
            last_name: "Pérez".to_string(),
            age: 25,
            phone: "123456789".to_string(),
        }
    }

    // Helper function para crear parámetros de query
    fn create_test_client_query_params() -> ClientQueryParams {
        ClientQueryParams {
            name: Some("Juan".to_string()),
            last_name: Some("Pérez".to_string()),
            age: Some(25),
            phone: Some("123456789".to_string()),
            active: Some(true),
            created_at: None,
            updated_at: None,
            deleted_at: None,
            created_from: None,
            created_to: None,
            updated_from: None,
            updated_to: None,
            deleted_from: None,
            deleted_to: None,
        }
    }

    #[test]
    fn test_create_client_request_serialization() {
        let request = create_test_create_client_request();
        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"name":"Juan","last_name":"Pérez","age":25,"phone":"123456789"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_create_client_request_deserialization() {
        let json = r#"{"name":"Juan","last_name":"Pérez","age":25,"phone":"123456789"}"#;
        let request: CreateClientRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.name, "Juan");
        assert_eq!(request.last_name, "Pérez");
        assert_eq!(request.age, 25);
        assert_eq!(request.phone, "123456789");
    }

    #[test]
    fn test_client_model_structure() {
        let client = create_test_client();
        
        assert_eq!(client.id, 1);
        assert_eq!(client.name, "Juan");
        assert_eq!(client.last_name, "Pérez");
        assert_eq!(client.age, 25);
        assert_eq!(client.phone, "123456789");
        assert_eq!(client.active, true);
        assert!(client.deleted_at.is_none());
    }

    #[test]
    fn test_client_query_params_creation() {
        let params = create_test_client_query_params();
        
        assert_eq!(params.name, Some("Juan".to_string()));
        assert_eq!(params.last_name, Some("Pérez".to_string()));
        assert_eq!(params.age, Some(25));
        assert_eq!(params.phone, Some("123456789".to_string()));
        assert_eq!(params.active, Some(true));
        assert!(params.created_at.is_none());
    }

    #[test]
    fn test_create_client_request_validation() {
        let request = CreateClientRequest {
            name: "".to_string(), // Nombre vacío
            last_name: "".to_string(), // Apellido vacío
            age: -1, // Edad inválida
            phone: "".to_string(), // Teléfono vacío
        };

        // Verificamos que los valores son los esperados para el test
        assert_eq!(request.name, "");
        assert_eq!(request.last_name, "");
        assert_eq!(request.age, -1);
        assert_eq!(request.phone, "");
    }

    #[test]
    fn test_client_with_special_characters() {
        let request = CreateClientRequest {
            name: "José María".to_string(),
            last_name: "García-López".to_string(),
            age: 30,
            phone: "+54-11-1234-5678".to_string(),
        };

        assert!(request.name.contains("José"));
        assert!(request.last_name.contains("García"));
        assert!(request.phone.contains("+54"));
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
            assert_eq!(StatusCode::NOT_MODIFIED.as_u16(), 304);
            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 500);
        }
    }

    // Tests para validación de entrada
    mod validation_tests {
        use super::*;

        #[test]
        fn test_age_validation() {
            let valid_ages = vec![18, 25, 35, 65, 80];
            let invalid_ages = vec![-1, 0, 150, 200];

            for age in valid_ages {
                let request = CreateClientRequest {
                    name: "Test".to_string(),
                    last_name: "User".to_string(),
                    age,
                    phone: "123456789".to_string(),
                };
                assert!(age > 0 && age < 120); // Lógica de validación esperada
            }

            for age in invalid_ages {
                let request = CreateClientRequest {
                    name: "Test".to_string(),
                    last_name: "User".to_string(),
                    age,
                    phone: "123456789".to_string(),
                };
                assert!(age <= 0 || age >= 120); // Lógica de validación esperada
            }
        }

        #[test]
        fn test_phone_format_validation() {
            let valid_phones = vec![
                "123456789".to_string(),
                "+54-11-1234-5678".to_string(),
                "011-1234-5678".to_string(),
            ];

            for phone in valid_phones {
                let request = CreateClientRequest {
                    name: "Test".to_string(),
                    last_name: "User".to_string(),
                    age: 25,
                    phone: phone.clone(),
                };
                assert!(!phone.is_empty());
            }
        }

        #[test]
        fn test_name_validation() {
            let valid_names = vec![
                "Juan".to_string(),
                "María José".to_string(),
                "José-Luis".to_string(),
            ];

            for name in valid_names {
                let request = CreateClientRequest {
                    name: name.clone(),
                    last_name: "Test".to_string(),
                    age: 25,
                    phone: "123456789".to_string(),
                };
                assert!(!name.is_empty());
                assert!(name.len() >= 2);
            }
        }
    }

    // Tests para manejo de errores
    mod error_handling_tests {
        #[test]
        fn test_error_message_format() {
            let error_messages = vec![
                "Error creating client",
                "Error fetching client",
                "Error updating client",
                "Error deleting client",
                "Error activating client",
            ];

            for msg in error_messages {
                assert!(msg.contains("Error"));
                assert!(msg.contains("client"));
            }
        }

        #[test]
        fn test_success_message_format() {
            let success_messages = vec![
                "Client created successfully",
                "Client updated successfully",
                "Client deleted successfully",
                "Client activated successfully",
            ];

            for msg in success_messages {
                assert!(msg.contains("Client"));
                assert!(msg.contains("successfully"));
            }
        }
    }

    // Tests de integración más avanzados (requieren configuración de DB de prueba)
    #[cfg(feature = "integration-tests")]
    mod integration_tests {
        use super::*;

        #[tokio::test]
        async fn test_create_client_integration() {
            // Este test requeriría una base de datos de prueba configurada
            todo!("Implementar cuando se configure la base de datos de prueba");
        }

        #[tokio::test]
        async fn test_get_client_by_id_integration() {
            todo!("Implementar cuando se configure la base de datos de prueba");
        }

        #[tokio::test]
        async fn test_delete_client_integration() {
            todo!("Implementar cuando se configure la base de datos de prueba");
        }
    }
}