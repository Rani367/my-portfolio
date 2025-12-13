//! Clock component for the menu bar.
//!
//! Displays the current time in macOS menu bar format.

use leptos::prelude::*;
use crate::hooks::use_clock;

/// Clock component that displays the current time.
///
/// Uses the use_clock hook to get formatted time that updates every minute.
#[component]
pub fn Clock() -> impl IntoView {
    let time = use_clock();

    view! {
        <span class="menu-time">
            {move || time.get()}
        </span>
    }
}
