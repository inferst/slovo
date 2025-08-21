use dioxus::prelude::*;
use gloo::storage::{LocalStorage, Storage};

use crate::{AppState, Route};

#[component]
pub fn Settings() -> Element {
    let navigator = use_navigator();

    let mut channel = use_context::<AppState>().channel;
    let mut value = use_signal(String::new);

    use_effect(move || {
        value.set(channel());
    });

    let on_update = use_callback(move |event: Event<FormData>| {
        value.set(event.value());
    });

    let on_save = use_callback(move |_| {
        channel.set(value());
        LocalStorage::set("channel", channel()).unwrap();
        navigator.replace(Route::Game {});
    });

    rsx! {
        div {
            class: "mx-auto w-[400px] mt-8",

            input {
                class: "w-full rounded-md border-1 py-2 px-4 outline-none",
                placeholder: "Twitch канал",
                value: channel,
                oninput: on_update
            }

            button {
                class: "bg-blue-700 rounded-md py-2 px-4 mt-4 w-full cursor-pointer",
                onclick: on_save,
                "Сохранить"
            }
        }
    }
}
