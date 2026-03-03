use dioxus::prelude::*;

use crate::api::get_services;
use crate::components::{CategoryList, Header};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let categories = use_server_future(get_services)?;

    let categories = match categories.value()() {
        Some(Ok(cats)) => cats,
        Some(Err(e)) => {
            return rsx! {
                document::Link { rel: "icon", href: FAVICON }
                document::Link { rel: "stylesheet", href: MAIN_CSS }
                div { class: "app-container",
                    p { class: "error-message", "Failed to load services: {e}" }
                }
            };
        }
        None => {
            return rsx! {
                document::Link { rel: "icon", href: FAVICON }
                document::Link { rel: "stylesheet", href: MAIN_CSS }
                div { class: "app-container",
                    p { class: "loading-message", "Loading services..." }
                }
            };
        }
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "app-container",
            Header { categories: categories.clone() }

            main { class: "main-content",
                for category in categories {
                    CategoryList { category }
                }
            }
        }
    }
}
