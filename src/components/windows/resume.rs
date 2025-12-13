//! Resume PDF viewer component.

use leptos::prelude::*;

/// Resume PDF viewer content using browser's native PDF rendering.
#[component]
pub fn ResumeContent() -> impl IntoView {
    view! {
        <div class="pdf-container">
            <embed
                src="/public/files/resume.pdf"
                type="application/pdf"
                width="100%"
                height="100%"
            />
        </div>
    }
}
