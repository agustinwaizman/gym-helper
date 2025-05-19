use sqlx::MySqlPool;

pub async fn validate_client(
    pool: &MySqlPool,
    client_id: i32
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT * FROM subscriptions
        WHERE client_id = ?
        "#,
    )
    .bind(client_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(_) => Ok(true),
        None => Ok(false)
    }
}

pub async fn validate_membership(
    pool: &MySqlPool,
    membership_id: i32
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT * FROM memberships
        WHERE id = ?
        "#,
    )
    .bind(membership_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(_) => Ok(true),
        None => Ok(false)
    }
}