use sqlx::{self, MySqlPool};
use super::models::{Subscription, NewSubscriptionRequest};


pub async fn get_subscription_by_client_id(
    pool: &MySqlPool,
    client_id: i32,
    discipline_id: i32,
) -> Result<Option<Subscription>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT * FROM subscriptions
        WHERE client_id = ? AND discipline_id = ?
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
