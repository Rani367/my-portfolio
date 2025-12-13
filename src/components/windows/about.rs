//! About This Mac window component.

use leptos::prelude::*;

/// About This Mac content component.
#[component]
pub fn AboutContent() -> impl IntoView {
    view! {
        <img
            src="/public/images/logo.svg"
            alt="Apple"
            class="about-logo"
        />
        <h1 class="about-title">"macOS Portfolio"</h1>
        <p class="about-version">"Version 2.0.0"</p>
        <p class="about-version" style="color: #f77f00; font-weight: 600;">"100% Rust — Zero JavaScript"</p>
        <div class="about-specs">
            <p><strong>"Developer:"</strong>" Rani Malach"</p>
            <p><strong>"Framework:"</strong>" Leptos 0.8"</p>
            <p><strong>"Target:"</strong>" WebAssembly"</p>
        </div>
        <p class="about-footer">"Made with attention to detail"</p>
    }
}
