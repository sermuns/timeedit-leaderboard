use serde::{Deserialize, Serialize};
use tracing::info;

use crate::FETCH_CONCURRENCY;

#[derive(Deserialize, Debug)]
pub struct ObjectSearchResponse {
    pub count: u16,
    pub records: Vec<TeacherObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherObject {
    pub id: u32,
    #[serde(rename = "values")]
    pub name: String,
}

pub async fn fetch_teachers() -> anyhow::Result<Vec<TeacherObject>> {
    let client = reqwest::Client::new();

    let mut all_objects = Vec::new();
    const TEACHER: &str = "184";
    const MAX: u16 = 100;
    let max_str = MAX.to_string();

    let mut start = 0;
    let mut tasks = Vec::new();

    // Launch initial batch of concurrent requests
    for i in 0..FETCH_CONCURRENCY {
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
            Ok((current_start, data))
        });

        tasks.push(task);
    }

    start += FETCH_CONCURRENCY * MAX as usize;

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

    Ok(all_objects)
}
