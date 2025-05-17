use sqlx::mysql::MySqlQueryResult;
use sqlx::{self, MySqlPool};
use super::models::requests::{NewMembershipRequest, NewDisciplineRequest};

/////////////////////////////////////////////////////////////////////////////////
/////////////////// DISCIPLINE HANDLERS //////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

pub async fn create_discipline_handler(
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

pub async fn delete_discipline_handler(
    pool: &MySqlPool,
    id: i32,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE disciplines
        SET deleted_at = NOW()
        WHERE id = ?
        "#
    )
    .bind(id)
    .execute(pool)
    .await;

    result
}

pub async fn activate_discipline_handler(
    pool: &MySqlPool,
    id: i32,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE disciplines
        SET deleted_at = NULL
        WHERE id = ?
        "#
    )
    .bind(id)
    .execute(pool)
    .await;

    result
}

/////////////////////////////////////////////////////////////////////////////////
/////////////////// MEMBERSHIP HANDLERS //////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

pub async fn create_membership_handler(
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
    .bind(true) // Assuming active is always true when creating a new membership
    .bind(req.duration_days)
    .execute(pool)
    .await;

    result
}

pub async fn delete_membership_by_discipline_handler(
    pool: &MySqlPool,
    discipline_id: i32,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE memberships
        SET active = false, deleted_at = NOW()
        WHERE discipline_id = ?
        "#
    )
    .bind(discipline_id)
    .execute(pool)
    .await;

    result
}
