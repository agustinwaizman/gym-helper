use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, MySqlPool, Row};
use chrono::NaiveDateTime;
use utoipa::ToSchema;
use super::handlers::get_subscription_by_id_handler;


#[derive(Serialize, Deserialize, ToSchema)]
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

#[derive(Serialize, Deserialize, ToSchema)]
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ClassAttendance {
    pub id: i32,
    pub subscription_id: i32,
    pub attended_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"subscription_id": 1}))]
pub struct ClassAttendanceRequest {
    pub subscription_id: i32,
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

    pub fn validate_if_active(&self) -> Result<(), String> {
        if !self.active {
            return Err("Subscription is not active".to_string());
        }
        if self.remaining_classes <= 0 {
            return Err("No remaining classes".to_string());
        }
        if self.expires_at < chrono::Utc::now().naive_utc() {
            return Err("Subscription expired".to_string());
        }
        Ok(())
    }

    pub async fn expire_subscription(&self, pool: &MySqlPool) -> Result<(), String> {
        sqlx::query(
            r#"
            UPDATE subscriptions
            SET active = 0, deleted_at = NOW(), expires_at = NOW(), remaining_classes = 0
            WHERE id = ?
            "#,
        )
        .bind(self.id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error expiring subscription: {}", e))?;

        Ok(())
    }

    async fn has_attendance_today(&self, pool: &MySqlPool) -> Result<bool, String> {
        let today = chrono::Utc::now().naive_utc();
        let attendance_exists = sqlx::query(
            r#"
            SELECT 1 FROM class_attendance
            WHERE subscription_id = ?
            AND DATE(attended_at) = DATE(?)
            "#,
        )
        .bind(self.id)
        .bind(today)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error validating attendance: {}", e))?;

        Ok(attendance_exists.is_some())
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(example = json!({"client_id": 1, "membership_id": 1}))]
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

impl ClassAttendanceRequest {
    pub async fn validate(&self, pool: &MySqlPool) -> Result<Subscription, String> {
        let subscription = get_subscription_by_id_handler(pool, self.subscription_id)
            .await
            .map_err(|e| format!("Error fetching subscription: {}", e))?
            .ok_or_else(|| "Subscription ID doesn't exists".to_string())?;

        if let Err(e) = subscription.validate_if_active() {
            subscription
                .expire_subscription(pool)
                .await
                .map_err(|e| format!("Error expiring subscription: {}", e))?;
            tracing::error!("Subscription not valid: {}", e);
            return Err(format!("Subscription not valid: {}", e));
        }
        if subscription.has_attendance_today(pool).await? {
            return Err("Attendance already registered today".to_string());
        }
        Ok(subscription)
    }
}