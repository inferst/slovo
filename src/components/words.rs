use crate::{
    components::{word::UserPosition, History, Input, Leaderboard, Word},
    Request,
};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[component]
pub fn Words(requests: ReadOnlySignal<Vec<Request>>) -> Element {
    let last = requests.iter().max_by_key(|word| word.id);
    let last_id = last.as_ref().map_or(0, |word| word.id);
    let is_completed = last.is_some_and(|word| word.score.completed);

    let words = use_memo(move || {
        let mut words = requests.cloned();
        words.sort_by_key(|request| request.score.rank);
        words
    });

    info!("Words render");

    rsx! {
        div {
            class: "mx-auto w-[800px] flex flex-col text-xl items-center justify-center",

            if is_completed {
                button {
                    class: "w-[200px] mt-4 font-bold p-2 rounded-md text-center bg-blue-600 hover:bg-blue-700 cursor-pointer",
                    "New Game"
                }
            }

            div {
                class: "w-[400px]",

                if is_completed {
                    Leaderboard { }
                }
            }

            Input { }

            div {
                class: "flex w-full mt-4",

                div {
                    class: "w-[50%] mr-1",

                    History { requests }
                }
                ul {
                    class: "w-[50%] ml-1",
                    for item in words().iter().filter(|item| item.score.rank > 0) {
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
