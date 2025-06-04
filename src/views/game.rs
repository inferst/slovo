use crate::{
    api::Score,
    components::{Request, Words},
    views::{
        setup_websocket_listener,
        websocket::{ChatMessage, Command},
    },
    API,
};
use dioxus::logger::tracing::info;
use dioxus::{logger::tracing::debug, prelude::*};
use gloo::storage::{LocalStorage, Storage};
use gloo_timers::future::TimeoutFuture;
use std::collections::HashMap;

fn is_only_russian_letters(s: &str) -> bool {
    s.chars().all(|c| matches!(c, 'а'..='я' | 'ё'))
}

#[component]
pub fn Game() -> Element {
    let mut requests = use_signal::<Vec<Request>>(Vec::new);
    let mut words = use_signal::<Vec<Request>>(Vec::new);

    let mut is_completed = use_signal(|| false);

    let mut challenge = use_resource(async || {
        let api = API.read().clone();
        api.get_random_challenge().await
    });

    let mut add_request =
        async move |challenge_id: String, score: Score, user: String, color: String| {
            let rank = score.rank;

            for request in requests.write().iter_mut() {
                request.animate = false;
            }

            let next_id = requests()
                .iter()
                .max_by_key(|word| word.id)
                .as_ref()
                .map_or(0, |word| word.id)
                + 1;

            let word = Request {
                id: next_id,
                score,
                user: user.clone(),
                color,
                animate: false,
            };

            requests.write().push(word);

            words.set(requests.read().clone());
            words.write().sort_by_key(|request| request.score.rank);

            if rank == 1 {
                let wins = LocalStorage::get::<HashMap<String, String>>("wins");

                if wins.is_err() {
                    let mut wins = HashMap::new();
                    wins.insert(challenge_id.clone(), user.clone());

                    LocalStorage::set("wins", wins).unwrap();
                }

                if let Ok(mut wins) = wins {
                    if !wins.contains_key(&challenge_id) {
                        wins.insert(challenge_id.clone(), user.clone());
                        LocalStorage::set("wins", wins).unwrap();
                    }
                }

                is_completed.set(true);
            }

            spawn(async move {
                TimeoutFuture::new(10).await;

                for request in requests.write().iter_mut() {
                    if request.id == next_id {
                        request.animate = true;
                    }
                }

                for request in words.write().iter_mut() {
                    if request.id == next_id {
                        request.animate = true;
                    }
                }
            });
        };

    let _tx = use_coroutine(setup_websocket_listener(
        move |message: ChatMessage| async move {
            debug!(?message);

            match message {
                ChatMessage::Word(word_message) => {
                    let api = API.read().clone();

                    let challenge_id = (*challenge.read_unchecked())
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .id
                        .clone();

                    if is_only_russian_letters(&word_message.word) {
                        let word = word_message.word.to_lowercase().replace('ё', "е");
                        let result = api.get_score(challenge_id.to_string(), word).await;

                        if let Ok(item) = result {
                            add_request(challenge_id, item, word_message.user, word_message.color)
                                .await;
                        }
                    }
                }
                ChatMessage::Command(command) => {
                    if command == Command::Next && *is_completed.read() {
                        debug!("Restarting challenge");
                        requests.clear();
                        is_completed.set(false);
                        challenge.restart();
                    }
                }
            }
        },
    ));

    use_future(move || async move {
        let challenge_id = "".to_string();

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 50;
        score.word = "слово".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 100;
        score.word = "слово".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 150;
        score.word = "слово".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 200;
        score.word = "слово".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 500;
        score.word = "слово".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 1000;
        score.word = "слово".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 2000;
        score.word = "слово".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 3000;
        score.word = "слово".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = -1;
        score.word = "слово".to_string();
        score.details = "Слово слово не найдено в словаре".to_string();
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#ff0000".to_string(),
        )
        .await;

        TimeoutFuture::new(500).await;
        let mut score = Score::new();
        score.rank = 1;
        score.word = "слово".to_string();
        score.completed = true;
        add_request(
            challenge_id.clone(),
            score,
            "MikeRime".to_string(),
            "#0000ff".to_string(),
        )
        .await;
    });

    info!("Game render");

    rsx! {
        match &*challenge.read_unchecked() {
            Some(Ok(_)) => rsx! {
                Words {
                    requests,
                    words,
                }
            },
            Some(Err(e)) => rsx! { p { "Loading failed, {e}" } },
            None =>  rsx! { p { "Loading..." } }
        }
    }
}
