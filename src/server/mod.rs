use dioxus::prelude::*;
use server_fn::codec::{StreamingText, TextStream};

#[cfg(feature = "server")]
use twitch_irc::{
    login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

#[cfg(feature = "server")]
async fn subscribe(tx: futures::channel::mpsc::UnboundedSender<Result<String, ServerFnError>>) {
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);

            if tx.unbounded_send(Ok(format!("{:?}", message))).is_err() {
                // If the channel is closed, stop sending chunks
                println!("error");
            }
        }
    });

    client.join("mikerimebot".to_owned()).unwrap();

    println!("joined");

    join_handle.await.unwrap();
}

#[server(output = StreamingText)]
pub async fn start_game() -> Result<TextStream, ServerFnError> {
    let (tx, rx) = futures::channel::mpsc::unbounded();

    tokio::spawn(async move {
        subscribe(tx).await;
    });

    Ok(TextStream::new(rx))
}
