//! Scroll position tracking hook for scroll-reactive animations.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

/// Track scroll position for scroll-reactive animations.
/// Returns (scroll_y, scroll_progress) where progress is 0.0-1.0.
pub fn use_scroll() -> (ReadSignal<f64>, ReadSignal<f64>) {
    let (scroll_y, set_scroll_y) = signal(0.0);
    let (scroll_progress, set_scroll_progress) = signal(0.0);

    Effect::new(move |_| {
        let Some(win) = window() else { return };
        let Some(doc) = win.document() else { return };
        let Some(doc_element) = doc.document_element() else { return };

        let update_scroll = {
            let win = win.clone();
            let doc_element = doc_element.clone();
            Closure::<dyn Fn()>::new(move || {
                let y = win.scroll_y().unwrap_or(0.0);
                let max_scroll = (doc_element.scroll_height() - doc_element.client_height()) as f64;
                let progress = if max_scroll > 0.0 { y / max_scroll } else { 0.0 };

                set_scroll_y.set(y);
                set_scroll_progress.set(progress.clamp(0.0, 1.0));
            })
        };

        // Initial update
        if let Some(func) = update_scroll.as_ref().dyn_ref::<js_sys::Function>() {
            let _ = func.call0(&JsValue::NULL);
        }

        // Listen to scroll events with passive flag for performance
        let _ = win.add_event_listener_with_callback(
            "scroll",
            update_scroll.as_ref().unchecked_ref()
        );

        // Keep closure alive
        update_scroll.forget();
    });

    (scroll_y, scroll_progress)
}

/// Intersection observer hook for reveal-on-scroll animations.
/// Returns a signal that becomes true when element with given ID is visible.
pub fn use_element_in_view(element_id: &'static str, threshold: f64) -> ReadSignal<bool> {
    let (is_visible, set_is_visible) = signal(false);

    Effect::new(move |_| {
        let Some(win) = window() else { return };

        // Small delay to ensure element exists in DOM
        let callback = Closure::<dyn Fn()>::new(move || {
            let Some(doc) = window().and_then(|w| w.document()) else { return };
            let Some(element) = doc.get_element_by_id(element_id) else { return };

            let observer_callback = Closure::<dyn Fn(js_sys::Array, web_sys::IntersectionObserver)>::new(
                move |entries: js_sys::Array, _observer: web_sys::IntersectionObserver| {
                    for entry in entries.iter() {
                        if let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() {
                            if entry.is_intersecting() {
                                set_is_visible.set(true);
                            }
                        }
                    }
                }
            );

            let options = web_sys::IntersectionObserverInit::new();
            options.set_threshold(&JsValue::from_f64(threshold));

            if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
                observer_callback.as_ref().unchecked_ref(),
                &options
            ) {
                observer.observe(&element);
                observer_callback.forget();
            }
        });

        // Use requestAnimationFrame to delay execution until after render
        let _ = win.request_animation_frame(callback.as_ref().unchecked_ref());
        callback.forget();
    });

    is_visible
}
