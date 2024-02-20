use crate::data::{self, TotalHours};
use cli_table::{print_stdout, Cell, Style, Table};

pub struct Logbook {}

impl Logbook {
    pub async fn add_log(log: Log) -> Result<(), sqlx::Error> {
        data::create_log(log).await
    }

    pub async fn get_log(id: u32) -> Result<IdentifiableLog, sqlx::Error> {
        data::get_log(id).await
    }

    pub async fn update_log(id: u32, log: Log) -> Result<(), sqlx::Error> {
        data::update_log(id, log).await
    }

    pub async fn list() {
        let logs = data::list_logs().await.unwrap();

        let table = logs
            .into_iter()
            .map(|log| {
                vec![
                    log.id.cell(),
                    log.date.cell(),
                    log.aircraft_type.cell(),
                    log.aircraft_registration.cell(),
                    log.captain.cell(),
                    log.holders_capacity.cell(),
                    log.from_location.cell(),
                    log.to_location.cell(),
                    log.duration.cell(),
                ]
            })
            .table()
            .title(vec![
                "ID".cell().bold(true),
                "Date".cell().bold(true),
                "Aircraft Type".cell().bold(true),
                "Aircraft Registration".cell().bold(true),
                "Captain".cell().bold(true),
                "Holders Capacity".cell().bold(true),
                "From".cell().bold(true),
                "To".cell().bold(true),
                "Duration".cell().bold(true),
            ])
            .bold(true);

        print_stdout(table).unwrap();
    }

    pub async fn delete_log(id: u32) -> Result<(), sqlx::Error> {
        data::delete_log(id).await
    }

    pub async fn total_hours() -> Result<TotalHours, sqlx::Error> {
        data::get_total_hours().await
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Log {
    pub date: String,
    pub aircraft_type: String,
    pub aircraft_registration: String,
    pub captain: String,
    pub holders_capacity: String,
    pub from_location: String,
    pub to_location: String,
    pub duration: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct IdentifiableLog {
    pub id: i64,
    pub date: String,
    pub aircraft_type: String,
    pub aircraft_registration: String,
    pub captain: String,
    pub holders_capacity: String,
    pub from_location: String,
    pub to_location: String,
    pub duration: String,
}

impl Log {
    pub fn new(
        date: String,
        aircraft_type: String,
        aircraft_registration: String,
        captain: String,
        holders_capacity: String,
        from_location: String,
        to_location: String,
        duration: String,
    ) -> Self {
        Self {
            date,
            aircraft_type,
            aircraft_registration,
            captain,
            holders_capacity,
            from_location,
            to_location,
            duration,
        }
    }
}
