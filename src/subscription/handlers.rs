use sqlx::{self, mysql::MySqlArguments, MySqlPool};
use super::models::{Subscription, NewSubscriptionRequest, SubscriptionQueryParams};
use sqlx::Arguments;
use chrono;
use crate::add_filter;
use crate::membership::models::membership::Membership;


async fn get_by_id(
    pool: &MySqlPool,
    id: i32,
) -> Result<Subscription, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM subscriptions WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(Subscription::from_row(&row))
}

pub async fn get_subscription_by_client_id(
    pool: &MySqlPool,
    client_id: i32,
    discipline_id: i32,
) -> Result<Option<Subscription>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT * FROM subscriptions
        WHERE client_id = ?
        AND discipline_id = ?
        "#,
    )
    .bind(client_id)
    .bind(discipline_id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        let subscription = Subscription::from_row(&row);
        Ok(Some(subscription))
    } else {
        Ok(None)
    }
}

pub async fn get_all_client_subscriptions(
    pool: &MySqlPool,
    client_id: i32,
) -> Result<Vec<Subscription>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT * FROM subscriptions
        WHERE client_id = ?
        "#,
    )
    .bind(client_id)
    .fetch_all(pool)
    .await?;

    let subscriptions = rows
        .into_iter()
        .map(|row| Subscription::from_row(&row))
        .collect();

    Ok(subscriptions)
}

pub async fn get_subscription_by_id_handler(
    pool: &MySqlPool,
    id: i32,
) -> Result<Option<Subscription>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM subscriptions WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = row {
        let subscription = Subscription::from_row(&row);
        Ok(Some(subscription))
    } else {
        Ok(None)
    }
}

pub async fn get_all_subscriptions_handler(
    pool: &MySqlPool,
) -> Result<Vec<Subscription>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM subscriptions")
        .fetch_all(pool)
        .await?;

    let mut subscriptions = Vec::new();
    for row in rows {
        subscriptions.push(Subscription::from_row(&row));
    }
    Ok(subscriptions)
}

pub async fn create_subscription_handler(
    pool: &MySqlPool,
    req: &NewSubscriptionRequest,
    membership: &Membership,
) -> Result<Subscription, sqlx::Error> {
    let now = chrono::Utc::now().naive_utc();
    let expires_at = now + chrono::Duration::days(membership.duration_days as i64);
    let remaining_classes = membership.total_classes;

    // 1. Insertar
    let result = sqlx::query(
        r#"
        INSERT INTO subscriptions (client_id, discipline_id, remaining_classes, expires_at, active)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(req.client_id)
    .bind(membership.discipline_id)
    .bind(remaining_classes)
    .bind(expires_at)
    .bind(true)
    .execute(pool)
    .await?;

    // 2. Obtener el ID recién insertado
    let inserted_id = result.last_insert_id();
    // 3. Buscar el registro completo
    let subscription = get_by_id(pool, inserted_id as i32).await?;

    Ok(subscription)
}

pub async fn update_subscription_handler(
    pool: &MySqlPool,
    subscription_id: i32,
    remaining_classes: i32,
    expires_at: chrono::NaiveDateTime,
) -> Result<Subscription, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE subscriptions
        SET remaining_classes = ?, expires_at = ?, updated_at = NOW(), active = ?, deleted_at = NULL
        WHERE id = ?
        "#,
    )
    .bind(remaining_classes)
    .bind(expires_at)
    .bind(true)
    .bind(subscription_id)
    .execute(pool)
    .await?;

    // 3. Buscar el registro completo
    let subscription = get_by_id(pool, subscription_id).await?;

    Ok(subscription)
}

pub async fn get_subscription_by_query_params_handler(
    pool: &MySqlPool,
    params: SubscriptionQueryParams,
) -> Result<Vec<Subscription>, sqlx::Error> {
    let mut query = String::from("SELECT * FROM subscriptions WHERE 1=1");
    let mut args = MySqlArguments::default();

    add_filter!(query, args, &params.client_id, " AND client_id = ?");
    add_filter!(query, args, &params.discipline_id, " AND discipline_id = ?");
    add_filter!(query, args, &params.active, " AND active = ?");
    add_filter!(query, args, &params.expires_at, " AND expires_at = ?");
    add_filter!(query, args, &params.created_at, " AND created_at = ?");
    add_filter!(query, args, &params.updated_at, " AND updated_at = ?");
    add_filter!(query, args, &params.deleted_at, " AND deleted_at = ?");
    add_filter!(query, args, &params.created_at_from, " AND created_at >= ?");
    add_filter!(query, args, &params.created_at_to, " AND created_at <= ?");
    add_filter!(query, args, &params.updated_at_from, " AND updated_at >= ?");
    add_filter!(query, args, &params.updated_at_to, " AND updated_at <= ?");
    add_filter!(query, args, &params.deleted_at_from, " AND deleted_at >= ?");
    add_filter!(query, args, &params.deleted_at_to, " AND deleted_at <= ?");
    add_filter!(query, args, &params.expires_at_from, " AND expires_at >= ?");
    add_filter!(query, args, &params.expires_at_to, " AND expires_at <= ?");

    let rows = sqlx::query_with(&query, args)
        .fetch_all(pool)
        .await?;

    Ok(rows.iter().map(Subscription::from_row).collect())
}

pub async fn delete_subscription_handler(
    pool: &MySqlPool,
    id: i32,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE subscriptions
        SET deleted_at = NOW(), active = false, remaining_classes = 0
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result)
}