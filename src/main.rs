use api::{Contextno, Score};
use dioxus::dioxus_core::AttributeValue;
use dioxus::logger;
use dioxus::logger::tracing::Level;
use dioxus::{logger::tracing::debug, prelude::*};
use futures::stream::{AbortHandle, Abortable};
use gloo::storage::{LocalStorage, Storage};
use gloo_timers::future::TimeoutFuture;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use views::{Game, Navbar, Settings};
use websocket::{setup_websocket, ChatMessage, Command, WordMessage};

mod api;
mod components;
mod views;
mod websocket;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Game {},

        #[route("/settings")]
        Settings {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    logger::init(Level::DEBUG).expect("failed to init logger");
    dioxus::launch(App);
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub id: u32,
    pub score: Score,
    pub user: String,
    pub color: String,
    pub animate: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub channel: Signal<String>,
    pub wins: Signal<HashMap<String, String>>,
    pub requests: Signal<Vec<Request>>,
    pub on_add: Callback<String, ()>,
}

#[component]
fn App() -> Element {
    let mut is_completed = use_signal(|| LocalStorage::get("is_completed").unwrap_or(false));
    let mut wins = use_signal(|| LocalStorage::get("wins").unwrap_or(HashMap::new()));
    let mut requests =
        use_signal(|| LocalStorage::get::<Vec<Request>>("requests").unwrap_or(Vec::new()));

    let mut add_request = async move |challenge_id: String, word_message: WordMessage| {
        debug!(?word_message);

        if word_message
            .word
            .chars()
            .all(|c| matches!(c, 'а'..='я' | 'ё'))
        {
            let word = word_message.word.to_lowercase().replace('ё', "е");

            let mut result = None;

            if requests
                .read()
                .iter()
                .any(|request| request.score.word == word)
            {
                result = Some(Score {
                    error: Some(format!("Слово {} уже использовалось", word)),
                    distance: None,
                    word: "".to_string(),
                });
            } else {
                let score = Contextno::get_score(challenge_id.to_string(), word).await;

                if let Ok(score) = score {
                    result = Some(score);
                }
            }

            if let Some(item) = result {
                let distance = item.distance;

                for request in requests.write().iter_mut() {
                    request.animate = false;
                }

                let current_id = requests
                    .read()
                    .iter()
                    .max_by_key(|word| word.id)
                    .map_or(0, |word| word.id);

                let next_id = current_id + 1;

                let word = Request {
                    id: next_id,
                    score: item,
                    user: word_message.user.clone(),
                    color: word_message.color.clone(),
                    animate: false,
                };

                requests.write().push(word);

                if let Some(distance) = distance {
                    if distance == 1 {
                        wins.write().insert(challenge_id, word_message.user);
                        is_completed.set(true);
                        let _ = LocalStorage::set("wins", wins());
                        // let _ = LocalStorage::set("is_completed", is_completed());
                    }
                }

                // let _ = LocalStorage::set("requests", requests());

                spawn(async move {
                    TimeoutFuture::new(10).await;

                    for request in requests.write().iter_mut() {
                        if request.id == next_id {
                            request.animate = true;
                        }
                    }
                });
            }
        }
    };

    let mut challenge = use_resource(async || Contextno::get_random_challenge().await);

    let state = use_context_provider(|| AppState {
        channel: Signal::new(LocalStorage::get("channel").unwrap_or(String::from(""))),
        wins,
        requests,
        on_add: Callback::new(move |word: String| {
            let challenge_clone = {
                match &*challenge.read_unchecked() {
                    Some(Ok(challenge)) => Some(challenge.clone()),
                    _ => None,
                }
            };

            async move {
                if let Some(option) = challenge_clone {
                    let challenge_id = option.id.clone();

                    add_request(
                        challenge_id,
                        WordMessage {
                            word,
                            user: "".to_string(),
                            color: "#ffffff".to_string(),
                        },
                    )
                    .await;
                }
            }
        }),
    });

    let abort_handle = use_hook(|| Rc::new(RefCell::new(None::<AbortHandle>)));

    use_effect(move || {
        let channel = state.channel.read().clone();

        if let Some(handle) = abort_handle.borrow().as_ref() {
            debug!("Aborting websocket");
            handle.abort();
        }

        let (handle, reg) = AbortHandle::new_pair();
        *abort_handle.borrow_mut() = Some(handle);

        spawn({
            let future = setup_websocket(channel, move |message: ChatMessage| async move {
                let challenge_clone = {
                    match &*challenge.read_unchecked() {
                        Some(Ok(challenge)) => Some(challenge.clone()),
                        _ => None,
                    }
                };

                if let Some(option) = challenge_clone {
                    let challenge_id = option.id.clone();

                    match message {
                        ChatMessage::Word(word_message) => {
                            debug!(?word_message);
                            add_request(challenge_id, word_message).await;
                        }
                        ChatMessage::Command(command) => {
                            if command == Command::Next && is_completed() {
                                debug!("Restarting challenge");
                                requests.clear();
                                is_completed.set(false);
                                challenge.restart();
                            }
                        }
                    }
                }
            });

            async move {
                let _ = Abortable::new(future, reg).await;
            }
        });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com"}
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com"}
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap"}
        document::Script { data: AttributeValue::Text("qwerty".to_string()), async: true, src: "https://identity.netlify.com/v1/netlify-identity-widget.js" }

        match &*challenge.read_unchecked() {
            Some(Ok(_)) => rsx! {
                Router::<Route> {}
            },
            Some(Err(e)) => rsx! { p { "Loading failed, {e}" } },
            None =>  rsx! { p { "Loading..." } }
        }
    }
}
