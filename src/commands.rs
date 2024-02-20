use chrono::NaiveDate;
pub use clap::Parser;
use clap::{command, Subcommand};
use inquire::validator::Validation;
use inquire::{DateSelect, Select, Text};
use regex::Regex;

use crate::logbook::{Log, Logbook};

#[derive(Debug, Parser)]
#[command(name = "logbook")]
#[command(about = "A cli tool for recording your flights", long_about = None)]
#[command(bin_name = "logbook")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List all entries in your logbook
    List,
    /// Add a new entry to your logbook
    Add,
    /// Delete an entry
    Delete {
        /// ID of the entry to delete
        id: u32,
    },
    /// Update an entry
    Update {
        /// ID of the entry to update
        id: u32,
    },
    /// List total hours flown
    Total,
}

pub async fn create_log() -> Result<(), sqlx::Error> {
    let date_input = DateSelect::new("When did you fly?").prompt();
    let date = match date_input {
        Ok(date) => date.to_string(),
        Err(_) => panic!("No input provided"),
    };

    let duration_input = Text::new("How long did you fly for?")
        .with_validator(|d: &str| {
            let re = Regex::new("[0-9]{2}:[0-6]{1}[0-9]{1}").unwrap();

            match re.is_match(&d) {
                true => Ok(Validation::Valid),
                false => Ok(Validation::Invalid(
                    "Please use HH:MM format eg. 01:20".into(),
                )),
            }
        })
        .prompt();
    let duration = match duration_input {
        Ok(duration) => duration,
        Err(_) => panic!("No input provided"),
    };

    let aircraft_type_input = Text::new("What type aircraft did you fly?").prompt();
    let aircraft_type = match aircraft_type_input {
        Ok(aircraft_type) => aircraft_type,
        Err(_) => panic!("No input provided"),
    };

    let aircraft_registration_input = Text::new("What was the aircraft registration?").prompt();
    let aircraft_registration = match aircraft_registration_input {
        Ok(aircraft_registration) => aircraft_registration,
        Err(_) => panic!("No input provided"),
    };

    let captain_input = Text::new("Who was the captain?").prompt();
    let captain = match captain_input {
        Ok(captain) => captain,
        Err(_) => panic!("No input provided"),
    };

    let holders_capacity_input =
        Select::new("What was the holders capacity?", vec!["PIC", "PUT"]).prompt();
    let holders_capacity = match holders_capacity_input {
        Ok(holders_capacity) => holders_capacity,
        Err(_) => panic!("No input provided"),
    };

    let from_input = Text::new("Where did you fly from?").prompt();
    let from = match from_input {
        Ok(from) => from,
        Err(_) => panic!("No input provided"),
    };

    let to_input = Text::new("Where did you fly to?").prompt();
    let to = match to_input {
        Ok(to) => to,
        Err(_) => panic!("No input provided"),
    };

    let log = Log::new(
        date,
        aircraft_type,
        aircraft_registration,
        captain,
        holders_capacity.into(),
        from,
        to,
        duration,
    );

    Logbook::add_log(log).await
}

pub async fn list_logs() {
    Logbook::list().await;
}

pub async fn delete_log(id: u32) -> Result<(), sqlx::Error> {
    return Logbook::delete_log(id).await;
}

pub async fn update_log(id: u32) -> Result<(), sqlx::Error> {
    let original_log = Logbook::get_log(id)
        .await
        .expect("No log exists with that ID");

    let default_date = NaiveDate::parse_from_str(&original_log.date, "%Y-%m-%d").unwrap();
    let date_input = DateSelect::new("When did you fly?")
        .with_default(default_date)
        .prompt();

    let date = match date_input {
        Ok(date) => date.to_string(),
        Err(_) => panic!("No input provided"),
    };

    let default_duration = original_log.duration;
    let duration_input = Text::new("How long did you fly for?")
        .with_default(&default_duration)
        .with_validator(|d: &str| {
            let re = Regex::new("[0-9]{2}:[0-6]{1}[0-9]{1}").unwrap();

            match re.is_match(&d) {
                true => Ok(Validation::Valid),
                false => Ok(Validation::Invalid(
                    "Please use HH:MM format eg. 01:20".into(),
                )),
            }
        })
        .prompt();

    let duration = match duration_input {
        Ok(duration) => duration,
        Err(_) => panic!("No input provided"),
    };

    let default_aircraft_type = original_log.aircraft_type;
    let aircraft_type_input = Text::new("What type aircraft did you fly?")
        .with_default(&default_aircraft_type)
        .prompt();

    let aircraft_type = match aircraft_type_input {
        Ok(aircraft_type) => aircraft_type,
        Err(_) => panic!("No input provided"),
    };

    let default_aircraft_registration = original_log.aircraft_registration;
    let aircraft_registration_input = Text::new("What was the aircraft registration?")
        .with_default(&default_aircraft_registration)
        .prompt();

    let aircraft_registration = match aircraft_registration_input {
        Ok(aircraft_registration) => aircraft_registration,
        Err(_) => panic!("No input provided"),
    };

    let default_captain = original_log.captain;
    let captain_input = Text::new("Who was the captain?")
        .with_default(&default_captain)
        .prompt();

    let captain = match captain_input {
        Ok(captain) => captain,
        Err(_) => panic!("No input provided"),
    };

    let holders_capacity_input =
        Select::new("What was the holders capacity?", vec!["PIC", "PUT"]).prompt();

    let holders_capacity = match holders_capacity_input {
        Ok(holders_capacity) => holders_capacity,
        Err(_) => panic!("No input provided"),
    };

    let default_from = original_log.from_location;
    let from_input = Text::new("Where did you fly from?")
        .with_default(&default_from)
        .prompt();

    let from = match from_input {
        Ok(from) => from,
        Err(_) => panic!("No input provided"),
    };

    let default_to = original_log.to_location;
    let to_input = Text::new("Where did you fly to?")
        .with_default(&default_to)
        .prompt();

    let to = match to_input {
        Ok(to) => to,
        Err(_) => panic!("No input provided"),
    };

    let log = Log::new(
        date,
        aircraft_type,
        aircraft_registration,
        captain,
        holders_capacity.into(),
        from,
        to,
        duration,
    );

    return Logbook::update_log(id, log).await;
}

pub async fn total_hours() -> Result<(), sqlx::Error> {
    let total_hours = Logbook::total_hours().await;

    match total_hours {
        Ok(total_hours) => {
            println!("{total_hours}");
            Result::Ok(())
        }
        Err(_) => {
            eprintln!("There was a problem getting the total hours.");
            Result::Err(sqlx::Error::RowNotFound)
        }
    }
}
