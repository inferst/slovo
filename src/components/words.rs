use crate::{
    api::Score,
    components::{word::UserPosition, History, Leaderboard, Word},
};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Request {
    pub id: u32,
    pub score: Score,
    pub user: String,
    pub color: String,
    pub animate: bool,
}

#[component]
pub fn Words(
    requests: ReadOnlySignal<Vec<Request>>,
    words: ReadOnlySignal<Vec<Request>>,
) -> Element {
    let last = requests.iter().max_by_key(|word| word.id);
    let last_id = last.as_ref().map_or(0, |word| word.id);
    let is_completed = last.as_ref().is_some_and(|word| word.score.completed);

    info!("Words render");

    rsx! {
        div {
            class: "mx-auto w-[800px] flex flex-col text-xl items-center justify-center",

            if requests.len() == 0 {
                div {
                    class: "w-[400px] my-4 flex font-bold border-2 p-2 rounded-md justify-center items-center text-center h-[100%]",
                    "Start typing"
                }
            } else {
                div {
                    class: "w-[400px]",

                    if is_completed {
                        Leaderboard { }
                    }

                    // if let Some(item) = last {
                    //     div {
                    //         class: "my-4 min-h-[76px]",
                    //         if item.score.rank > 0 {
                    //             Word {
                    //                 word: item.score.word.clone(),
                    //                 user: item.user.clone(),
                    //                 color: item.color.clone(),
                    //                 distance: item.score.rank,
                    //                 animate: item.animate,
                    //                 user_position: UserPosition::Right,
                    //             }
                    //         } else {
                    //
                    //             div {
                    //                 class: "flex text-lg border-2 p-2 rounded-md justify-center items-center text-center h-[100%]",
                    //                 "{item.score.details}"
                    //             }
                    //         }
                    //     }
                    // }
                }

                div {
                    class: "flex w-full mt-4",
                    History { history: requests }
                    ul {
                        class: "w-[50%]",
                        for item in words().iter().filter(|item| item.score.rank > 0) {
                            li {
                                key: item.score.rank,
                                class: "ml-1",
                                class: if item.id == last_id && !item.animate {"opacity-0"},
                                class: if item.animate {"animate-slide-in"},
                                Word {
                                    word: item.score.word.clone(),
                                    user: item.user.clone(),
                                    color: item.color.clone(),
                                    distance: item.score.rank,
                                    animate: item.animate,
                                    user_position: UserPosition::Right,
                                    details: item.score.details.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
