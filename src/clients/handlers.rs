use sqlx::mysql::MySqlQueryResult;
use sqlx::{self, MySqlPool};
use super::models::clients::Client;


pub async fn create_client_in_db(
    pool: &MySqlPool,
    name: &str,
    last_name: &str,
    age: i32,
    phone: &str,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO clients (name, last_name, age, phone, active) VALUES (?, ?, ?, ?, ?)
        "#,
        name,
        last_name,
        age,
        phone,
        true // active by default
    )
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
