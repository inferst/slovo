use dioxus::prelude::*;
use crate::AppState;

#[component]
pub fn Input() -> Element {
    let on_add = use_context::<AppState>().on_add;
    let mut value = use_signal(String::new);

    let on_update = use_callback(move |event: Event<FormData>| {
        value.set(event.value());
    });

    let on_keydown = use_callback(move |event: Event<KeyboardData>| {
        if event.key() == Key::Enter {
            on_add.call(value().clone());
            value.set(String::new());
        }
    });

    rsx! {
        input {
            class: "w-full rounded-md border-1 py-2 px-4 outline-none mt-6",
            placeholder: "Start typing",
            value,
            oninput: on_update,
            onkeydown: on_keydown
        }
    }
}
