use sqlx::mysql::MySqlQueryResult;
use sqlx::{self, MySqlPool};
use crate::auth::models::{User, UserRole};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, PasswordHash, PasswordVerifier, rand_core::OsRng};


fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    match parsed_hash {
        Ok(ph) => Argon2::default()
            .verify_password(password.as_bytes(), &ph)
            .is_ok(),
        Err(_) => false,
    }
}

pub async fn create_user_in_db(
    pool: &MySqlPool,
    username: &str,
    password: &str,
    role: &str,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let hashed_password = match hash_password(password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Error hashing password: {}", e);
            return Err(sqlx::Error::ColumnNotFound("Error hashing password".to_string()));
        }
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, hash, role) VALUES (?, ?, ?)
        "#,
        username,
        hashed_password,
        role
    )
    .execute(pool)
    .await;

    result
}

pub async fn get_user_by_username(
    pool: &MySqlPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            hash,
            role as "role: UserRole",
            created_at,
            updated_at
        FROM users
        WHERE username = ?
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}
