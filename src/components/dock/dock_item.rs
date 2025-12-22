//! Individual dock item component.
//! Magnification is handled by the parent Dock component via direct DOM manipulation.

use leptos::prelude::*;
use leptos::html::Div;
use crate::data::dock_apps::DockApp;
use crate::state::app_state::use_app_state;
use crate::state::window_state::WindowId;

/// Individual dock item with icon, tooltip, and click handler.
/// Transform animations are applied directly by the parent Dock component.
#[component]
pub fn DockItem(
    app: &'static DockApp,
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let app_state = use_app_state();
    let app_state_click = app_state.clone();

    // Check if window is open for indicator
    let window_id = WindowId::from_dock_id(app.id);
    let is_open = Memo::new(move |_| {
        window_id.map(|id| app_state.is_window_open(id)).unwrap_or(false)
    });

    // Click handler
    let on_click = move |_| {
        if app.can_open {
            if let Some(id) = WindowId::from_dock_id(app.id) {
                if app_state_click.is_window_open(id) {
                    app_state_click.focus_window(id);
                } else {
                    app_state_click.open_window(id);
                }
            }
        } else if app.id == "trash" {
            app_state_click.open_window(WindowId::Finder);
        }
    };

    // Class names - only tracks open state, not animation
    let class_names = move || {
        let mut classes = vec!["dock-item"];
        if is_open.get() {
            classes.push("open");
        }
        classes.join(" ")
    };

    let aria_label = format!("Open {}", app.name);

    view! {
        <div
            node_ref=node_ref
            class=class_names
            on:click=on_click
            role="button"
            tabindex="0"
            aria-label=aria_label
        >
            <span class="dock-tooltip">{app.name}</span>
            <img
                src=format!("/public/images/{}", app.icon)
                alt=app.name
                class="dock-icon"
            />
            <div class="dock-indicator"></div>
        </div>
    }
}
