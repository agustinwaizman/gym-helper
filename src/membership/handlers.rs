use sqlx::mysql::MySqlQueryResult;
use sqlx::{self, MySqlPool};
use super::models::requests::{NewMembershipRequest, NewDisciplineRequest};


pub async fn create_discipline_in_db(
    pool: &MySqlPool,
    req: NewDisciplineRequest,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO disciplines (name, description) VALUES (?, ?)
        "#
    )
    .bind(req.name)
    .bind(req.description)
    .execute(pool)
    .await;

    result
}

pub async fn create_membership_in_db(
    pool: &MySqlPool,
    req: NewMembershipRequest,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO memberships (name, description, price, discipline_id, total_classes, active, duration_days) 
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(req.name)
    .bind(req.description)
    .bind(req.price)
    .bind(req.discipline_id)
    .bind(req.total_classes)
    .bind(req.active)
    .bind(req.duration_days)
    .execute(pool)
    .await;

    result
}