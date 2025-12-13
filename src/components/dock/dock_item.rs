//! Individual dock item component with magnification support.

use leptos::prelude::*;
use crate::data::dock_apps::DockApp;
use crate::state::app_state::use_app_state;
use crate::state::window_state::WindowId;

/// Individual dock item with icon, tooltip, and magnification effect.
#[component]
pub fn DockItem(
    app: &'static DockApp,
    mouse_x: ReadSignal<f64>,
    is_hovering: ReadSignal<bool>,
) -> impl IntoView {
    let app_state = use_app_state();
    let app_state_click = app_state.clone();
    let node_ref = NodeRef::<leptos::html::Div>::new();

    // Maximum magnification values
    const MAX_SCALE: f64 = 1.5;
    const MAX_TRANSLATE_Y: f64 = -20.0;
    const EFFECT_DISTANCE: f64 = 120.0;

    // Reactive signal for scale
    let (scale, set_scale) = signal(1.0_f64);
    let (translate_y, set_translate_y) = signal(0.0_f64);

    // Effect that updates on mouse_x and is_hovering changes
    Effect::new(move |_| {
        let hovering = is_hovering.get();
        let mx = mouse_x.get();

        if !hovering {
            set_scale.set(1.0);
            set_translate_y.set(0.0);
            return;
        }

        let Some(el) = node_ref.get() else {
            set_scale.set(1.0);
            set_translate_y.set(0.0);
            return;
        };

        let rect = el.get_bounding_client_rect();
        let item_center = rect.left() + rect.width() / 2.0;
        let distance = (mx - item_center).abs();

        if distance < EFFECT_DISTANCE {
            let ratio = 1.0 - distance / EFFECT_DISTANCE;
            // Quadratic easing for smooth falloff
            let eased_ratio = ratio * (2.0 - ratio);
            set_scale.set(1.0 + (MAX_SCALE - 1.0) * eased_ratio);
            set_translate_y.set(MAX_TRANSLATE_Y * eased_ratio);
        } else {
            set_scale.set(1.0);
            set_translate_y.set(0.0);
        }
    });

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

    // Transform style with CSS transition for smoothness
    let transform_style = move || {
        format!(
            "transform: scale({:.3}) translateY({:.1}px); transition: transform 0.08s ease-out;",
            scale.get(),
            translate_y.get()
        )
    };

    // Class names
    let class_names = move || {
        let mut classes = vec!["dock-item"];
        if is_open.get() {
            classes.push("open");
        }
        classes.join(" ")
    };

    view! {
        <div
            node_ref=node_ref
            class=class_names
            style=transform_style
            on:click=on_click
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
