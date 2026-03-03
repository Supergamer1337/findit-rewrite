use crate::components::ServiceCard;
use crate::models::Category;
use dioxus::prelude::*;

#[component]
pub fn CategoryList(category: Category) -> Element {
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
