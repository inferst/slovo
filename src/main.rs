use api::Contextno;
use dioxus::logger;

use dioxus::logger::tracing::Level;
use dioxus::prelude::*;

use views::{Game, Navbar, Settings};

mod components;
mod views;

mod api;
mod server;

static API: GlobalSignal<Contextno> = Global::new(|| Contextno::new());

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

#[component]
fn App() -> Element {
    let mut is_init = use_signal(|| false);

    use_future(move || async move {
        API.write().initialize_token().await;
        is_init.set(true);
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com"}
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com"}
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap"}

        if *is_init.read() {
            Router::<Route> {}
        }
    }
}
