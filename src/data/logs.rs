use crate::logbook::{IdentifiableLog, Log};

pub async fn create_log(log: Log) -> Result<(), sqlx::Error> {
    let mut conn = super::get_connection().await?;

    sqlx::query!(
        r#"
        INSERT INTO logs (
            date,
            aircraft_type,
            aircraft_registration,
            captain,
            holders_capacity,
            from_location,
            to_location,
            duration
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        log.date,
        log.aircraft_type,
        log.aircraft_registration,
        log.captain,
        log.holders_capacity,
        log.from_location,
        log.to_location,
        log.duration,
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn get_log(id: u32) -> Result<IdentifiableLog, sqlx::Error> {
    let mut conn = super::get_connection().await?;

    let log = sqlx::query_as!(
        IdentifiableLog,
        r#"
        SELECT
            id,
            date,
            aircraft_type,
            aircraft_registration,
            captain,
            holders_capacity,
            from_location,
            to_location,
            duration
        FROM logs
        WHERE id=$1
        "#,
        id
    )
    .fetch_one(&mut conn)
    .await?;

    Ok(log)
}

pub async fn update_log(id: u32, log: Log) -> Result<(), sqlx::Error> {
    let mut conn = super::get_connection().await?;

    sqlx::query!(
        r#"
        UPDATE logs SET
            date=$1,
            aircraft_type=$2,
            aircraft_registration=$3,
            captain=$4,
            holders_capacity=$5,
            from_location=$6,
            to_location=$7,
            duration=$8
        WHERE id=$9
        "#,
        log.date,
        log.aircraft_type,
        log.aircraft_registration,
        log.captain,
        log.holders_capacity,
        log.from_location,
        log.to_location,
        log.duration,
        id
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn delete_log(id: u32) -> Result<(), sqlx::Error> {
    let mut conn = super::get_connection().await?;

    sqlx::query!(
        r#"
        DELETE FROM logs WHERE id=$1
        "#,
        id
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn list_logs() -> Result<Vec<IdentifiableLog>, sqlx::Error> {
    let mut conn = super::get_connection().await?;

    let logs = sqlx::query_as!(
        IdentifiableLog,
        r#"
        SELECT
            id,
            date,
            aircraft_type,
            aircraft_registration,
            captain,
            holders_capacity,
            from_location,
            to_location,
            duration
        FROM logs
        ORDER BY date ASC
        "#,
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(logs)
}
