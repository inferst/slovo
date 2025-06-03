use dioxus::hooks::UnboundedReceiver;
use dioxus::logger::tracing::{debug, error};
use futures::{SinkExt, StreamExt};
use gloo::net::websocket::futures::WebSocket;
use gloo::net::websocket::Message;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{future::Future, pin::Pin};

#[derive(Clone, Debug)]
pub struct WordMessage {
    pub word: String,
    pub user: String,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Next,
}

#[derive(Clone, Debug)]
pub enum ChatMessage {
    Word(WordMessage),
    Command(Command),
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

        let user = map.get("display-name").unwrap_or(&"anonymous").to_string();
        let color = map.get("color").unwrap_or(&"#ffffff").to_string();

        let word = message[1..].to_string().trim().to_lowercase();

        if command == "PRIVMSG" && parts.len() == 1 {
            Some(match word.as_str() {
                "!next" => ChatMessage::Command(Command::Next),
                _ => ChatMessage::Word(WordMessage { word, user, color }),
            })
        } else {
            None
        }
    } else {
        None
    }
}

pub fn setup_websocket_listener<G, F>(
    on_request: G,
) -> impl FnMut(UnboundedReceiver<String>) -> Pin<Box<dyn Future<Output = ()>>>
where
    G: FnMut(ChatMessage) -> F + 'static,
    F: Future<Output = ()> + 'static,
{
    let on_request = Rc::new(RefCell::new(on_request));

    move |mut rx: UnboundedReceiver<String>| {
        let on_request = on_request.clone();

        Box::pin(async move {
            let mut connect = async move || {
                let url = "wss://irc-ws.chat.twitch.tv:443";
                debug!("Connecting to websocket at {url}");

                let mut socket = WebSocket::open(url).unwrap();
                debug!("Connected to websocket");

                socket
                    .send(Message::Text("NICK justinfan12345".to_string()))
                    .await
                    .unwrap();
                socket
                    .send(Message::Text("CAP REQ :twitch.tv/tags".to_string()))
                    .await
                    .unwrap();
                socket
                    .send(Message::Text("JOIN #mikerime".to_string()))
                    .await
                    .unwrap();

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
                                    debug!(?message);
                                    let callback = (on_request.borrow_mut())(message);
                                    callback.await;
                                } else if msg.trim() == "PING :tmi.twitch.tv" {
                                    let msg = Message::Text("PONG :tmi.twitch.tv".to_string());
                                    socket.send(msg).await.unwrap();
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

            for _ in 0..10 {
                connect().await;
                debug!("Disconnected from websocket");
                debug!("Trying to reconnect...");
            }

            debug!("Failed to reconnect to websocket");
        })
    }
}
