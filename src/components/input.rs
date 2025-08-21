use crate::AppState;
use dioxus::prelude::*;

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

    let on_click = use_callback(move |event: Event<MouseData>| {
        on_add.call(value().clone());
        value.set(String::new());
    });

    rsx! {
        div {
            class: "w-full flex mt-6",

            input {
                r#type: "text",
                name: "word",
                autocomplete: "off",
                spellcheck: false,
                class: "w-full rounded-md border-1 py-2 px-4 outline-none mr-2",
                placeholder: "Введите слово",
                value,
                oninput: on_update,
                onkeydown: on_keydown
            }

            button {
                class: "bg-blue-700 p-2 rounded-md cursor-pointer hover:bg-blue-600",
                onclick: on_click,
                "Отправить"
            }
        }
    }
}
