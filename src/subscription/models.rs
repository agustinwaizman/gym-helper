use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, MySqlPool, Row};
use chrono::NaiveDateTime;

use crate::membership::models::membership::Membership;
use async_trait::async_trait;


#[derive(Serialize, Deserialize)]
pub struct Subscription {
    pub id: i32,
    pub client_id: i32,
    pub discipline_id: i32,
    pub remaining_classes: i32,
    pub expires_at: NaiveDateTime,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Subscription {
    pub fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            client_id: row.get("client_id"),
            discipline_id: row.get("discipline_id"),
            remaining_classes: row.get("remaining_classes"),
            expires_at: row.get("expires_at"),
            active: row.get::<i8, _>("active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        }
    }

    pub async fn create(
        pool: &MySqlPool,
        req: &NewSubscriptionRequest,
        membership: &Membership,
    ) -> Result<Self, sqlx::Error> {
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
        let row = sqlx::query("SELECT * FROM subscriptions WHERE id = ?")
            .bind(inserted_id)
            .fetch_one(pool)
            .await?;

        Ok(Subscription::from_row(&row))
    }

    pub async fn update(
        &self,
        pool: &MySqlPool,
        remaining_classes: i32,
        expires_at: NaiveDateTime,
    ) -> Result<Self, sqlx::Error> {
        // 1. Ejecutar el UPDATE
        sqlx::query(
            r#"
            UPDATE subscriptions
            SET remaining_classes = ?, expires_at = ?, updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(remaining_classes)
        .bind(expires_at)
        .bind(self.id)
        .execute(pool)
        .await?;

        // 2. Obtener el registro actualizado
        let row = sqlx::query("SELECT * FROM subscriptions WHERE id = ?")
            .bind(self.id)
            .fetch_one(pool)
            .await?;

        Ok(Subscription::from_row(&row))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSubscriptionRequest{
    pub client_id: i32,
    pub membership_id: i32,
}

#[async_trait]
pub trait ValidateRequest {
    async fn validate(&self, pool: &MySqlPool) -> Result<(), String>;
}

#[async_trait]
impl ValidateRequest for NewSubscriptionRequest {
    async fn validate(&self, pool: &MySqlPool) -> Result<(), String> {
        // Validar existencia del cliente
        let client_exists = sqlx::query(
            r#"
            SELECT 1 FROM clients WHERE id = ?
            "#,
        )
        .bind(self.client_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error validating client ID: {}", e))?;

        if client_exists.is_none() {
            return Err("Invalid client ID".to_string());
        }

        // Validar existencia de la membresía
        let membership_exists = sqlx::query(
            r#"
            SELECT 1 FROM memberships WHERE id = ?
            "#,
        )
        .bind(self.membership_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error validating membership ID: {}", e))?;

        if membership_exists.is_none() {
            return Err("Invalid membership ID".to_string());
        }

        Ok(())
    }
}
