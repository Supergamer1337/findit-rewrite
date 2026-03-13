use dioxus::prelude::*;

use crate::auth::get_auth_status;
use crate::components::{Admin, Home, Login as LoginPage};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login?:next")]
    Login { next: Option<String> },
    #[route("/admin")]
    AdminRoute {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn AdminRoute() -> Element {
    let auth_status = use_server_future(get_auth_status)?;

    match auth_status.value()() {
        Some(Ok(status)) if status.authenticated => rsx! { Admin {} },
        Some(Ok(_)) => rsx! { LoginPage { next: Some("/admin".to_string()) } },
        Some(Err(err)) => rsx! {
            div { class: "app-container",
                div { class: "error-container",
                    h1 { class: "error-title", "Authentication unavailable" }
                    p { class: "error-message", "Failed to load authentication state: {err}" }
                }
            }
        },
        None => rsx! {
            div { class: "app-container",
                div { class: "loading-container",
                    div { class: "loading-spinner" }
                    p { class: "loading-message", "Checking admin access..." }
                }
            }
        },
    }
}

#[component]
pub fn Login(next: Option<String>) -> Element {
    rsx! { LoginPage { next } }
}
