use maud::{DOCTYPE, Markup, html};

use crate::leaderboard::LeaderboardEntry;

const TITLE: &str = "TimeEdit Leaderboard";

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
                        th {}
                        th {}
                        th { "Number of Bookings" }
                    }
                }
                tbody {
                    @for (i, entry) in top.enumerate() {
                        tr {
                            td { (i + 1) }
                            td style="width: 15em" { (entry.name) }
                            td {
                                a   target="_blank"
                                    href={
                                        "https://cloud.timeedit.net/liu/web/schema/ri.html?sid=3&p=20250101,20270101&objects="
                                        (entry.object_id)
                                    }
                                { "[1]" }
                            }
                            td {
                                a   target="_blank"
                                    href={
                                        "https://cloud.timeedit.net/liu/web/schema/ri.html?p=20250101,20270101&objects="
                                        (entry.object_id)
                                    }
                                { "[2]" }
                            }
                            td { (entry.num_bookings) }
                        }
                    }
                }
            }
        }
    }
}
