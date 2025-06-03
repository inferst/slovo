use std::collections::HashMap;

use dioxus::prelude::*;
use gloo::storage::{LocalStorage, Storage};

fn build_leaderboard(games: HashMap<String, String>) -> Vec<(String, usize)> {
    let mut leaderboard: HashMap<String, usize> = HashMap::new();

    for winner in games.values() {
        *leaderboard.entry(winner.clone()).or_insert(0) += 1;
    }

    let mut sorted: Vec<(String, usize)> = leaderboard.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    sorted
}

#[component]
pub fn Leaderboard() -> Element {
    let wins = LocalStorage::get::<HashMap<String, String>>("wins");

    rsx! {
        div {
            class: "font-bold border-2 border-yellow-400 rounded-md p-4 mt-12",

            div {
                class: "flex justify-center",
                div {
                    class: "bg-blue-700 border-2 rounded-md p-2 -mt-12",
                    "LEADERBOARD"
                }
            }

            if let Ok(wins) = wins {
                ul {
                    class: "mt-4",
                    for item in build_leaderboard(wins).iter().take(5).enumerate() {
                        li {
                            key: item.1.0,
                            class: "flex px-4 py-2 mb-2 rounded-md bg-blue-800",
                            div {
                                class: "min-w-8 mr-2",
                                "{item.0 + 1}"
                            }
                            div {
                                class: "grow break-all",
                                "{item.1.0}"
                            }
                            div {
                                class: "ml-2",
                                "{item.1.1}"
                            }
                        }
                    }
                }

            }

        }
    }
}
