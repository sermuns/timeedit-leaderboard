use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::{cmp, fs::File, io::BufReader};

use crate::{
    fetch::{TeacherObject, fetch_teachers},
    leaderboard::LeaderboardEntry,
};

mod fetch;
mod html;
mod leaderboard;

pub const FETCH_CONCURRENCY: usize = 1000;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    FetchObjects,
    FetchLeaderboard,
    DumpLeaderboardBin,
    /// Debug print leaderboard
    Print,
    /// Write leaderboard as static HTML
    Html,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(cli.verbosity)
        .init();

    match &cli.command {
        Commands::FetchObjects => {
            let objects = fetch_teachers().await?;
            let binary_data = postcard::to_allocvec(&objects)?;
            std::fs::write("objects.bin", binary_data)?;
        }
        Commands::FetchLeaderboard => {
            let objects: Vec<TeacherObject> = postcard::from_bytes(&std::fs::read("objects.bin")?)?;
            let mut leaderboard = leaderboard::generate_leaderboard(objects).await?;
            leaderboard.sort_by_key(|e| cmp::Reverse(e.num_bookings));
            std::fs::write("leaderboard.bin", postcard::to_allocvec(&leaderboard)?)?;
        }
        Commands::DumpLeaderboardBin => {
            let leaderboard_json_file_reader = BufReader::new(File::open("leaderboard.json")?);
            let mut leaderboard: Vec<LeaderboardEntry> =
                serde_json::from_reader(leaderboard_json_file_reader)?;
            leaderboard.sort_by_key(|e| cmp::Reverse(e.num_bookings));
            std::fs::write("leaderboard.bin", postcard::to_allocvec(&leaderboard)?)?;
        }
        Commands::Print => {
            let leaderboard: Vec<LeaderboardEntry> =
                postcard::from_bytes(&std::fs::read("leaderboard.bin")?)?;

            eprintln!("{:#?}", leaderboard);
        }
        Commands::Html => {
            let leaderboard: Vec<LeaderboardEntry> =
                postcard::from_bytes(&std::fs::read("leaderboard.bin")?)?;

            std::fs::write("index.html", html::generate_html(leaderboard).into_string())?;
        }
    }

    Ok(())
}
