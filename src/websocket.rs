use dioxus::logger::tracing::{debug, error};
use futures::{SinkExt, StreamExt};
use gloo::net::websocket::futures::WebSocket;
use gloo::net::websocket::Message;
use std::collections::HashMap;
use std::future::Future;

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

pub async fn setup_websocket<G, F>(channel: String, mut on_request: G)
where
    G: FnMut(ChatMessage) -> F + 'static,
    F: Future<Output = ()> + 'static,
{
    let mut connect = async move || {
        let url = "wss://irc-ws.chat.twitch.tv:443";
        debug!("Connecting to websocket at {url}");

        let mut socket = WebSocket::open(url).unwrap();
        debug!("Connected to websocket");

        let mut send = async |msg: &str| {
            let msg = Message::Text(msg.to_string());
            socket.send(msg).await.unwrap();
        };

        send("NICK justinfan12345").await;
        send("CAP REQ :twitch.tv/tags").await;
        send(&format!("JOIN #{}", channel)).await;

        loop {
            match socket.next().await {
                Some(Ok(Message::Text(msg))) => {
                    debug!("Receiving from socket");

                    if let Some(message) = parse_message(&msg) {
                        on_request(message).await;
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
            }
        }
    };

    for _ in 0..10 {
        connect().await;
        debug!("Disconnected from websocket");
        debug!("Trying to reconnect...");
    }

    debug!("Failed to reconnect to websocket");
}
