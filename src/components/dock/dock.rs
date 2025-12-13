//! Main dock component with glass effect and magnification.

use leptos::prelude::*;
use crate::data::dock_apps::DOCK_APPS;
use super::dock_item::DockItem;

/// The macOS-style dock with magnification effect.
#[component]
pub fn Dock() -> impl IntoView {
    // Track mouse X position for magnification
    let (mouse_x, set_mouse_x) = signal(0.0_f64);
    // Track if mouse is hovering over dock
    let (is_hovering, set_is_hovering) = signal(false);

    // Mouse event handlers
    let on_mouse_move = move |e: web_sys::MouseEvent| {
        set_mouse_x.set(e.client_x() as f64);
    };

    let on_mouse_enter = move |_| {
        set_is_hovering.set(true);
    };

    let on_mouse_leave = move |_| {
        set_is_hovering.set(false);
    };

    view! {
        <div id="dock">
            <div
                id="dock-container"
                class="dock-container"
                on:mousemove=on_mouse_move
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
            >
                {DOCK_APPS.iter().enumerate().map(|(index, app)| {
                    let is_last = index == DOCK_APPS.len() - 1;
                    view! {
                        // Add divider before the last item (trash)
                        {if is_last {
                            Some(view! { <div class="dock-divider"></div> })
                        } else {
                            None
                        }}
                        <DockItem
                            app=app
                            mouse_x=mouse_x
                            is_hovering=is_hovering
                        />
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
