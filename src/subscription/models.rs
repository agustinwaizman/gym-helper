use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, MySqlPool, Row};
use chrono::NaiveDateTime;


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

#[derive(Serialize, Deserialize)]
pub struct SubscriptionQueryParams {
    pub client_id: Option<i32>,
    pub discipline_id: Option<i32>,
    pub active: Option<bool>,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at_from: Option<NaiveDateTime>,
    pub created_at_to: Option<NaiveDateTime>,
    pub updated_at_from: Option<NaiveDateTime>,
    pub updated_at_to: Option<NaiveDateTime>,
    pub deleted_at_from: Option<NaiveDateTime>,
    pub deleted_at_to: Option<NaiveDateTime>,
    pub expires_at_from: Option<NaiveDateTime>,
    pub expires_at_to: Option<NaiveDateTime>,
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSubscriptionRequest{
    pub client_id: i32,
    pub membership_id: i32,
}

impl NewSubscriptionRequest {
    pub async fn validate(&self, pool: &MySqlPool) -> Result<(), String> {
        // Validar existencia del cliente
        let client_exists = sqlx::query(
            r#"
            SELECT 1 FROM clients WHERE id = ? AND active = 1
            "#,
        )
        .bind(self.client_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error validating client ID: {}", e))?;

        if client_exists.is_none() {
            return Err("Client ID doesn't exists or not is active".to_string());
        }

        // Validar existencia de la membresía
        let membership_exists = sqlx::query(
            r#"
            SELECT 1 FROM memberships WHERE id = ? AND active = 1
            "#,
        )
        .bind(self.membership_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error validating membership ID: {}", e))?;

        if membership_exists.is_none() {
            return Err("Membership ID doesn't exists or not is active".to_string());
        }

        Ok(())
    }
}
