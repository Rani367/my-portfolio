//! Window control buttons (close, minimize, maximize).
//!
//! This component renders the traffic light buttons in the window header
//! with proper hover states and click handlers.

use leptos::prelude::*;

/// Window control buttons component (traffic lights).
///
/// Renders the red (close), yellow (minimize), and green (maximize) buttons
/// that appear in the top-left corner of macOS windows.
#[component]
pub fn WindowControls(
    /// Callback when close button is clicked.
    on_close: Callback<()>,
    /// Callback when minimize button is clicked.
    on_minimize: Callback<()>,
    /// Callback when maximize button is clicked.
    on_maximize: Callback<()>,
) -> impl IntoView {
    let is_hovering = RwSignal::new(false);

    view! {
        <div
            class="window-controls"
            on:mouseenter=move |_| is_hovering.set(true)
            on:mouseleave=move |_| is_hovering.set(false)
        >
            <button
                class="control close"
                on:click=move |e| {
                    e.stop_propagation();
                    on_close.run(());
                }
            >
                <Show when=move || is_hovering.get()>
                    <svg viewBox="0 0 12 12" class="control-icon">
                        <line x1="3" y1="3" x2="9" y2="9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                        <line x1="9" y1="3" x2="3" y2="9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                    </svg>
                </Show>
            </button>
            <button
                class="control minimize"
                on:click=move |e| {
                    e.stop_propagation();
                    on_minimize.run(());
                }
            >
                <Show when=move || is_hovering.get()>
                    <svg viewBox="0 0 12 12" class="control-icon">
                        <line x1="2" y1="6" x2="10" y2="6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                    </svg>
                </Show>
            </button>
            <button
                class="control maximize"
                on:click=move |e| {
                    e.stop_propagation();
                    on_maximize.run(());
                }
            >
                <Show when=move || is_hovering.get()>
                    <svg viewBox="0 0 12 12" class="control-icon">
                        <path d="M2 4 L6 2 L10 4 L10 8 L6 10 L2 8 Z" fill="none" stroke="currentColor" stroke-width="1" stroke-linejoin="round"/>
                    </svg>
                </Show>
            </button>
        </div>
    }
}

/// Simplified window controls that only show close button.
#[component]
pub fn WindowControlsSimple(
    on_close: Callback<()>,
) -> impl IntoView {
    let is_hovering = RwSignal::new(false);

    view! {
        <div
            class="window-controls"
            on:mouseenter=move |_| is_hovering.set(true)
            on:mouseleave=move |_| is_hovering.set(false)
        >
            <button
                class="control close"
                on:click=move |e| {
                    e.stop_propagation();
                    on_close.run(());
                }
            >
                <Show when=move || is_hovering.get()>
                    <svg viewBox="0 0 12 12" class="control-icon">
                        <line x1="3" y1="3" x2="9" y2="9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                        <line x1="9" y1="3" x2="3" y2="9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                    </svg>
                </Show>
            </button>
            <button class="control minimize disabled"></button>
            <button class="control maximize disabled"></button>
        </div>
    }
}
