// GET mediante id
// GET mediante query params (name, last_name, phone, membership, active)
// GET todos los clientes

// POST para crear un cliente *
// PUT para actualizar un cliente
// DELETE para eliminar un cliente (soft delete)
// PATCH para actualizar un cliente (soft delete)

use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use sqlx::MySqlPool;
use crate::clients::models::requests::{CreateClientRequest, ClientQueryParams};
use super::handlers::{
    obtain_client_by_id, create_client_in_db,
    obtain_clients, filter_clients, delete_client,
    update_client, activate_client};

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
    match delete_client(&pool, id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Client deleted successfully"),
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