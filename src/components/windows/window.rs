//! Base window component for the portfolio application.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

use crate::state::{use_app_state, WindowId};

/// Base window component.
#[component]
pub fn Window(
    /// The window identifier.
    id: WindowId,
    /// The window title displayed in the header.
    #[prop(into)]
    title: String,
    /// Window width in pixels.
    #[prop(default = 800)]
    width: i32,
    /// Window height in pixels.
    #[prop(default = 500)]
    height: i32,
    /// Initial top position in pixels.
    #[prop(default = 60)]
    initial_top: i32,
    /// Initial left position in pixels.
    #[prop(default = 80)]
    initial_left: i32,
    /// Optional icon to display in the title bar.
    #[prop(optional, into)]
    icon: Option<String>,
    /// Optional additional class for window body.
    #[prop(optional, into)]
    body_class: Option<String>,
    /// The window content.
    children: Children,
) -> impl IntoView {
    let app_state = use_app_state();
    let window_id = id;

    // Local position state for dragging
    let (pos_x, set_pos_x) = signal(initial_left);
    let (pos_y, set_pos_y) = signal(initial_top);
    let (is_dragging, set_is_dragging) = signal(false);
    let (drag_offset_x, set_drag_offset_x) = signal(0);
    let (drag_offset_y, set_drag_offset_y) = signal(0);

    // Window state from global state
    let is_open = {
        let app_state = app_state.clone();
        Memo::new(move |_| {
            app_state.windows.get()
                .get(&window_id)
                .map(|w| w.is_open)
                .unwrap_or(false)
        })
    };

    let is_focused = {
        let app_state = app_state.clone();
        Memo::new(move |_| {
            app_state.windows.get()
                .get(&window_id)
                .map(|w| w.is_focused)
                .unwrap_or(false)
        })
    };

    let is_closing = {
        let app_state = app_state.clone();
        Memo::new(move |_| {
            app_state.windows.get()
                .get(&window_id)
                .map(|w| w.is_closing)
                .unwrap_or(false)
        })
    };

    let is_minimizing = {
        let app_state = app_state.clone();
        Memo::new(move |_| {
            app_state.windows.get()
                .get(&window_id)
                .map(|w| w.is_minimizing)
                .unwrap_or(false)
        })
    };

    let z_index = {
        let app_state = app_state.clone();
        Memo::new(move |_| {
            app_state.windows.get()
                .get(&window_id)
                .map(|w| w.z_index)
                .unwrap_or(1000)
        })
    };

    // Window style - always compute, only apply when active
    let window_style = Memo::new(move |_| {
        format!(
            "left: {}px; top: {}px; width: {}px; height: {}px; z-index: {};",
            pos_x.get(),
            pos_y.get(),
            width,
            height,
            z_index.get()
        )
    });

    // Window class - build based on state
    let window_class = Memo::new(move |_| {
        let mut classes = vec!["window"];
        if is_open.get() {
            classes.push("active");
        }
        if is_focused.get() {
            classes.push("focused");
        }
        if is_closing.get() {
            classes.push("closing");
        }
        if is_minimizing.get() {
            classes.push("minimizing");
        }
        classes.join(" ")
    });

    // Close handler - just starts the animation
    let app_state_close = app_state.clone();
    let on_close = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        // Start closing animation
        app_state_close.close_window(window_id);
        // Complete after animation (200ms like JS)
        let app_state_complete = app_state_close.clone();
        set_timeout(
            move || {
                app_state_complete.close_window_complete(window_id);
            },
            std::time::Duration::from_millis(200),
        );
    };

    // Minimize handler
    let app_state_minimize = app_state.clone();
    let on_minimize = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        app_state_minimize.minimize_window(window_id);
        let app_state_complete = app_state_minimize.clone();
        set_timeout(
            move || {
                app_state_complete.minimize_window_complete(window_id);
            },
            std::time::Duration::from_millis(350),
        );
    };

    // Focus handler - only focus, don't do anything else
    let app_state_focus = app_state.clone();
    let on_window_mousedown = move |_: MouseEvent| {
        // Don't focus if window is not open
        if is_open.get() {
            app_state_focus.focus_window(window_id);
        }
    };

    // Drag start handler - only on header, not on controls
    let on_header_mousedown = move |e: MouseEvent| {
        // Don't start drag if clicking on controls or nav buttons
        let target = e.target();
        if let Some(target) = target {
            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                if element.closest(".control").ok().flatten().is_some() ||
                   element.closest(".nav-btn").ok().flatten().is_some() {
                    return;
                }
            }
        }

        if e.button() == 0 {
            e.prevent_default();
            set_is_dragging.set(true);
            set_drag_offset_x.set(e.client_x() - pos_x.get());
            set_drag_offset_y.set(e.client_y() - pos_y.get());
        }
    };

    // Global mouse move for dragging
    let on_mousemove = move |e: MouseEvent| {
        if is_dragging.get() {
            let new_x = e.client_x() - drag_offset_x.get();
            let new_y = (e.client_y() - drag_offset_y.get()).max(28);
            set_pos_x.set(new_x);
            set_pos_y.set(new_y);
        }
    };

    // Global mouse up to stop dragging
    let on_mouseup = move |_: MouseEvent| {
        set_is_dragging.set(false);
    };

    // Render icon if present
    let icon_view = icon.map(|i| view! {
        <img src=i class="window-title-icon" alt="" />
    });

    // Render children once
    let children_view = children();

    // Build body class
    let body_class_str = match body_class {
        Some(bc) => format!("window-body {}", bc),
        None => "window-body".to_string(),
    };

    // The window is always rendered, visibility controlled by "active" class (matching JS behavior)
    view! {
        <div
            class=move || window_class.get()
            style=move || window_style.get()
            on:mousedown=on_window_mousedown
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
        >
            <div class="window-header" on:mousedown=on_header_mousedown>
                <div class="window-controls">
                    <button class="control close" on:click=on_close></button>
                    <button class="control minimize" on:click=on_minimize></button>
                    <button class="control maximize"></button>
                </div>
                <div class="window-title">
                    {icon_view}
                    <span class="title-text">{title}</span>
                </div>
            </div>
            <div class=body_class_str>
                {children_view}
            </div>
        </div>
    }
}
