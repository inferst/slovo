use super::Request;
use crate::components::{word::UserPosition, Word};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[component]
pub fn History(history: ReadOnlySignal<Vec<Request>>) -> Element {
    let last = history.iter().max_by_key(|word| word.id);
    let last_id = last.as_ref().map_or(0, |word| word.id);

    let history = use_memo::<Vec<Request>>(move || {
        let mut history = history();
        history.reverse();
        history.iter().take(10).cloned().collect()
    });

    info!("History render");

    rsx! {
        div {
            class: "w-[50%]",
            ul {
                class: "mask-b-from-0% min-h-[400px]",
                for item in history() {
                    li {
                        key: item.id,
                        class: "mr-1",
                        class: if item.id == last_id && !item.animate {"opacity-0"},
                        class: if item.animate {"animate-slide-in"},
                        Word {
                            word: item.score.word.clone(),
                            user: item.user.clone(),
                            color: item.color.clone(),
                            distance: item.score.rank,
                            animate: item.animate,
                            user_position: UserPosition::Left,
                            details: item.score.details.clone(),
                        }
                    }
                }
            }
        }
    }
}
