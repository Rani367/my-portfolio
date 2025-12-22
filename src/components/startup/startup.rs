//! 3D typing startup screen component.

use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;
use web_sys::window;

/// Automatically enabled in release builds, disabled in debug builds.
const STARTUP_SCREEN_ENABLED: bool = !cfg!(debug_assertions);

/// The name to type out on startup.
const NAME: &str = "Hi, I'm Rani";

/// Base typing speed in milliseconds.
const BASE_TYPING_MS: u32 = 160;

/// Random variation range (±ms) - smaller for smoother typing.
const TYPING_VARIATION_MS: u32 = 40;

/// Check if we're on a mobile device.
fn is_mobile() -> bool {
    window()
        .and_then(|w| w.inner_width().ok())
        .and_then(|w| w.as_f64())
        .map(|w| w < 768.0)
        .unwrap_or(false)
}

/// Generate a human-like random delay for typing.
fn human_typing_delay(char_index: usize, ch: char) -> u32 {
    // Simple pseudo-random based on character and position
    let seed = (char_index as u32 * 7 + ch as u32 * 13) % 100;

    // Base delay with random variation
    let variation = (seed % (TYPING_VARIATION_MS * 2)) as i32 - TYPING_VARIATION_MS as i32;
    let delay = (BASE_TYPING_MS as i32 + variation).max(80) as u32;

    // Add slight pause after space (between words)
    if ch == ' ' {
        delay + 80
    }
    // Occasional subtle pause
    else if seed < 8 {
        delay + 60
    } else {
        delay
    }
}

/// 3D typing startup screen that types out "Rani Malach".
#[component]
pub fn StartupScreen() -> impl IntoView {
    // Skip rendering if disabled
    if !STARTUP_SCREEN_ENABLED {
        return ().into_any();
    }

    let mobile = is_mobile();

    // On mobile, show full text immediately; on desktop, type it out
    let initial_text = if mobile { NAME.to_string() } else { String::new() };

    let (displayed_text, set_displayed_text) = signal(initial_text);
    let (is_typing_done, set_is_typing_done) = signal(mobile); // Already done on mobile
    let (is_fading, set_is_fading) = signal(false);
    let (is_removed, set_is_removed) = signal(false);

    // Typing animation (only runs the typing part on desktop)
    spawn_local(async move {
        if mobile {
            // On mobile, just wait and fade out
            TimeoutFuture::new(2000).await;
        } else {
            // Desktop: Initial delay before typing starts
            TimeoutFuture::new(500).await;

            // Type out each character with human-like timing
            let chars: Vec<char> = NAME.chars().collect();
            for (i, ch) in chars.iter().enumerate() {
                let delay = human_typing_delay(i, *ch);
                TimeoutFuture::new(delay).await;
                let text: String = chars[..=i].iter().collect();
                set_displayed_text.set(text);
            }

            // Mark typing as done
            set_is_typing_done.set(true);

            // Wait a moment to show the complete text before fading
            TimeoutFuture::new(1200).await;
        }

        // Start fade out
        set_is_fading.set(true);

        // Wait for fade-out animation to complete
        TimeoutFuture::new(500).await;
        set_is_removed.set(true);
    });

    view! {
        {move || {
            if is_removed.get() {
                ().into_any()
            } else {
                view! {
                    <div
                        id="startup-screen"
                        class=move || {
                            let mut classes = String::from("startup-3d");
                            if is_fading.get() {
                                classes.push_str(" fade-out");
                            }
                            classes
                        }
                    >
                        <div class="startup-3d-content">
                            <h1 class=move || {
                                let mut classes = String::from("startup-3d-text");
                                if is_typing_done.get() {
                                    classes.push_str(" typing-done");
                                }
                                classes
                            }>
                                {move || displayed_text.get()}
                                <span class=move || {
                                    if is_typing_done.get() { "cursor hidden" } else { "cursor" }
                                }>"|"</span>
                            </h1>
                        </div>
                    </div>
                }.into_any()
            }
        }}
    }.into_any()
}
