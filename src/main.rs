use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Category {
    category: String,
    services: Vec<Service>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Service {
    title: String,
    url: String,
    description: String,
    #[serde(default)]
    github_url: Option<String>,
    #[serde(default)]
    icon: Option<String>,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HAMBURGER_ICON: Asset = asset!("/assets/images/Hamburger_icon.png");

// Embed the JSON data into the binary
const SERVICE_DATA: &str = include_str!("data/service.json");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let categories: Vec<Category> = serde_json::from_str(SERVICE_DATA).unwrap_or_else(|_| vec![]);

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

#[component]
fn Header(categories: Vec<Category>) -> Element {
    let mut show_nav = use_signal(|| false);

    rsx! {
        nav { class: "header-nav",
            h1 { class: "header-title", "findIT" }

            div { class: "header-links-desktop",
                for cat in categories.clone() {
                    a { href: "#{cat.category}", "{cat.category}" }
                }
            }

            button {
                class: "header-mobile-toggle",
                onclick: move |_| show_nav.toggle(),
                img { src: HAMBURGER_ICON, alt: "Menu" }
            }

            if show_nav() {
                div { class: "header-links-mobile",
                    for cat in categories {
                        a {
                            href: "#{cat.category}",
                            onclick: move |_| show_nav.set(false),
                            "{cat.category}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CategoryList(category: Category) -> Element {
    rsx! {
        div { class: "category-section",
            div { id: "{category.category}", class: "category-anchor" }

            h2 { class: "category-title", "{category.category}" }

            div { class: "category-grid",
                for service in category.services {
                    ServiceCard { service }
                }
            }
        }
    }
}

#[component]
fn ServiceCard(service: Service) -> Element {
    let icon_src = if let Some(icon) = &service.icon {
        format!("/images/{}.svg", icon)
    } else {
        format!("{}/favicon.ico", service.url)
    };

    rsx! {
        div { class: "service-card",
            div { class: "service-card-header",
                img {
                    class: "service-icon",
                    src: "{icon_src}",
                    alt: "{service.title} icon",
                }
                a {
                    class: "service-title",
                    href: "{service.url}",
                    target: "_blank",
                    "{service.title}"
                }
            }

            p { class: "service-description", "{service.description}" }

            if let Some(github) = &service.github_url {
                if !github.is_empty() {
                    a {
                        class: "service-github",
                        href: "{github}",
                        target: "_blank",
                        "GITHUB"
                    }
                }
            }

            a {
                class: "service-open-btn",
                href: "{service.url}",
                target: "_blank",
                "OPEN SERVICE"
            }
        }
    }
}
