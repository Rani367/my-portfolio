//! Main dock component with glass effect and magnification.

use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use std::rc::Rc;
use std::cell::RefCell;

use crate::data::dock_apps::DOCK_APPS;
use super::dock_item::DockItem;

/// Type alias for the animation frame closure to reduce complexity.
type AnimationFrameClosure = Rc<RefCell<Option<Closure<dyn Fn(f64)>>>>;

/// Magnification constants
const MAX_SCALE: f64 = 1.5;
const MAX_TRANSLATE_Y: f64 = -20.0;
const EFFECT_DISTANCE: f64 = 120.0;

/// Lerp factor - controls animation smoothness (0.1 = slow, 0.25 = snappy)
const LERP_FACTOR: f64 = 0.18;

/// Animated state for each dock item
#[derive(Clone, Copy)]
struct DockItemState {
    current_scale: f64,
    current_ty: f64,
}

impl Default for DockItemState {
    fn default() -> Self {
        Self {
            current_scale: 1.0,
            current_ty: 0.0,
        }
    }
}

/// The macOS-style dock with magnification effect.
/// Uses a single requestAnimationFrame loop for smooth, GPU-accelerated animations.
#[component]
pub fn Dock() -> impl IntoView {
    // Track mouse X position for magnification
    let (mouse_x, set_mouse_x) = signal(0.0_f64);
    // Track if mouse is hovering over dock
    let (is_hovering, set_is_hovering) = signal(false);

    // Create NodeRefs for all dock items - one per app
    let dock_refs: Vec<NodeRef<Div>> = DOCK_APPS.iter().map(|_| NodeRef::new()).collect();
    let dock_refs_for_loop = dock_refs.clone();

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

    // Single RAF loop for all dock item animations with lerping
    Effect::new(move |_| {
        let dock_refs = dock_refs_for_loop.clone();

        // Animated state for each dock item - persists across frames
        let animated_state: Rc<RefCell<Vec<DockItemState>>> = Rc::new(RefCell::new(
            vec![DockItemState::default(); DOCK_APPS.len()]
        ));

        let f: AnimationFrameClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_timestamp: f64| {
            // Use get_untracked to avoid triggering Leptos reactivity
            let mx = mouse_x.get_untracked();
            let hovering = is_hovering.get_untracked();

            let mut state = animated_state.borrow_mut();

            for (i, node_ref) in dock_refs.iter().enumerate() {
                if let Some(el) = node_ref.get() {
                    // Get the underlying web_sys HtmlElement
                    let html_el: &web_sys::HtmlElement = &el;
                    let rect = html_el.get_bounding_client_rect();
                    let center = rect.left() + rect.width() / 2.0;
                    let distance = (mx - center).abs();

                    // Calculate TARGET values (what we want to reach)
                    let (target_scale, target_ty) = if hovering && distance < EFFECT_DISTANCE {
                        let ratio = 1.0 - distance / EFFECT_DISTANCE;
                        let eased = ratio * (2.0 - ratio); // Quadratic ease-out
                        (1.0 + (MAX_SCALE - 1.0) * eased, MAX_TRANSLATE_Y * eased)
                    } else {
                        (1.0, 0.0) // Rest state
                    };

                    // Lerp CURRENT values towards target for smooth animation
                    state[i].current_scale += (target_scale - state[i].current_scale) * LERP_FACTOR;
                    state[i].current_ty += (target_ty - state[i].current_ty) * LERP_FACTOR;

                    // Apply current (animated) values directly to DOM
                    let _ = html_el.style().set_property(
                        "transform",
                        &format!("scale({:.3}) translateY({:.1}px)", state[i].current_scale, state[i].current_ty)
                    );
                }
            }

            // Request next frame - keep running for smooth return-to-rest animation
            if let Some(win) = window() {
                if let Some(ref closure) = *f.borrow() {
                    let _ = win.request_animation_frame(closure.as_ref().unchecked_ref());
                }
            }
        }) as Box<dyn Fn(f64)>));

        // Start the animation loop
        if let Some(win) = window() {
            if let Some(ref closure) = *g.borrow() {
                let _ = win.request_animation_frame(closure.as_ref().unchecked_ref());
            }
        }

        // Note: The closure is kept alive by the Rc and runs for the lifetime of the component
    });

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
                    let node_ref = dock_refs[index];
                    view! {
                        // Add divider before the last item (trash)
                        {if is_last {
                            Some(view! { <div class="dock-divider"></div> })
                        } else {
                            None
                        }}
                        <DockItem
                            app=app
                            node_ref=node_ref
                        />
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
