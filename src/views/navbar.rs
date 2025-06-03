use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let path: Route = use_route();

    let link_class = |route: Route| {
        let mut current = "hover:text-gray-300";

        if route == path {
            current = "text-purple-400 hover:text-purple-500 font-bold";
        }

        let class = "mx-2 transition-colors duration-200 ease hover:cursor-pointer";

        format!("{} {}", class, current)
    };

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            class: "flex justify-center",
            Link {
                class: link_class(Route::Game {}),
                to: Route::Game {},
                "Game"
            }

            Link {
                class: link_class(Route::Settings {}),
                to: Route::Settings {},
                "Settings"
            }
        }

        Outlet::<Route> {}
    }
}
