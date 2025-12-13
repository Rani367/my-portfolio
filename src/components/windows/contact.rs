//! Contact window component with social links.

use leptos::prelude::*;
use crate::data::SOCIALS;

/// Contact content component showing social links.
#[component]
pub fn ContactContent() -> impl IntoView {
    view! {
        <div class="contact-content">
            <h2 class="contact-title">"Get in Touch"</h2>
            <p class="contact-subtitle">"Feel free to reach out through any of these platforms"</p>
            <div class="socials-grid">
                {SOCIALS.iter().map(|social| {
                    let bg_style = format!("background-color: {};", social.bg);
                    view! {
                        <a
                            class="social-card"
                            href=social.link
                            target="_blank"
                            style=bg_style
                        >
                            <img src=social.icon alt=social.text class="social-icon" />
                            <span class="social-text">{social.text}</span>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
