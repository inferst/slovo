use dioxus::prelude::*;

use crate::{api::Score, components::Word};

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

    rsx! {
        div {
            class: "mx-auto w-[400px]",

            if let Some(item) = last {
                div {
                    class: "my-4 h-[48px]",
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
                            class: "text-center",
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
