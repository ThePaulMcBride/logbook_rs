mod logs;

pub use logs::{
    create_log, delete_log, get_log, get_total_hours, list_logs, update_log, TotalHours,
};
use sqlx::{migrate, Connection, SqliteConnection};

pub async fn get_connection() -> Result<sqlx::SqliteConnection, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        let mut fallback =
            std::env::var("XDG_DATA_HOME").expect("DATABASE_URL or XDG_DATA_HOME must be set");

        fallback.push_str("/logbook.db?mode=rwc");
        fallback
    });

    let mut conn = SqliteConnection::connect(&database_url).await?;

    migrate!("db/migrations").run(&mut conn).await?;

    Ok(conn)
}
