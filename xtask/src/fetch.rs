use serde::{Deserialize, Serialize};
use tracing::info;

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

    loop {
        let resp = client
            .get("https://cloud.timeedit.net/liu/web/schema/objects/o.json")
            .query(&[
                ("types", TEACHER),
                ("sid", "3"),
                ("max", &max_str),
                ("start", &start.to_string()),
            ])
            .send()
            .await?;

        let mut data: ObjectSearchResponse = resp.json().await?;

        if data.count == 0 {
            break;
        }

        if let Some(first) = data.records.first() {
            info!("start={}: {}", start, first.name);
        }

        all_objects.append(&mut data.records);

        start += MAX as usize;
    }

    Ok(all_objects)
}
