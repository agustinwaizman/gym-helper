use sqlx::mysql::MySqlQueryResult;
use sqlx::{self, MySqlPool};
use super::models::clients::Client;
use super::models::requests::{ClientQueryParams, CreateClientRequest};
use sqlx::Arguments;
use sqlx::mysql::MySqlArguments;


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

    if let Some(name) = &params.name {
        query.push_str(" AND name = ?");
        args.add(name);
    }
    if let Some(last_name) = &params.last_name {
        query.push_str(" AND last_name = ?");
        args.add(last_name);
    }
    if let Some(age) = params.age {
        query.push_str(" AND age = ?");
        args.add(age);
    }
    if let Some(phone) = &params.phone {
        query.push_str(" AND phone = ?");
        args.add(phone);
    }
    if let Some(active) = params.active {
        query.push_str(" AND active = ?");
        args.add(active);
    }
    if let Some(created_at) = params.created_at {
        query.push_str(" AND created_at = ?");
        args.add(created_at);
    }
    if let Some(updated_at) = params.updated_at {
        query.push_str(" AND updated_at = ?");
        args.add(updated_at);
    }
    if let Some(deleted_at) = params.deleted_at {
        query.push_str(" AND deleted_at = ?");
        args.add(deleted_at);
    }
    if let Some(from) = params.created_from {
        query.push_str(" AND created_at >= ?");
        args.add(from);
    }
    if let Some(to) = params.created_to {
        query.push_str(" AND created_at <= ?");
        args.add(to);
    }
    if let Some(from) = params.updated_from {
        query.push_str(" AND updated_at >= ?");
        args.add(from);
    }
    if let Some(to) = params.updated_to {
        query.push_str(" AND updated_at <= ?");
        args.add(to);
    }
    if let Some(from) = params.deleted_from {
        query.push_str(" AND deleted_at >= ?");
        args.add(from);
    }
    if let Some(to) = params.deleted_to {
        query.push_str(" AND deleted_at <= ?");
        args.add(to);
    }
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
