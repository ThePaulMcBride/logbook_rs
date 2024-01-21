mod commands;
mod data;
mod logbook;

use commands::{Cli, Commands, Parser};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let args = Cli::parse();
    match args.command {
        Commands::List => commands::list_logs().await,
        Commands::Add => commands::create_log().await?,
    }

    Ok(())
}
