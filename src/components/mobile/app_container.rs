//! Full-screen mobile app container with swipe-to-close gesture.
//!
//! Wraps app content in a full-screen view with a header and back button.
//! Supports swipe-down gesture to close the app.

use leptos::prelude::*;
use js_sys::Reflect;
use web_sys::TouchEvent;
use crate::state::{use_app_state, WindowId};

/// Velocity threshold (pixels per millisecond) to trigger close.
const VELOCITY_THRESHOLD: f64 = 0.5;
/// Animation duration in milliseconds.
const ANIMATION_DURATION: u64 = 300;

/// Get viewport-relative swipe threshold (15% of viewport height, min 50px).
fn get_swipe_threshold() -> f64 {
    web_sys::window()
        .and_then(|w| w.inner_height().ok())
        .and_then(|h| h.as_f64())
        .map(|h| (h * 0.15).max(50.0))
        .unwrap_or(100.0)
}

/// Get the Y position from a touch event's first touch point using Reflect.
fn get_touch_y(event: &TouchEvent) -> Option<f64> {
    let touches = Reflect::get(event, &"touches".into()).ok()?;
    let first_touch = Reflect::get(&touches, &0.into()).ok()?;
    Reflect::get(&first_touch, &"clientY".into())
        .ok()
        .and_then(|v| v.as_f64())
}

/// Full-screen container for a mobile app.
#[component]
pub fn MobileAppContainer(
    /// The window ID for this app.
    id: WindowId,
    /// The title shown in the header.
    title: &'static str,
    /// The app content.
    children: Children,
) -> impl IntoView {
    let app_state = use_app_state();

    // Swipe gesture state
    let (touch_start_y, set_touch_start_y) = signal(0.0_f64);
    let (touch_start_time, set_touch_start_time) = signal(0.0_f64);
    let (current_offset, set_current_offset) = signal(0.0_f64);
    let (is_swiping, set_is_swiping) = signal(false);
    let (is_closing, set_is_closing) = signal(false);

    // Check if this app is active
    let is_active = Memo::new(move |_| {
        app_state.mobile_active_app.get() == Some(id)
    });

    // Toggle body scroll lock and handle transition completion
    {
        let app_state = app_state.clone();
        Effect::new(move |prev_active: Option<bool>| {
            let active = is_active.get();

            // Handle body scroll lock
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        if active {
                            let _ = body.class_list().add_1("mobile-app-open");
                        } else {
                            let _ = body.class_list().remove_1("mobile-app-open");
                        }
                    }
                }
            }

            // Mark transition complete after open animation
            if active && prev_active != Some(true) {
                let app_state = app_state.clone();
                set_timeout(
                    move || {
                        app_state.mobile_transition_complete();
                    },
                    std::time::Duration::from_millis(ANIMATION_DURATION),
                );
            }

            active
        });
    }

    // Handle touch start
    let on_touch_start = move |e: TouchEvent| {
        if let Some(y) = get_touch_y(&e) {
            set_touch_start_y.set(y);
            set_touch_start_time.set(js_sys::Date::now());
            set_is_swiping.set(true);
            set_current_offset.set(0.0);
        }
    };

    // Handle touch move
    let on_touch_move = move |e: TouchEvent| {
        if !is_swiping.get() {
            return;
        }

        if let Some(y) = get_touch_y(&e) {
            let delta = y - touch_start_y.get();
            // Only allow downward swipe (positive delta)
            if delta > 0.0 {
                set_current_offset.set(delta);
            }
        }
    };

    // Handle touch end
    let on_touch_end = {
        let app_state = app_state.clone();
        move |_: TouchEvent| {
            if !is_swiping.get() || is_closing.get() {
                return;
            }

            let offset = current_offset.get();
            let elapsed = js_sys::Date::now() - touch_start_time.get();
            let velocity = if elapsed > 0.0 { offset / elapsed } else { 0.0 };
            let threshold = get_swipe_threshold();

            // Check if swipe threshold is met
            if offset > threshold || velocity > VELOCITY_THRESHOLD {
                // Trigger close animation
                set_is_closing.set(true);
                // Close after animation
                let app_state = app_state.clone();
                set_timeout(
                    move || {
                        app_state.mobile_close_app();
                        app_state.mobile_transition_complete();
                        set_is_closing.set(false);
                        set_current_offset.set(0.0);
                    },
                    std::time::Duration::from_millis(ANIMATION_DURATION),
                );
            } else {
                // Snap back
                set_current_offset.set(0.0);
            }

            set_is_swiping.set(false);
        }
    };

    // Handle back button click
    let on_back_click = {
        let app_state = app_state.clone();
        move |_| {
            // Prevent double-close if already closing
            if is_closing.get() {
                return;
            }
            set_is_closing.set(true);
            let app_state = app_state.clone();
            set_timeout(
                move || {
                    app_state.mobile_close_app();
                    app_state.mobile_transition_complete();
                    set_is_closing.set(false);
                },
                std::time::Duration::from_millis(ANIMATION_DURATION),
            );
        }
    };

    // Compute container style for swipe animation
    let container_style = move || {
        let offset = current_offset.get();
        if offset > 0.0 && !is_closing.get() {
            let scale = 1.0 - (offset / 1000.0).min(0.1);
            let radius = (offset / 10.0).min(40.0);
            format!(
                "transform: translateY({:.0}px) scale({:.3}); border-radius: {:.0}px;",
                offset, scale, radius
            )
        } else {
            String::new()
        }
    };

    // CSS classes for the container
    let container_class = move || {
        let mut classes = vec!["mobile-app-container"];
        if is_active.get() {
            classes.push("active");
        }
        if is_closing.get() {
            classes.push("closing");
        }
        classes.join(" ")
    };

    view! {
        <div
            class=container_class
            style=container_style
            on:touchstart=on_touch_start
            on:touchmove=on_touch_move
            on:touchend=on_touch_end
        >
            <div class="mobile-app-header">
                <button
                    class="mobile-back-btn"
                    on:click=on_back_click
                    aria-label="Return to home screen"
                >
                    <svg class="mobile-back-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
                        <path d="M15 18l-6-6 6-6"/>
                    </svg>
                    "Home"
                </button>
                <span class="mobile-app-title">{title}</span>
                <div class="mobile-header-spacer"></div>
            </div>
            <div class="mobile-app-content">
                {children()}
            </div>
            <div class="mobile-home-indicator"></div>
        </div>
    }
}
