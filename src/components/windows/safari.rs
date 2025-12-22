//! Safari browser window component.

use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::state::use_notification_state;

/// Safari browser content component.
#[component]
pub fn SafariContent() -> impl IntoView {
    let notifications = use_notification_state();

    let (url, set_url) = signal("https://www.google.com/webhp?igu=1".to_string());
    let (input_value, set_input_value) = signal("https://www.google.com".to_string());
    let (is_focused, set_is_focused) = signal(false);

    // Navigation history
    let (history, set_history) = signal(vec!["https://www.google.com/webhp?igu=1".to_string()]);
    let (history_index, set_history_index) = signal(0usize);

    // Computed: can go back/forward
    let can_go_back = Memo::new(move |_| history_index.get() > 0);
    let can_go_forward = Memo::new(move |_| {
        history.with(|h| history_index.get() < h.len().saturating_sub(1))
    });

    // Extract domain from URL for display
    let display_text = Memo::new(move |_| {
        if is_focused.get() {
            input_value.get()
        } else {
            let url = input_value.get();
            url.replace("https://", "")
                .replace("http://", "")
                .split('/')
                .next()
                .unwrap_or(&url)
                .replace("www.", "")
                .to_string()
        }
    });

    // Navigate to URL and add to history
    let navigate_to = move |new_url: String| {
        set_history.update(|h| {
            let idx = history_index.get();
            // Truncate forward history when navigating to new page
            h.truncate(idx + 1);
            h.push(new_url.clone());
        });
        set_history_index.update(|i| *i += 1);
        set_url.set(new_url.clone());
        set_input_value.set(new_url);
    };

    let on_submit = move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        let input = input_value.get().trim().to_string();

        if input.is_empty() {
            return;
        }

        let is_url = input.starts_with("http://")
            || input.starts_with("https://")
            || (input.contains('.') && !input.contains(' '));

        let new_url = if is_url {
            if input.starts_with("http://") || input.starts_with("https://") {
                input.clone()
            } else {
                format!("https://{}", input)
            }
        } else {
            let encoded_query = js_sys::encode_uri_component(&input);
            format!("https://www.google.com/search?igu=1&q={}", encoded_query)
        };

        navigate_to(new_url);

        // Blur the input after submit
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.active_element() {
                if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                    let _ = html_el.blur();
                }
            }
        }
    };

    let on_input = move |e: web_sys::Event| {
        if let Some(target) = e.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                set_input_value.set(input.value());
            }
        }
    };

    let on_focus = move |_| set_is_focused.set(true);
    let on_blur = move |_| set_is_focused.set(false);

    // Back button handler
    let on_back = move |_| {
        if can_go_back.get() {
            set_history_index.update(|i| *i = i.saturating_sub(1));
            let new_url = history.with(|h| h.get(history_index.get()).cloned().unwrap_or_default());
            set_url.set(new_url.clone());
            set_input_value.set(new_url);
        }
    };

    // Forward button handler
    let on_forward = move |_| {
        if can_go_forward.get() {
            set_history_index.update(|i| *i += 1);
            let new_url = history.with(|h| h.get(history_index.get()).cloned().unwrap_or_default());
            set_url.set(new_url.clone());
            set_input_value.set(new_url);
        }
    };

    // Reload button handler
    let on_reload = move |_| {
        let current_url = url.get();
        // Force reload by setting to empty then back
        set_url.set(String::new());
        // Use a small delay to ensure the iframe updates
        let url_to_restore = current_url;
        set_timeout(
            move || set_url.set(url_to_restore),
            std::time::Duration::from_millis(50),
        );
    };

    // Share button handler - copy URL to clipboard using proper Clipboard API
    let on_share = move |_| {
        let current_url = input_value.get();
        let notifications = notifications.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let _ = JsFuture::from(clipboard.write_text(&current_url)).await;
            }
            notifications.show("/public/images/safari.png", "Safari", "Copied", "URL copied to clipboard");
        });
    };

    view! {
        <div class="safari-toolbar">
            <div class="safari-toolbar-left">
                <button class="safari-btn" disabled=true title="Show Sidebar">
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <rect x="1.5" y="2.5" width="13" height="11" rx="1.5" stroke="currentColor" stroke-width="1.2"/>
                        <path d="M5.5 2.5V13.5" stroke="currentColor" stroke-width="1.2"/>
                    </svg>
                </button>
                <div class="safari-nav-group">
                    <button
                        class="safari-btn"
                        title="Back"
                        disabled=move || !can_go_back.get()
                        on:click=on_back
                    >
                        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                            <path d="M10 3L5 8L10 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                    </button>
                    <button
                        class="safari-btn"
                        title="Forward"
                        disabled=move || !can_go_forward.get()
                        on:click=on_forward
                    >
                        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                            <path d="M6 3L11 8L6 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                    </button>
                </div>
            </div>

            <form class="safari-address-bar" on:submit=on_submit class:focused=is_focused>
                <div class="safari-address-content">
                    <svg class="safari-lock-icon" width="12" height="12" viewBox="0 0 12 12" fill="none">
                        <path d="M3.5 5.5V4C3.5 2.61929 4.61929 1.5 6 1.5C7.38071 1.5 8.5 2.61929 8.5 4V5.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
                        <rect x="2.5" y="5.5" width="7" height="5" rx="1" stroke="currentColor" stroke-width="1.2"/>
                    </svg>
                    <input
                        type="text"
                        class="safari-address-input"
                        prop:value=move || display_text.get()
                        on:input=on_input
                        on:focus=on_focus
                        on:blur=on_blur
                        placeholder="Search or enter website name"
                    />
                    <button type="button" class="safari-reload-btn" title="Reload" on:click=on_reload>
                        <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                            <path d="M11.5 7C11.5 9.48528 9.48528 11.5 7 11.5C4.51472 11.5 2.5 9.48528 2.5 7C2.5 4.51472 4.51472 2.5 7 2.5C8.5 2.5 9.83333 3.25 10.6 4.4" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
                            <path d="M8.5 4.5H11V2" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                    </button>
                </div>
            </form>

            <div class="safari-toolbar-right">
                <button class="safari-btn" title="Share" on:click=on_share>
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path d="M8 10V2.5M8 2.5L5 5.5M8 2.5L11 5.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
                        <path d="M3 8.5V12.5C3 13.0523 3.44772 13.5 4 13.5H12C12.5523 13.5 13 13.0523 13 12.5V8.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
                    </svg>
                </button>
                <button class="safari-btn" disabled=true title="New Tab">
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <rect x="2" y="4" width="12" height="9" rx="1.5" stroke="currentColor" stroke-width="1.2"/>
                        <path d="M2 6.5H14" stroke="currentColor" stroke-width="1.2"/>
                        <circle cx="3.5" cy="5.25" r="0.5" fill="currentColor"/>
                        <circle cx="5" cy="5.25" r="0.5" fill="currentColor"/>
                        <circle cx="6.5" cy="5.25" r="0.5" fill="currentColor"/>
                    </svg>
                </button>
            </div>
        </div>
        <iframe
            src=move || url.get()
            class="safari-iframe"
            sandbox="allow-same-origin allow-scripts allow-popups allow-forms"
        ></iframe>
    }
}
