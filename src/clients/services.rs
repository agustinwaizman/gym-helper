// GET mediante id
// GET mediante query params (name, last_name, phone, membership, active)
// GET todos los clientes

// POST para crear un cliente *
// PUT para actualizar un cliente
// DELETE para eliminar un cliente (soft delete)
// PATCH para actualizar un cliente (soft delete)

use actix_web::{get, post, web, HttpResponse};
use sqlx::MySqlPool;
use crate::clients::models::requests::{CreateClientRequest, ClientQueryParams};
use super::handlers::{obtain_client_by_id, create_client_in_db, obtain_clients, filter_clients};

#[post("/")]
pub async fn create_client(
    pool: web::Data<MySqlPool>,
    req: web::Json<CreateClientRequest>
) -> HttpResponse {
    match create_client_in_db(&pool, &req.name, &req.last_name, req.age, &req.phone).await {
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


// #[get("/clients/{id}")]
// pub async fn get_client_by_id(
//     pool: web::Data<MySqlPool>,
//     id: web::Path<i32>,
// ) -> HttpResponse {
//     let client = sqlx::query_as::<_, Client>("SELECT * FROM clients WHERE id = ?")
//         .bind(id.into_inner())
//         .fetch_one(pool.get_ref())
//         .await;

//     match client {
//         Ok(client) => HttpResponse::Ok().json(client),
//         Err(_) => HttpResponse::NotFound().body("Client not found"),
//     }
// }
