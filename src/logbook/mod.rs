use crate::data;
use cli_table::{print_stdout, Cell, Style, Table};

pub struct Logbook {}

impl Logbook {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn add_log(log: Log) -> Result<(), sqlx::Error> {
        data::create_log(log).await
    }

    pub async fn list(&self) {
        // let table = vec![
        //     vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
        //     vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
        //     vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
        // ]
        // .table()
        // .title(vec![
        //     "Name".cell().bold(true),
        //     "Age (in years)".cell().bold(true),
        // ])
        // .bold(true);

        // print_stdout(table).unwrap();

        let logs = data::list_logs().await.unwrap();

        let table = logs
            .into_iter()
            .map(|log| {
                vec![
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
