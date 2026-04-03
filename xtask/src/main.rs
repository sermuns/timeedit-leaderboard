use clap::{Parser, Subcommand};
use postcard::to_allocvec;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::teachers::{TeacherObject, fetch_teachers};

mod teachers;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Load,
    Fetch,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let log_env = match cli.verbosity {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    unsafe { std::env::set_var("RUST_LOG", log_env) };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    match &cli.command {
        Commands::Load => {
            let data = std::fs::read("objects.bin")?;
            let objects: Vec<TeacherObject> = postcard::from_bytes(&data)?;
            info!("loaded {} objects", objects.len());
            // std::fs::write("objects.json", serde_json::to_string_pretty(&objects)?)?;
        }
        Commands::Fetch => {
            let all_objects = fetch_teachers().await?;
            let binary_data: Vec<u8> = to_allocvec(&all_objects)?;
            std::fs::write("objects.bin", binary_data)?;
        }
    }

    Ok(())
}
