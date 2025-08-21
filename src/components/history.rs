use crate::components::{word::UserPosition, Word};
use crate::Request;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[component]
pub fn History(requests: ReadOnlySignal<Vec<Request>>) -> Element {
    let last = requests.iter().max_by_key(|word| word.id);
    let last_id = last.as_ref().map_or(0, |word| word.id);

    let history = use_memo::<Vec<Request>>(move || {
        let mut history = requests();
        history.reverse();
        history.iter().take(10).cloned().collect()
    });

    info!("History render");

    rsx! {
        ul {
            class: "mask-b-from-0% min-h-[400px]",
            for item in history() {
                li {
                    key: item.id,
                    class: if item.id == last_id && !item.animate {"opacity-0"},
                    class: if item.animate {"animate-slide-in"},
                    Word {
                        word: item.score.word.clone(),
                        user: item.user.clone(),
                        color: item.color.clone(),
                        distance: item.score.distance.unwrap_or(-1),
                        animate: item.animate,
                        user_position: UserPosition::Left,
                        details: item.score.error.clone().unwrap_or("".to_string()),
                    }
                }
            }
        }
    }
}
