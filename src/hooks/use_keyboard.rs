//! Keyboard event hooks including Konami code detection.
//!
//! This module provides hooks for handling keyboard events and detecting
//! special key sequences like the Konami code easter egg.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{KeyboardEvent, window};

/// The Konami code sequence: Up, Up, Down, Down, Left, Right, Left, Right, B, A
const KONAMI_CODE: [&str; 10] = [
    "ArrowUp", "ArrowUp", "ArrowDown", "ArrowDown",
    "ArrowLeft", "ArrowRight", "ArrowLeft", "ArrowRight",
    "KeyB", "KeyA"
];

/// Return type for the keyboard hook.
pub struct UseKeyboardReturn {
    /// Signal that fires when Konami code is entered.
    pub konami_activated: RwSignal<bool>,
    /// The last key pressed.
    pub last_key: RwSignal<Option<String>>,
}

/// Hook to listen for global keyboard events.
///
/// This hook sets up a global keydown listener and tracks key sequences
/// to detect the Konami code easter egg.
///
/// # Returns
/// A UseKeyboardReturn with signals for Konami activation and last key pressed.
pub fn use_keyboard() -> UseKeyboardReturn {
    let konami_activated = RwSignal::new(false);
    let last_key = RwSignal::new(None::<String>);
    let key_sequence = RwSignal::new(Vec::<String>::new());

    // Set up the keyboard listener on mount
    Effect::new(move |_| {
        let win = window().expect("no global window exists");

        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let key = e.code();
            last_key.set(Some(key.clone()));

            // Update key sequence for Konami code detection
            key_sequence.update(|seq| {
                seq.push(key.clone());
                // Keep only the last 10 keys
                if seq.len() > 10 {
                    seq.remove(0);
                }

                // Debug: log the current sequence
                #[cfg(debug_assertions)]
                web_sys::console::log_1(&format!("Key sequence: {:?}", seq).into());

                // Check for Konami code
                if seq.len() == 10 {
                    let matches = seq.iter()
                        .zip(KONAMI_CODE.iter())
                        .all(|(a, b)| a == *b);
                    if matches {
                        #[cfg(debug_assertions)]
                        web_sys::console::log_1(&"Konami code activated!".into());
                        konami_activated.update(|v| *v = !*v);
                        seq.clear();
                    }
                }
            });
        }) as Box<dyn Fn(KeyboardEvent)>);

        win.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("failed to add keydown listener");

        // Prevent closure from being dropped
        closure.forget();
    });

    UseKeyboardReturn {
        konami_activated,
        last_key,
    }
}

/// Simplified hook that just returns the Konami activation signal.
pub fn use_konami_code() -> RwSignal<bool> {
    use_keyboard().konami_activated
}

/// Modifier key requirements for keyboard shortcuts.
#[derive(Clone, Copy, Default)]
pub struct KeyModifiers {
    pub cmd: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyModifiers {
    /// Create modifiers with Cmd/Meta key required.
    pub fn cmd() -> Self {
        Self { cmd: true, ..Default::default() }
    }

    /// Create modifiers with no keys required.
    pub fn none() -> Self {
        Self::default()
    }
}

/// Set up a keyboard shortcut listener.
/// 
/// Note: This sets up an effect that listens for the specified key combination.
/// The callback will be called when the shortcut is pressed.
pub fn setup_keyboard_shortcut(
    key: &'static str,
    modifiers: KeyModifiers,
    on_activate: impl Fn() + 'static + Clone + Send + Sync,
) {
    Effect::new(move |_| {
        let win = window().expect("no global window exists");
        let callback = on_activate.clone();

        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let key_matches = e.code() == key;
            let mods_match = 
                e.meta_key() == modifiers.cmd &&
                e.ctrl_key() == modifiers.ctrl &&
                e.alt_key() == modifiers.alt &&
                e.shift_key() == modifiers.shift;

            if key_matches && mods_match {
                e.prevent_default();
                callback();
            }
        }) as Box<dyn Fn(KeyboardEvent)>);

        win.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("failed to add keydown listener");

        closure.forget();
    });
}
