use maud::{DOCTYPE, Markup, html};

use crate::leaderboard::LeaderboardEntry;

const TITLE: &str = "TimeEdit Leaderboard";
const BOOKINGS_HTML_BASE_URL: &str =
    "https://cloud.timeedit.net/liu/web/schema/ri.html?sid=3&p=20250101,20270101&objects=";

pub fn generate_html(leaderboard: Vec<LeaderboardEntry>) -> Markup {
    let top = leaderboard
        .iter()
        // .take(TAKE_NUM)
        .filter(|entry| entry.name.trim() != "Amanuens" && entry.num_bookings > 0);

    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { (TITLE) }
            style { (include_str!("style.css")) }
        }
        body {
            h1 { (TITLE) " | Linköping university" }
            p { "In the time-period 2025-08-01 to 2026-07-01." }
            p { "People with zero bookings are omitted from the list." }
            p {
                i {
                    "Source code at "
                    a target="_blank" href=(env!("CARGO_PKG_REPOSITORY")) {
                        (env!("CARGO_PKG_REPOSITORY"))
                    }
                }
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
                            td {
                                a target="_blank" href={ (BOOKINGS_HTML_BASE_URL) (entry.object_id) } {
                                    (entry.name)
                                }
                            }
                            td { (entry.num_bookings) }
                        }
                    }
                }
            }
        }
    }
}
