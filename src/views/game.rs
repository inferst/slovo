use std::collections::HashMap;

use crate::{
    api::{Contextno, Score},
    components::{Request, Words},
};
use dioxus::{
    logger::tracing::{debug, error, info},
    prelude::*,
};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_timers::future::TimeoutFuture;

struct ChatMessage {
    word: String,
    user: String,
    color: String,
}

fn parse_message(msg: &str) -> Option<ChatMessage> {
    let parts: Vec<&str> = msg.splitn(5, ' ').collect();

    if let [tags, _, command, _, message] = parts[..] {
        let tags = tags.split(";");
        let mut map = HashMap::new();

        for tag in tags {
            let parts: Vec<&str> = tag.splitn(2, '=').collect();
            if let [key, value] = parts[..] {
                map.insert(key, value);
            }
        }

        let parts: Vec<_> = message.split(" ").collect();

        let user_name = map.get("display-name").unwrap_or(&"anonymous").to_string();
        let color = map.get("color").unwrap_or(&"#ffffff").to_string();

        if command == "PRIVMSG" && parts.len() == 1 {
            Some(ChatMessage {
                word: message[1..].to_string(),
                user: user_name,
                color,
            })
        } else {
            None
        }
    } else {
        None
    }
}

#[component]
pub fn Game() -> Element {
    let mut requests = use_signal::<Vec<Request>>(Vec::new);

    let mut add_request = async move |score: Score, user: String, color: String| {
        let rank = score.rank;

        for request in requests.write().iter_mut() {
            request.animate = false;
        }

        let last = requests
            .iter()
            .reduce(|acc, e| if e.id > acc.id { e } else { acc })
            .map_or(0, |word| word.id);

        let word = Request {
            id: last + 1,
            score,
            user,
            color,
            animate: false,
        };

        requests.write().push(word);

        requests.write().sort_by_key(|request| request.score.rank);

        use_future(move || async move {
            TimeoutFuture::new(0).await;

            for request in requests.write().iter_mut() {
                if request.score.rank == rank {
                    request.animate = true;
                }
            }
        });
    };

    let challenge = use_resource(Contextno::get_random_challenge);

    let tx = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
        let mut connect = async move || {
            let url = "wss://irc-ws.chat.twitch.tv:443";
            debug!("Connecting to websocket at {url}");
            let mut socket = WebSocket::open(url).unwrap();
            debug!("Connected to websocket.");

            loop {
                match futures::future::select(rx.next(), socket.next()).await {
                    futures::future::Either::Left((msg, _)) => {
                        if let Some(msg) = msg {
                            debug!("Sending to socket");
                            socket.send(Message::Text(msg)).await.unwrap();
                        } else {
                            break;
                        }
                    }
                    futures::future::Either::Right((msg, _)) => match msg {
                        Some(Ok(Message::Text(msg))) => {
                            debug!("Receiving from socket");

                            if let Some(message) = parse_message(&msg) {
                                let challenge_id = (*challenge.read_unchecked())
                                    .as_ref()
                                    .unwrap()
                                    .as_ref()
                                    .unwrap()
                                    .id
                                    .clone();

                                let result = Contextno::get_score(challenge_id, message.word).await;

                                if let Ok(item) = result {
                                    add_request(item, message.user, message.color).await;
                                }
                            }
                        }
                        Some(Ok(Message::Bytes(msg))) => {
                            error!("Received binary message: {:?}", msg);
                        }
                        Some(Err(err)) => {
                            error!("Error: {:?}", err);
                            break;
                        }
                        None => {
                            break;
                        }
                    },
                }
            }
        };

        loop {
            connect().await;
            debug!("Disconnected from websocket");

            TimeoutFuture::new(1_000).await;
        }
    });

    use_effect(move || {
        tx.send("NICK justinfan12345".to_string());
        tx.send("CAP REQ :twitch.tv/tags".to_string());
        tx.send("JOIN #mikerime".to_string());
    });

    info!("render");

    rsx! {
        match &*challenge.read_unchecked() {
            Some(Ok(_)) => rsx! {
                Words {
                    requests,
                }
            },
            Some(Err(e)) => rsx! { p { "Loading failed, {e}" } },
            None =>  rsx! { p { "Loading..." } }
        }
    }
}
