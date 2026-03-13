use crate::admin_api::{
    add_icon, add_manual_service, delete_icon, delete_manual_service, list_icons,
    list_manual_services, update_icon, update_manual_service,
};
use crate::models::{IconRecord, ManualServiceRecord};
use dioxus::prelude::*;

#[component]
pub fn Admin() -> Element {
    let mut icons = use_resource(list_icons);
    let mut manual_services = use_resource(list_manual_services);

    let mut new_name = use_signal(String::new);
    let mut new_file_b64 = use_signal(|| Option::<String>::None);
    let mut new_file_ext = use_signal(|| Option::<String>::None);
    let mut new_file_label = use_signal(|| "No file chosen".to_string());
    let mut add_error = use_signal(|| Option::<String>::None);
    let mut add_loading = use_signal(|| false);

    let mut edit_id = use_signal(|| Option::<i64>::None);
    let mut edit_name = use_signal(String::new);
    let mut edit_file_b64 = use_signal(|| Option::<String>::None);
    let mut edit_file_ext = use_signal(|| Option::<String>::None);
    let mut edit_file_label = use_signal(|| "Keep existing image".to_string());
    let mut edit_error = use_signal(|| Option::<String>::None);
    let mut edit_loading = use_signal(|| false);

    let mut new_service_title = use_signal(String::new);
    let mut new_service_url = use_signal(String::new);
    let mut new_service_description = use_signal(String::new);
    let mut new_service_category = use_signal(String::new);
    let mut new_service_github_url = use_signal(String::new);
    let mut new_service_icon_id = use_signal(|| Option::<i64>::None);
    let mut add_service_error = use_signal(|| Option::<String>::None);
    let mut add_service_loading = use_signal(|| false);

    let mut edit_service_id = use_signal(|| Option::<i64>::None);
    let mut edit_service_title = use_signal(String::new);
    let mut edit_service_url = use_signal(String::new);
    let mut edit_service_description = use_signal(String::new);
    let mut edit_service_category = use_signal(String::new);
    let mut edit_service_github_url = use_signal(String::new);
    let mut edit_service_icon_id = use_signal(|| Option::<i64>::None);
    let mut edit_service_error = use_signal(|| Option::<String>::None);
    let mut edit_service_loading = use_signal(|| false);
    let mut confirm_delete_service_id = use_signal(|| Option::<i64>::None);

    let mut modal_open = use_signal(|| false);
    let mut modal_type = use_signal(|| Option::<String>::None);

    let icon_count = match icons() {
        Some(Ok(icon_list)) => icon_list.len().to_string(),
        Some(Err(_)) => "!".to_string(),
        None => "...".to_string(),
    };

    let manual_service_count = match manual_services() {
        Some(Ok(service_list)) => service_list.len().to_string(),
        Some(Err(_)) => "!".to_string(),
        None => "...".to_string(),
    };

    let icon_options = match icons() {
        Some(Ok(icon_list)) => icon_list,
        _ => Vec::new(),
    };

    rsx! {
        div { class: "admin-page",
            nav { class: "header-nav",
                h1 { class: "header-title", "findIT" }
                span { class: "admin-nav-badge", "Admin" }
                a { class: "admin-nav-back", href: "/", "← Back to dashboard" }
            }

            div { class: "admin-content",
                div { class: "admin-panel admin-hero",
                    p { class: "admin-eyebrow", "Control center" }
                    h2 { class: "admin-section-title", "Manage services and icons in one place" }
                    p { class: "admin-section-subtitle",
                        "Add manual services for anything Docker cannot discover, then assign shared icons from the same library used by label-based services. Everything on this page updates the dashboard directly."
                    }

                    div { class: "admin-overview-grid",
                        div { class: "admin-overview-card",
                            span { class: "admin-overview-label", "Manual services" }
                            strong { class: "admin-overview-value", "{manual_service_count}" }
                            span { class: "admin-overview-meta", "Shown on the dashboard" }
                        }
                        div { class: "admin-overview-card",
                            span { class: "admin-overview-label", "Icon library" }
                            strong { class: "admin-overview-value", "{icon_count}" }
                            span { class: "admin-overview-meta", "Available in Docker labels and admin forms" }
                        }
                    }
                }

                section { id: "manual-services", class: "admin-section",
                    div { class: "admin-section-header",
                        p { class: "admin-eyebrow", "Manual services" }
                        h2 { class: "admin-section-title", "Create services outside Docker discovery" }
                        p { class: "admin-section-subtitle",
                            "These entries support the same fields as the label-based flow: title, URL, description, category, optional GitHub link, and an icon from the shared library."
                        }
                    }

                    div { class: "admin-panel",
                        h3 { class: "admin-panel-title", "Add manual service" }
                        p { class: "admin-panel-subtitle",
                            "Create a dashboard entry for anything that is not discoverable through Docker labels. You can come back to edit or remove it later."
                        }

                        div { class: "admin-form",
                            ManualServiceFields {
                                title: new_service_title,
                                url: new_service_url,
                                description: new_service_description,
                                category: new_service_category,
                                github_url: new_service_github_url,
                                icon_id: new_service_icon_id,
                                icon_options: icon_options.clone(),
                            }

                            if let Some(err) = add_service_error() {
                                p { class: "admin-error", "{err}" }
                            }

                            div { class: "admin-form-actions",
                                button {
                                    class: "admin-btn admin-btn-primary",
                                    disabled: add_service_loading(),
                                    onclick: move |_| {
                                        let title = new_service_title().trim().to_string();
                                        let url = new_service_url().trim().to_string();
                                        let description = new_service_description().trim().to_string();
                                        let category = new_service_category().trim().to_string();

                                        if let Some(err) = validate_service_form(&title, &url, &description, &category) {
                                            add_service_error.set(Some(err));
                                            return;
                                        }

                                        let github_url = optional_string(new_service_github_url());
                                        let icon_id = new_service_icon_id();

                                        add_service_error.set(None);
                                        add_service_loading.set(true);

                                        spawn(async move {
                                            match add_manual_service(
                                                title,
                                                url,
                                                description,
                                                category,
                                                github_url,
                                                icon_id,
                                            )
                                            .await
                                            {
                                                Ok(_) => {
                                                    new_service_title.set(String::new());
                                                    new_service_url.set(String::new());
                                                    new_service_description.set(String::new());
                                                    new_service_category.set(String::new());
                                                    new_service_github_url.set(String::new());
                                                    new_service_icon_id.set(None);
                                                    manual_services.restart();
                                                }
                                                Err(e) => add_service_error.set(Some(e.to_string())),
                                            }

                                            add_service_loading.set(false);
                                        });
                                    },
                                    if add_service_loading() { "Saving service..." } else { "Add service" }
                                }
                            }
                        }
                    }

                    match manual_services() {
                        None => rsx! {
                            div { class: "admin-loading",
                                div { class: "loading-spinner" }
                                p { class: "loading-message", "Loading services..." }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            p { class: "admin-error", "Failed to load services: {e}" }
                        },
                        Some(Ok(service_list)) => {
                            if service_list.is_empty() {
                                rsx! {
                                    p { class: "admin-empty", "No manual services yet. Add one above to publish it on the dashboard." }
                                }
                            } else {
                                rsx! {
                                {
                                    // Edit Modal for Services - render based on state, not list iteration
                                    rsx! {
                                        if let Some(true) = modal_type().map(|t| t == "service") {
                                            if let Some(service_id) = edit_service_id() {
                                                Modal {
                                                    title: "Edit service".to_string(),
                                                    is_open: modal_open(),
                                                    on_close: move |_| modal_open.set(false),
                                                    div { class: "admin-modal-content",
                                                        ManualServiceFields {
                                                            title: edit_service_title,
                                                            url: edit_service_url,
                                                            description: edit_service_description,
                                                            category: edit_service_category,
                                                            github_url: edit_service_github_url,
                                                            icon_id: edit_service_icon_id,
                                                            icon_options: icon_options.clone(),
                                                        }

                                                        if let Some(err) = edit_service_error() {
                                                            p { class: "admin-error", "{err}" }
                                                        }

                                                        div { class: "admin-modal-actions",
                                                            button {
                                                                class: "admin-btn admin-btn-primary",
                                                                disabled: edit_service_loading(),
                                                                onclick: move |_| {
                                                                    let title = edit_service_title().trim().to_string();
                                                                    let url = edit_service_url().trim().to_string();
                                                                    let description = edit_service_description().trim().to_string();
                                                                    let category = edit_service_category().trim().to_string();

                                                                    if let Some(err) = validate_service_form(
                                                                        &title,
                                                                        &url,
                                                                        &description,
                                                                        &category,
                                                                    ) {
                                                                        edit_service_error.set(Some(err));
                                                                        return;
                                                                    }

                                                                    let github_url = optional_string(edit_service_github_url());
                                                                    let icon_id = edit_service_icon_id();
                                                                    edit_service_error.set(None);
                                                                    edit_service_loading.set(true);

                                                                    spawn(async move {
                                                                        match update_manual_service(
                                                                            service_id,
                                                                            title,
                                                                            url,
                                                                            description,
                                                                            category,
                                                                            github_url,
                                                                            icon_id,
                                                                        )
                                                                        .await
                                                                        {
                                                                            Ok(_) => {
                                                                                modal_open.set(false);
                                                                                modal_type.set(None);
                                                                                edit_service_id.set(None);
                                                                                manual_services.restart();
                                                                            }
                                                                            Err(e) => {
                                                                                edit_service_error.set(Some(e.to_string()));
                                                                            }
                                                                        }

                                                                        edit_service_loading.set(false);
                                                                    });
                                                                },
                                                                if edit_service_loading() { "Saving..." } else { "Save" }
                                                            }
                                                            button {
                                                                class: "admin-btn admin-btn-secondary",
                                                                onclick: move |_| {
                                                                    modal_open.set(false);
                                                                    modal_type.set(None);
                                                                    edit_service_id.set(None);
                                                                    edit_service_error.set(None);
                                                                },
                                                                "Cancel"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                    div { class: "admin-service-grid",
                                        for service in service_list {
                                            ManualServiceCard {
                                                key: "{service.id}",
                                                service: service.clone(),
                                                icon_options: icon_options.clone(),
                                                on_start_edit: {
                                                    let service = service.clone();
                                                    move |_| {
                                                        edit_service_id.set(Some(service.id));
                                                        edit_service_title.set(service.title.clone());
                                                        edit_service_url.set(service.url.clone());
                                                        edit_service_description.set(service.description.clone());
                                                        edit_service_category.set(service.category.clone());
                                                        edit_service_github_url
                                                            .set(service.github_url.clone().unwrap_or_default());
                                                        edit_service_icon_id.set(service.icon_id);
                                                        edit_service_error.set(None);
                                                        modal_type.set(Some("service".to_string()));
                                                        modal_open.set(true);
                                                    }
                                                },
                                                on_request_delete: {
                                                    let service_id = service.id;
                                                    move |_| {
                                                        confirm_delete_service_id.set(Some(service_id));
                                                        // Show confirmation dialog
                                                        if web_sys::window()
                                                            .and_then(|w| w.confirm_with_message("Delete this manual service?").ok())
                                                            .unwrap_or(false)
                                                        {
                                                            spawn(async move {
                                                                let _ = delete_manual_service(service_id).await;
                                                                manual_services.restart();
                                                            });
                                                        }
                                                    }
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                section { id: "icon-library", class: "admin-section",
                    div { class: "admin-section-header",
                        p { class: "admin-eyebrow", "Icon library" }
                        h2 { class: "admin-section-title", "Manage the shared icon collection" }
                        p { class: "admin-section-subtitle",
                            "Icons can be referenced in Docker with "
                            code { "findit.icon" }
                            " and selected directly from manual services in this admin panel."
                        }
                    }

                    div { class: "admin-panel admin-add-panel",
                        h3 { class: "admin-panel-title", "Add new icon" }

                        div { class: "admin-form",
                            div { class: "admin-form-group",
                                label { class: "admin-label", r#for: "new-icon-name", "Icon name" }
                                input {
                                    id: "new-icon-name",
                                    class: "admin-input",
                                    r#type: "text",
                                    placeholder: "e.g. my-service",
                                    value: "{new_name}",
                                    oninput: move |e| new_name.set(e.value()),
                                }
                                p { class: "admin-hint",
                                    "Lowercase name used in the "
                                    code { "findit.icon" }
                                    " label. Must be unique."
                                }
                            }

                            div { class: "admin-form-group",
                                label { class: "admin-label", "Icon file" }
                                label { class: "admin-file-label",
                                    input {
                                        class: "admin-file-input",
                                        r#type: "file",
                                        accept: ".svg,.png,.jpg,.jpeg,.webp,.gif,.ico",
                                        onchange: move |e| {
                                            read_file_to_signal(e, new_file_b64, new_file_ext, new_file_label);
                                        },
                                    }
                                    span { class: "admin-file-btn", "Choose file" }
                                    span { class: "admin-file-name", "{new_file_label}" }
                                }
                                p { class: "admin-hint", "Supported: SVG, PNG, JPG, WEBP, GIF, ICO" }
                            }

                            if let Some(err) = add_error() {
                                p { class: "admin-error", "{err}" }
                            }

                            button {
                                class: "admin-btn admin-btn-primary",
                                disabled: add_loading(),
                                onclick: move |_| {
                                    let name = new_name().trim().to_lowercase();
                                    let b64 = new_file_b64();
                                    let ext = new_file_ext();

                                    if name.is_empty() {
                                        add_error.set(Some("Please enter a name.".into()));
                                        return;
                                    }

                                    let Some(b64) = b64 else {
                                        add_error.set(Some("Please choose a file.".into()));
                                        return;
                                    };

                                    let Some(ext) = ext else {
                                        add_error.set(Some("Could not determine file type.".into()));
                                        return;
                                    };

                                    add_error.set(None);
                                    add_loading.set(true);

                                    spawn(async move {
                                        match add_icon(name, b64, ext).await {
                                            Ok(_) => {
                                                new_name.set(String::new());
                                                new_file_b64.set(None);
                                                new_file_ext.set(None);
                                                new_file_label.set("No file chosen".into());
                                                icons.restart();
                                                manual_services.restart();
                                            }
                                            Err(e) => add_error.set(Some(e.to_string())),
                                        }

                                        add_loading.set(false);
                                    });
                                },
                                if add_loading() { "Uploading..." } else { "Add icon" }
                            }
                        }
                    }

                    match icons() {
                        None => rsx! {
                            div { class: "admin-loading",
                                div { class: "loading-spinner" }
                                p { class: "loading-message", "Loading icons..." }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            p { class: "admin-error", "Failed to load icons: {e}" }
                        },
                        Some(Ok(icon_list)) => {
                            if icon_list.is_empty() {
                                rsx! {
                                    p { class: "admin-empty", "No icons yet. Add one above to use them in Docker labels and manual services." }
                                }
                            } else {
                                rsx! {
                                {
                                    // Edit Modal for Icons
                                    rsx! {
                                        if let Some(true) = modal_type().map(|t| t == "icon") {
                                            if let Some(icon_id) = edit_id() {
                                                Modal {
                                                    title: "Edit icon".to_string(),
                                                    is_open: modal_open(),
                                                    on_close: move |_| modal_open.set(false),
                                                    div { class: "admin-modal-content",
                                                        div { class: "admin-form-group",
                                                            label { class: "admin-label", "Name" }
                                                            input {
                                                                class: "admin-input",
                                                                r#type: "text",
                                                                value: "{edit_name}",
                                                                oninput: move |e| edit_name.set(e.value()),
                                                            }
                                                        }

                                                        div { class: "admin-form-group",
                                                            label { class: "admin-label", "Replace image" }
                                                            label { class: "admin-file-label",
                                                                input {
                                                                    class: "admin-file-input",
                                                                    r#type: "file",
                                                                    accept: ".svg,.png,.jpg,.jpeg,.webp,.gif,.ico",
                                                                    onchange: move |e| {
                                                                        read_file_to_signal(
                                                                            e,
                                                                            edit_file_b64,
                                                                            edit_file_ext,
                                                                            edit_file_label,
                                                                        );
                                                                    },
                                                                }
                                                                span { class: "admin-file-btn", "Choose file" }
                                                                span { class: "admin-file-name", "{edit_file_label}" }
                                                            }
                                                        }

                                                        if let Some(err) = edit_error() {
                                                            p { class: "admin-error", "{err}" }
                                                        }

                                                        div { class: "admin-modal-actions",
                                                            button {
                                                                class: "admin-btn admin-btn-primary",
                                                                disabled: edit_loading(),
                                                                onclick: move |_| {
                                                                    let name = edit_name().trim().to_lowercase();
                                                                    if name.is_empty() {
                                                                        edit_error.set(Some("Name must not be empty.".into()));
                                                                        return;
                                                                    }

                                                                    let b64 = edit_file_b64();
                                                                    let ext = edit_file_ext();
                                                                    edit_error.set(None);
                                                                    edit_loading.set(true);

                                                                    spawn(async move {
                                                                        match update_icon(icon_id, Some(name), b64, ext).await {
                                                                            Ok(_) => {
                                                                                modal_open.set(false);
                                                                                modal_type.set(None);
                                                                                edit_id.set(None);
                                                                                icons.restart();
                                                                                manual_services.restart();
                                                                            }
                                                                            Err(e) => {
                                                                                edit_error.set(Some(e.to_string()));
                                                                            }
                                                                        }

                                                                        edit_loading.set(false);
                                                                    });
                                                                },
                                                                if edit_loading() { "Saving..." } else { "Save" }
                                                            }
                                                            button {
                                                                class: "admin-btn admin-btn-secondary",
                                                                onclick: move |_| {
                                                                    modal_open.set(false);
                                                                    modal_type.set(None);
                                                                    edit_id.set(None);
                                                                    edit_error.set(None);
                                                                },
                                                                "Cancel"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                    div { class: "admin-icon-grid",
                                        for icon in icon_list {
                                            IconCard {
                                                key: "{icon.id}",
                                                icon: icon.clone(),
                                                on_start_edit: {
                                                    let name = icon.name.clone();
                                                    move |_| {
                                                        edit_id.set(Some(icon.id));
                                                        edit_name.set(name.clone());
                                                        edit_file_b64.set(None);
                                                        edit_file_ext.set(None);
                                                        edit_file_label.set("Keep existing image".into());
                                                        edit_error.set(None);
                                                        modal_type.set(Some("icon".to_string()));
                                                        modal_open.set(true);
                                                    }
                                                },
                                                on_request_delete: {
                                                    let icon_id = icon.id;
                                                    move |_| {
                                                        if web_sys::window()
                                                            .and_then(|w| w.confirm_with_message("Delete this icon?").ok())
                                                            .unwrap_or(false)
                                                        {
                                                            spawn(async move {
                                                                let _ = delete_icon(icon_id).await;
                                                                icons.restart();
                                                                manual_services.restart();
                                                            });
                                                        }
                                                    }
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ManualServiceFields(
    title: Signal<String>,
    url: Signal<String>,
    description: Signal<String>,
    category: Signal<String>,
    github_url: Signal<String>,
    icon_id: Signal<Option<i64>>,
    icon_options: Vec<IconRecord>,
) -> Element {
    let selected_icon = icon_options
        .iter()
        .find(|icon| Some(icon.id) == icon_id())
        .cloned();

    rsx! {
        div { class: "admin-service-form-grid",
            div { class: "admin-form-group",
                label { class: "admin-label", r#for: "manual-service-title", "Title" }
                input {
                    id: "manual-service-title",
                    class: "admin-input",
                    r#type: "text",
                    placeholder: "Service name",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                }
            }

            div { class: "admin-form-group",
                label { class: "admin-label", r#for: "manual-service-category", "Category" }
                input {
                    id: "manual-service-category",
                    class: "admin-input",
                    r#type: "text",
                    placeholder: "e.g. Utilities",
                    value: "{category}",
                    oninput: move |e| category.set(e.value()),
                }
            }

            div { class: "admin-form-group admin-form-group--wide",
                label { class: "admin-label", r#for: "manual-service-url", "Service URL" }
                input {
                    id: "manual-service-url",
                    class: "admin-input",
                    r#type: "url",
                    placeholder: "https://service.example.com",
                    value: "{url}",
                    oninput: move |e| url.set(e.value()),
                }
            }

            div { class: "admin-form-group admin-form-group--wide",
                label { class: "admin-label", r#for: "manual-service-description", "Description" }
                textarea {
                    id: "manual-service-description",
                    class: "admin-input admin-textarea",
                    rows: "4",
                    placeholder: "What should people know before opening it?",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                }
            }

            div { class: "admin-form-group",
                label { class: "admin-label", r#for: "manual-service-github", "GitHub URL" }
                input {
                    id: "manual-service-github",
                    class: "admin-input",
                    r#type: "url",
                    placeholder: "Optional source link",
                    value: "{github_url}",
                    oninput: move |e| github_url.set(e.value()),
                }
            }

            div { class: "admin-form-group",
                label { class: "admin-label", r#for: "manual-service-icon", "Icon" }
                select {
                    id: "manual-service-icon",
                    class: "admin-input admin-select",
                    value: "{selected_icon_value(icon_id())}",
                    onchange: move |e| icon_id.set(parse_optional_i64(&e.value())),
                    option { value: "", "No icon" }
                    for icon in icon_options.clone() {
                        option { value: "{icon.id}", "{icon.name}" }
                    }
                }

                if let Some(icon) = selected_icon {
                    div { class: "admin-inline-icon",
                        img { class: "admin-inline-icon-img", src: "{icon.path}", alt: "{icon.name}" }
                        span { class: "admin-inline-icon-name", "{icon.name}" }
                    }
                } else if icon_options.is_empty() {
                    p { class: "admin-hint", "No icons yet. Add one in the icon library if you want a custom image." }
                } else {
                    p { class: "admin-hint", "Select an icon from the shared library or leave it empty." }
                }
            }
        }
    }
}

#[component]
fn ManualServiceCard(
    service: ManualServiceRecord,
    icon_options: Vec<IconRecord>,
    on_start_edit: EventHandler<MouseEvent>,
    on_request_delete: EventHandler<MouseEvent>,
) -> Element {
    let fallback = service
        .title
        .chars()
        .next()
        .map(|ch| ch.to_ascii_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    rsx! {
        div { class: "admin-service-card",
            div { class: "admin-service-header",
                div { class: "admin-service-avatar",
                    if let Some(icon_path) = &service.icon_path {
                        img { class: "admin-service-avatar-img", src: "{icon_path}", alt: "{service.title} icon" }
                    } else {
                        span { class: "admin-service-avatar-fallback", "{fallback}" }
                    }
                }

                div { class: "admin-service-meta",
                    div { class: "admin-service-topline",
                        p { class: "admin-service-title", "{service.title}" }
                        span { class: "admin-service-category", "{service.category}" }
                    }
                    p { class: "admin-service-url", "{service.url}" }
                }
            }

            div { class: "admin-service-body",
                p { class: "admin-service-description", "{service.description}" }

                div { class: "admin-service-chips",
                    if let Some(icon_name) = &service.icon_name {
                        span { class: "admin-service-chip", "Icon: {icon_name}" }
                    }
                    if service.github_url.as_ref().is_some_and(|url| !url.is_empty()) {
                        span { class: "admin-service-chip", "GitHub linked" }
                    }
                }

                div { class: "admin-service-links",
                    if let Some(github_url) = &service.github_url {
                        if !github_url.is_empty() {
                            a {
                                class: "admin-inline-link",
                                href: "{github_url}",
                                target: "_blank",
                                "View source"
                            }
                        }
                    }

                    a {
                        class: "admin-inline-link",
                        href: "{service.url}",
                        target: "_blank",
                        "Open service"
                    }
                }
            }

            div { class: "admin-icon-actions",
                button {
                    class: "admin-btn admin-btn-secondary",
                    onclick: move |e| on_start_edit.call(e),
                    "Edit"
                }
                button {
                    class: "admin-btn admin-btn-danger",
                    onclick: move |e| on_request_delete.call(e),
                    "Delete"
                }
            }
        }
    }
}

#[component]
fn IconCard(
    icon: IconRecord,
    on_start_edit: EventHandler<MouseEvent>,
    on_request_delete: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "admin-icon-card",
            div { class: "admin-icon-preview",
                img {
                    class: "admin-icon-img",
                    src: "{icon.path}",
                    alt: "{icon.name}",
                }
            }

            div { class: "admin-icon-info",
                p { class: "admin-icon-name", "{icon.name}" }
            }
            div { class: "admin-icon-actions",
                button {
                    class: "admin-btn admin-btn-secondary",
                    onclick: move |e| on_start_edit.call(e),
                    "Edit"
                }
                button {
                    class: "admin-btn admin-btn-danger",
                    onclick: move |e| on_request_delete.call(e),
                    "Delete"
                }
            }
        }
    }
}

fn validate_service_form(
    title: &str,
    url: &str,
    description: &str,
    category: &str,
) -> Option<String> {
    if title.is_empty() {
        return Some("Please enter a title.".into());
    }

    if url.is_empty() {
        return Some("Please enter a service URL.".into());
    }

    if description.is_empty() {
        return Some("Please enter a description.".into());
    }

    if category.is_empty() {
        return Some("Please enter a category.".into());
    }

    None
}

fn optional_string(value: String) -> Option<String> {
    let value = value.trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn selected_icon_value(icon_id: Option<i64>) -> String {
    icon_id.map(|id| id.to_string()).unwrap_or_default()
}

fn parse_optional_i64(value: &str) -> Option<i64> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        value.parse().ok()
    }
}

fn read_file_to_signal(
    event: Event<FormData>,
    mut b64_signal: Signal<Option<String>>,
    mut ext_signal: Signal<Option<String>>,
    mut label_signal: Signal<String>,
) {
    spawn(async move {
        let files = event.files();
        if let Some(file) = files.into_iter().next() {
            let name = file.name();
            let ext = std::path::Path::new(&name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("bin")
                .to_lowercase();

            if let Ok(bytes) = file.read_bytes().await {
                use base64::Engine;
                let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
                b64_signal.set(Some(encoded));
                ext_signal.set(Some(ext));
                label_signal.set(name);
            }
        }
    });
}

#[component]
fn Modal(title: String, is_open: bool, children: Element, on_close: EventHandler<()>) -> Element {
    rsx! {
        if is_open {
            // Overlay and modal are siblings so the modal's backdrop-filter
            // blurs the actual page content, not the overlay in front of it.
            div { class: "admin-modal-overlay", onclick: move |_| on_close.call(()) }
            div {
                class: "admin-modal",
                onclick: move |e| e.stop_propagation(),
                div { class: "admin-modal-header",
                    h2 { class: "admin-modal-title", "{title}" }
                    button {
                        class: "admin-modal-close",
                        onclick: move |_| on_close.call(()),
                        "×"
                    }
                }
                div { class: "admin-modal-body",
                    {children}
                }
            }
        }
    }
}
