use crate::logbook::Log;

pub async fn create_log(log: Log) -> Result<(), sqlx::Error> {
    let mut conn = super::get_connection().await?;

    sqlx::query(
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
    )
    .bind(log.date)
    .bind(log.aircraft_type)
    .bind(log.aircraft_registration)
    .bind(log.captain)
    .bind(log.holders_capacity)
    .bind(log.from_location)
    .bind(log.to_location)
    .bind(log.duration)
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn list_logs() -> Result<Vec<Log>, sqlx::Error> {
    let mut conn = super::get_connection().await?;

    let logs = sqlx::query_as::<_, Log>(
        r#"
        SELECT
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
