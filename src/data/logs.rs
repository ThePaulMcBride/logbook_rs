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
