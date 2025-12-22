//! Mobile device detection hook.
//!
//! This hook detects the current device type based on viewport width
//! and updates the AppState accordingly.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use crate::state::use_app_state;

/// Mobile breakpoint (below this is considered mobile).
const MOBILE_BREAKPOINT: f64 = 768.0;
/// Phone breakpoint (below this is considered a phone).
const PHONE_BREAKPOINT: f64 = 640.0;

/// Device type based on viewport width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceType {
    /// Desktop device (>= 768px).
    Desktop,
    /// Tablet device (640px - 767px).
    Tablet,
    /// Phone device (< 640px).
    Phone,
}

impl DeviceType {
    /// Check if this is a mobile device (phone or tablet).
    pub fn is_mobile(&self) -> bool {
        matches!(self, DeviceType::Phone | DeviceType::Tablet)
    }
}

/// Get the current viewport width.
fn get_viewport_width() -> f64 {
    window()
        .and_then(|w| w.inner_width().ok())
        .and_then(|w| w.as_f64())
        .unwrap_or(1024.0)
}

/// Determine device type from width.
fn device_type_from_width(width: f64) -> DeviceType {
    if width < PHONE_BREAKPOINT {
        DeviceType::Phone
    } else if width < MOBILE_BREAKPOINT {
        DeviceType::Tablet
    } else {
        DeviceType::Desktop
    }
}

/// Hook that tracks device type and updates AppState.
///
/// Returns a memo that updates when the viewport width crosses a breakpoint.
/// Sets up both resize and orientationchange listeners to handle all viewport changes.
pub fn use_mobile_detection() -> Memo<DeviceType> {
    let app_state = use_app_state();
    let (width, set_width) = signal(get_viewport_width());

    // Set up resize and orientation listeners
    Effect::new(move |_| {
        let Some(win) = window() else { return };

        // Set initial width
        set_width.set(get_viewport_width());

        // Create handler for both events
        let update_width = Closure::wrap(Box::new(move |_: web_sys::Event| {
            set_width.set(get_viewport_width());
        }) as Box<dyn Fn(_)>);

        // Add resize listener
        let _ = win.add_event_listener_with_callback(
            "resize",
            update_width.as_ref().unchecked_ref(),
        );

        // Add orientationchange listener (fires before resize on iOS)
        let _ = win.add_event_listener_with_callback(
            "orientationchange",
            update_width.as_ref().unchecked_ref(),
        );

        // Keep closure alive for the lifetime of the app
        update_width.forget();
    });

    // Compute device type from width (pure computation)
    let device_type = Memo::new(move |_| {
        device_type_from_width(width.get())
    });

    // Update AppState in a separate Effect (side effect)
    Effect::new(move |_| {
        let is_mobile = device_type.get().is_mobile();
        app_state.is_mobile.set(is_mobile);
    });

    device_type
}

/// Simple hook to check if currently on mobile.
/// Uses app_state.is_mobile which is set by use_mobile_detection().
/// Call use_mobile_detection() first in the app root.
pub fn use_is_mobile() -> Memo<bool> {
    let app_state = use_app_state();
    Memo::new(move |_| app_state.is_mobile.get())
}
