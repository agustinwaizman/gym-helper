use sqlx::mysql::MySqlQueryResult;
use sqlx::{self, MySqlPool};


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
