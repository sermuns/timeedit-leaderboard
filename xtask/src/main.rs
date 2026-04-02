use clap::Parser;
use postcard::to_allocvec;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Deserialize, Debug)]
pub struct ObjectSearchResponse {
    pub count: u16,
    pub records: Vec<ObjectRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectRecord {
    pub id: u32,
    #[serde(rename = "values")]
    pub name: String,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[arg(long)]
    load: bool,
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
    eprintln!("setting {}", log_env);
    unsafe { std::env::set_var("RUST_LOG", log_env) };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    if cli.load {
        let data = std::fs::read("objects.bin")?;
        let objects: Vec<ObjectRecord> = postcard::from_bytes(&data)?;
        info!("loaded {} objects", objects.len());
        std::fs::write("objects.json", serde_json::to_string_pretty(&objects)?)?;
        return Ok(());
    }

    let client = reqwest::Client::new();

    let mut all_objects = Vec::new();
    const TEACHER: &str = "184";
    const MAX: u16 = 100;
    const CONCURRENCY: usize = 30; // Number of concurrent requests
    let max_str = MAX.to_string();

    let mut start = 0;
    let mut tasks = Vec::new();

    // Launch initial batch of concurrent requests
    for i in 0..CONCURRENCY {
        let client = client.clone();
        let max_str = max_str.clone();
        let current_start = start + (i * MAX as usize);

        let task = tokio::spawn(async move {
            let resp = client
                .get("https://cloud.timeedit.net/liu/web/schema/objects/o.json")
                .query(&[
                    ("types", TEACHER),
                    ("sid", "3"),
                    ("max", &max_str),
                    ("start", &current_start.to_string()),
                ])
                .send()
                .await?;

            info!("fetched {}", resp.url());
            let data: ObjectSearchResponse = resp.json().await?;
            Ok::<_, anyhow::Error>((current_start, data))
        });

        tasks.push(task);
    }

    start += CONCURRENCY * MAX as usize;

    // Process results and launch new requests as needed
    while !tasks.is_empty() {
        let (result, _idx, remaining) = futures::future::select_all(tasks).await;
        tasks = remaining;

        let (batch_start, mut data) = result??;

        if data.count == 0 {
            info!("received empty response at start={}, stopping", batch_start);
            // Cancel remaining tasks by dropping them
            break;
        }

        if let Some(first) = data.records.first() {
            info!("batch start={}: {}", batch_start, first.name);
        }

        all_objects.append(&mut data.records);

        // Launch next request to maintain concurrency
        let client = client.clone();
        let max_str = max_str.clone();
        let current_start = start;

        let task = tokio::spawn(async move {
            let resp = client
                .get("https://cloud.timeedit.net/liu/web/schema/objects/o.json")
                .query(&[
                    ("types", TEACHER),
                    ("sid", "3"),
                    ("max", &max_str),
                    ("start", &current_start.to_string()),
                ])
                .send()
                .await?;

            info!("fetched {}", resp.url());
            let data: ObjectSearchResponse = resp.json().await?;
            Ok::<_, anyhow::Error>((current_start, data))
        });

        tasks.push(task);
        start += MAX as usize;
    }

    let data: Vec<u8> = to_allocvec(&all_objects)?;
    std::fs::write("objects.bin", data)?;

    Ok(())
}
