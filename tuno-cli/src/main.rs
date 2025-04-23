use dotenv::dotenv;
use anyhow::Result;
use clap::Parser;

use tuno_cli::tuno_commands::TunoCommands;

#[derive(Parser)]
#[command(
    name = env!("CARGO_BIN_NAME"),
    about = "Client for interacting with the Tuno Media network",
)]
struct Args {
    #[command(subcommand)]
    command: TunoCommands,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();
    env_logger::init();
    args.command.execute().await?;
    Ok(())
}
