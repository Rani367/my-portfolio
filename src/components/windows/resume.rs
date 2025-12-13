//! Resume viewer component.

use leptos::prelude::*;

/// Resume content using Canva embed.
#[component]
pub fn ResumeContent() -> impl IntoView {
    view! {
        <div class="pdf-container">
            <div class="resume-toolbar">
                <a
                    href="/public/files/resume.pdf"
                    download="Rani_Malach_Resume.pdf"
                    class="resume-download-btn"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7 10 12 15 17 10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    "Download Resume"
                </a>
            </div>
            <iframe
                src="https://www.canva.com/design/DAG3wRQVrnA/vd_irZM1MCJzI5_l70veew/view?embed"
                width="100%"
                height="100%"
                style="border: none;"
                allowfullscreen="true"
                allow="fullscreen"
            />
        </div>
    }
}
