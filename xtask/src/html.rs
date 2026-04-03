use maud::{DOCTYPE, Markup, html};

use crate::leaderboard::LeaderboardEntry;

const TITLE: &str = "TimeEdit leaderboard";
const TAKE_NUM: usize = 100;

pub fn generate_html(leaderboard: Vec<LeaderboardEntry>) -> Markup {
    let top = leaderboard
        .iter()
        .take(100)
        .filter(|entry| entry.name.trim() != "Amanuens");

    html! {
        (DOCTYPE)
        head {
            title { (TITLE) "| Top " (TAKE_NUM) }
            style { (include_str!("style.css")) }
        }
        body {
            h1 { (TITLE) }
            p {
               "In the time-period 2025-08-01 to 2026-07-01."
            }
            table {
                thead {
                    tr {
                        th { "Rank" }
                        th { "Name" }
                        th { "Number of Bookings" }
                    }
                }
                tbody {
                    @for (rank, entry) in top.enumerate() {
                        tr {
                            td { (rank + 1) }
                            td { (entry.name) }
                            td { (entry.num_bookings) }
                        }
                    }
                }
            }
        }
    }
}
