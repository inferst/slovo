use dioxus::prelude::*;

use crate::{
    api::Score,
    components::{Leaderboard, Word},
};

pub struct Request {
    pub id: u32,
    pub score: Score,
    pub user: String,
    pub color: String,
    pub animate: bool,
}

#[component]
pub fn Words(requests: ReadOnlySignal<Vec<Request>>) -> Element {
    let last = requests.iter().max_by_key(|word| word.id);
    let last_id = last.as_ref().map_or(0, |word| word.id);
    let is_completed = last.as_ref().is_some_and(|word| word.score.completed);

    rsx! {
        div {
            class: "mx-auto w-[400px] text-xl",

            if requests.len() == 0 {
                div {
                    class: "my-4 flex font-bold border-2 p-2 rounded-md justify-center items-center text-center h-[100%]",
                    "Start typing"
                }
            } else {
                if is_completed {
                    Leaderboard { }
                }

                if let Some(item) = last {
                    div {
                        class: "my-4 min-h-[76px]",
                        if item.score.rank > 0 {
                            Word {
                                word: item.score.word.clone(),
                                user: item.user.clone(),
                                color: item.color.clone(),
                                distance: item.score.rank,
                                animate: item.animate,
                            }
                        } else {

                            div {
                                class: "flex text-lg border-2 p-2 rounded-md justify-center items-center text-center h-[100%]",
                                "{item.score.details}"
                            }
                        }
                    }
                }

                ul {
                    for item in requests.iter().filter(|item| item.score.rank > 0) {
                        li {
                            key: item.score.rank,
                            class: if item.id == last_id && !item.animate {"opacity-0"},
                            class: if item.animate {"animate-slide-in"},
                            Word {
                                word: item.score.word.clone(),
                                user: item.user.clone(),
                                color: item.color.clone(),
                                distance: item.score.rank,
                                animate: item.animate,
                            }
                        }
                    }
                }
            }
        }
    }
}
