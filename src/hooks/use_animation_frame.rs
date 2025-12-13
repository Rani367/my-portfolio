//! Animation frame hook for smooth animations.
//!
//! This hook provides a wrapper around requestAnimationFrame for
//! creating smooth, 60fps animations in components.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

/// Return type for the animation frame hook.
pub struct UseAnimationFrameReturn {
    /// Whether the animation is currently running.
    pub is_running: RwSignal<bool>,
}

/// Hook to run a callback on each animation frame.
///
/// The animation starts automatically when the component mounts.
/// Use the returned is_running signal to stop it.
///
/// # Arguments
/// * callback - Function called on each animation frame with the elapsed time in ms.
///
/// # Returns
/// A UseAnimationFrameReturn with the running state signal.
pub fn use_animation_frame<F>(callback: F) -> UseAnimationFrameReturn
where
    F: Fn(f64) + 'static + Clone + Send + Sync,
{
    let is_running = RwSignal::new(true);
    let start_time = RwSignal::new(None::<f64>);

    Effect::new(move |_| {
        let callback = callback.clone();
        
        // We need to use a recursive closure pattern for RAF
        let f: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn Fn(f64)>>>> = 
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
            if !is_running.get() {
                return;
            }

            // Calculate elapsed time
            let elapsed = match start_time.get() {
                Some(start) => timestamp - start,
                None => {
                    start_time.set(Some(timestamp));
                    0.0
                }
            };

            // Call the user callback
            callback(elapsed);

            // Request next frame
            if is_running.get() {
                if let Some(win) = window() {
                    if let Some(ref closure) = *f.borrow() {
                        let _ = win.request_animation_frame(closure.as_ref().unchecked_ref());
                    }
                }
            }
        }) as Box<dyn Fn(f64)>));

        // Start the animation loop
        if let Some(win) = window() {
            if let Some(ref closure) = *g.borrow() {
                let _ = win.request_animation_frame(closure.as_ref().unchecked_ref());
            }
        }

        // Note: The closure is kept alive by the Rc, and will stop
        // when is_running is set to false
    });

    UseAnimationFrameReturn { is_running }
}

/// Request a single animation frame.
/// 
/// This is a simple wrapper around requestAnimationFrame that calls
/// the callback once on the next frame.
pub fn request_animation_frame<F>(callback: F)
where
    F: FnOnce(f64) + 'static,
{
    let closure = Closure::once(Box::new(callback) as Box<dyn FnOnce(f64)>);
    
    if let Some(win) = window() {
        let _ = win.request_animation_frame(closure.as_ref().unchecked_ref());
    }
    
    closure.forget();
}

/// Easing functions for animations.
pub mod easing {
    /// Linear easing (no easing).
    pub fn linear(t: f64) -> f64 {
        t
    }

    /// Ease in quadratic.
    pub fn ease_in_quad(t: f64) -> f64 {
        t * t
    }

    /// Ease out quadratic.
    pub fn ease_out_quad(t: f64) -> f64 {
        t * (2.0 - t)
    }

    /// Ease in-out quadratic.
    pub fn ease_in_out_quad(t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }

    /// Ease out cubic.
    pub fn ease_out_cubic(t: f64) -> f64 {
        let t = t - 1.0;
        t * t * t + 1.0
    }

    /// Ease in-out cubic.
    pub fn ease_in_out_cubic(t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let t = 2.0 * t - 2.0;
            0.5 * t * t * t + 1.0
        }
    }
}
