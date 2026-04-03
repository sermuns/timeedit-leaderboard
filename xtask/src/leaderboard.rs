use futures::{StreamExt, TryStreamExt, stream};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{FETCH_CONCURRENCY, fetch::TeacherObject};

#[derive(Debug, Deserialize, Serialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub object_id: u32,
    pub num_bookings: u32,
}

#[derive(Debug, Deserialize)]
pub struct CalendarResponse {
    info: Info,
    // pub reservations: Vec<Reservation>,
}

#[derive(Debug, Deserialize)]
struct Info {
    #[serde(rename = "reservationcount")]
    count: u32,
}

// #[derive(Debug, Deserialize)]
// struct Reservation {
//     pub id: String,
//     pub startdate: String,
//     pub starttime: String,
//     pub enddate: String,
//     pub endtime: String,
//
//     pub columns: [String; 9],
// }

const BOOKINGS_BASE_URL: &str =
    "https://cloud.timeedit.net/liu/web/schema/ri.json?p=20250101,20270101&objects=";

pub async fn generate_leaderboard(
    objects: Vec<TeacherObject>,
) -> anyhow::Result<Vec<LeaderboardEntry>> {
    let client = reqwest::Client::new();

    let leaderboard = stream::iter(objects)
        .map(|o| {
            let client = client.clone();
            async move {
                let response: CalendarResponse = client
                    .get(format!("{}{}", BOOKINGS_BASE_URL, o.id))
                    .send()
                    .await?
                    .json()
                    .await?;

                info!("found {} bookings for {}", response.info.count, o.name);

                anyhow::Ok(LeaderboardEntry {
                    name: o.name,
                    object_id: o.id,
                    num_bookings: response.info.count,
                })
            }
        })
        .buffer_unordered(FETCH_CONCURRENCY)
        .try_collect()
        .await?;

    Ok(leaderboard)
}
