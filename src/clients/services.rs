// GET mediante id
// GET mediante query params (name, last_name, phone, membership, active)
// GET todos los clientes

// POST para crear un cliente *
// PUT para actualizar un cliente
// DELETE para eliminar un cliente (soft delete)
// PATCH para actualizar un cliente (soft delete)

use actix_web::{post, web, HttpResponse};
use sqlx::MySqlPool;
use crate::clients::models::requests::CreateClientRequest;
use crate::clients::handlers::create_client_in_db;

#[post("/clients")]
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
