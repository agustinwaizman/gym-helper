use sqlx::mysql::MySqlQueryResult;
use sqlx::{self, MySqlPool};
use super::models::clients::Client;
use super::models::requests::{ClientQueryParams, CreateClientRequest};
use sqlx::Arguments;
use sqlx::mysql::MySqlArguments;
use crate::add_filter;


pub async fn create_client_in_db(
    pool: &MySqlPool,
    req: CreateClientRequest,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO clients (name, last_name, age, phone, active) VALUES (?, ?, ?, ?, ?)
        "#
    )
    .bind(req.name)
    .bind(req.last_name)
    .bind(req.age)
    .bind(req.phone)
    .bind(true)
    .execute(pool)
    .await;

    result
}

pub async fn obtain_client_by_id(
    pool: &MySqlPool,
    id: i32,
) -> Result<Option<Client>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT * FROM clients WHERE id = ?
        "#)
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|row| Client::from_row(&row)))
}

pub async fn obtain_clients(
    pool: &MySqlPool,
) -> Result<Vec<Client>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT * FROM clients
        "#)
        .fetch_all(pool)
        .await?;

    Ok(rows.iter().map(Client::from_row).collect())
}

pub async fn filter_clients(
    pool: &MySqlPool,
    params: ClientQueryParams,
) -> Result <Vec<Client>, sqlx::Error> {
    let mut query = String::from("SELECT * FROM clients WHERE 1=1");
    let mut args = MySqlArguments::default();

    add_filter!(query, args, &params.name, " AND name = ?");
    add_filter!(query, args, &params.last_name, " AND last_name = ?");
    add_filter!(query, args, &params.age, " AND age = ?");
    add_filter!(query, args, &params.phone, " AND phone = ?");
    add_filter!(query, args, &params.active, " AND active = ?");
    add_filter!(query, args, &params.created_at, " AND created_at = ?");
    add_filter!(query, args, &params.updated_at, " AND updated_at = ?");
    add_filter!(query, args, &params.deleted_at, " AND deleted_at = ?");
    add_filter!(query, args, &params.created_from, " AND created_at >= ?");
    add_filter!(query, args, &params.created_to, " AND created_at <= ?");
    add_filter!(query, args, &params.updated_from, " AND updated_at >= ?");
    add_filter!(query, args, &params.updated_to, " AND updated_at <= ?");
    add_filter!(query, args, &params.deleted_from, " AND deleted_at >= ?");
    add_filter!(query, args, &params.deleted_to, " AND deleted_at <= ?");

    let rows = sqlx::query_with(&query, args)
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(Client::from_row).collect())
}

pub async fn delete_client(
    pool: &MySqlPool,
    id: i32
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE clients 
        SET deleted_at = NOW(), active = false
        WHERE id = ?"#)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result)
}

pub async fn update_client(
    pool: &MySqlPool,
    id: i32,
    req: CreateClientRequest
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE clients
        SET name = ?, last_name = ?, age = ?, phone = ?, active = ?, deleted_at = NULL
        WHERE id = ?
        "#)
        .bind(req.name)
        .bind(req.last_name)
        .bind(req.age)
        .bind(req.phone)
        .bind(true)
        .bind(id)
        .execute(pool)
        .await;

    result
}

pub async fn activate_client(
    pool: &MySqlPool,
    id: i32,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE clients
        SET active = true, deleted_at = NULL
        WHERE id = ?
        "#)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result)
}
