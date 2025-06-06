use dioxus::logger::tracing::info;
use dioxus::prelude::*;

use crate::components::Words;
use crate::AppState;

#[component]
pub fn Game() -> Element {
    let requests = use_context::<AppState>().requests;

    info!("Game render");

    rsx! {
        Words {
            requests,
        }
    }
}
