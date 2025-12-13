//! Safari browser window component.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Safari browser content component.
#[component]
pub fn SafariContent() -> impl IntoView {
    let (url, set_url) = signal("https://www.google.com/webhp?igu=1".to_string());
    let (input_value, set_input_value) = signal("https://www.google.com".to_string());

    let on_submit = move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        let mut new_url = input_value.get();

        // Add https:// if not present
        if !new_url.starts_with("http://") && !new_url.starts_with("https://") {
            new_url = format!("https://{}", new_url);
        }

        set_url.set(new_url.clone());
        set_input_value.set(new_url);
    };

    let on_input = move |e: web_sys::Event| {
        let target = e.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        set_input_value.set(input.value());
    };

    view! {
        <div class="safari-toolbar">
            <div class="safari-nav-buttons">
                <button class="safari-nav-btn" disabled=true>
                    <svg width="12" height="12" viewBox="0 0 12 12">
                        <path d="M7.5 2L3.5 6L7.5 10" stroke="currentColor" stroke-width="1.5" fill="none"/>
                    </svg>
                </button>
                <button class="safari-nav-btn" disabled=true>
                    <svg width="12" height="12" viewBox="0 0 12 12">
                        <path d="M4.5 2L8.5 6L4.5 10" stroke="currentColor" stroke-width="1.5" fill="none"/>
                    </svg>
                </button>
            </div>
            <form class="safari-url-bar" on:submit=on_submit>
                <span class="safari-lock-icon">"🔒"</span>
                <input
                    type="text"
                    class="safari-url-input"
                    prop:value=move || input_value.get()
                    on:input=on_input
                    placeholder="Search or enter website name"
                />
            </form>
            <button class="safari-nav-btn safari-reload">
                <svg width="12" height="12" viewBox="0 0 12 12">
                    <path d="M10 6a4 4 0 11-1-2.5M10 1v2.5h-2.5" stroke="currentColor" stroke-width="1.5" fill="none"/>
                </svg>
            </button>
        </div>
        <iframe
            src=move || url.get()
            class="safari-iframe"
            sandbox="allow-same-origin allow-scripts allow-popups allow-forms"
        ></iframe>
    }
}
