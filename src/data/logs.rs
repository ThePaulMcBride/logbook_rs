use std::fmt::Display;

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

pub struct TotalHours {
    pub pic: String,
    pub put: String,
    pub total: String,
}

impl Display for TotalHours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PIC: {}\nPUT: {}\nTotal: {}",
            self.pic, self.put, self.total
        )
    }
}

pub async fn get_total_hours() -> Result<TotalHours, sqlx::Error> {
    let mut conn = super::get_connection().await?;

    let total_hours = sqlx::query!(
        r#"
        SELECT duration, holders_capacity FROM logs
        "#,
    )
    .fetch_all(&mut conn)
    .await?;

    let mut pic_mins = 0;
    let mut put_mins = 0;
    let mut total = 0;

    for log in total_hours {
        // parse minutes from hh:mm format
        let duration = log.duration.split(":").collect::<Vec<&str>>();
        let hours = duration[0].parse::<i32>().unwrap();
        let minutes = duration[1].parse::<i32>().unwrap();
        let total_minutes = (hours * 60) + minutes;

        if log.holders_capacity == "PIC" {
            pic_mins += total_minutes;
        } else {
            put_mins += total_minutes;
        }
        total += total_minutes;
    }

    let pic_hours = pic_mins / 60;
    let pic_remainder = pic_mins % 60;
    let put_hours = put_mins / 60;
    let put_remainder = put_mins % 60;
    let total_hours = total / 60;
    let total_remainder = total % 60;

    let pic = format!("{:02}:{:02}", pic_hours, pic_remainder);
    let put = format!("{:02}:{:02}", put_hours, put_remainder);
    let total = format!("{:02}:{:02}", total_hours, total_remainder);

    let total_hours = TotalHours { pic, put, total };

    Ok(total_hours)
}
