use dioxus::prelude::*;

use crate::auth::get_auth_status;

#[component]
pub fn Login(next: Option<String>) -> Element {
    let next = sanitize_next(next.as_deref());
    let auth_status = use_server_future(get_auth_status)?;

    let auth_status = auth_status.value()();
    let redirect_target = format!("/auth/login?next={}", urlencoding::encode(&next));

    if let Some(Ok(status)) = auth_status {
        if status.authenticated {
            return rsx! {
                div { class: "app-container",
                    nav { class: "header-nav",
                        h1 { class: "header-title", "findIT" }
                        a { class: "admin-nav-back", href: "/", "← Back to dashboard" }
                    }

                    main { class: "main-content",
                        section { class: "admin-panel admin-hero",
                            p { class: "admin-eyebrow", "Already signed in" }
                            h2 { class: "admin-section-title", "You can continue to the admin panel" }
                            p { class: "admin-section-subtitle",
                                if let Some(name) = status.display_name {
                                    "Signed in as {name}."
                                } else {
                                    "Your Gamma session is active."
                                }
                            }
                            div { class: "admin-form-actions",
                                a { class: "admin-btn admin-btn-primary", href: next.clone(), "Go to admin" }
                                a { class: "admin-btn admin-btn-secondary", href: "/auth/logout", "Sign out" }
                            }
                        }
                    }
                }
            };
        }
    }

    rsx! {
        div { class: "app-container",
            nav { class: "header-nav",
                h1 { class: "header-title", "findIT" }
                a { class: "admin-nav-back", href: "/", "← Back to dashboard" }
            }

            main { class: "main-content",
                section { class: "admin-panel admin-hero",
                    p { class: "admin-eyebrow", "Admin access" }
                    h2 { class: "admin-section-title", "Sign in with Gamma to continue" }
                    p { class: "admin-section-subtitle",
                        "The admin panel is protected through OpenID Connect. Authenticate with Gamma to manage manual services and icons."
                    }
                    div { class: "admin-form-actions",
                        a { class: "admin-btn admin-btn-primary", href: redirect_target, "Sign in with Gamma" }
                    }
                }
            }
        }
    }
}

fn sanitize_next(next: Option<&str>) -> String {
    let next = next.unwrap_or("/admin");
    if next.starts_with('/') && !next.starts_with("//") {
        next.to_string()
    } else {
        "/admin".to_string()
    }
}
