//! iOS-style status bar for mobile.
//!
//! Displays time, network status, and battery in the iOS status bar style.

use leptos::prelude::*;
use crate::hooks::{use_clock, use_battery, use_network};

/// iOS-style status bar component.
#[component]
pub fn MobileStatusBar() -> impl IntoView {
    let time = use_clock();
    let battery = use_battery();
    let network = use_network();

    // Format battery icon based on level
    let battery_icon = Memo::new(move |_| {
        let status = battery.get();
        let level = status.percentage();
        if status.charging {
            "battery-charging"
        } else if level > 75 {
            "battery-full"
        } else if level > 50 {
            "battery-75"
        } else if level > 25 {
            "battery-50"
        } else {
            "battery-low"
        }
    });

    view! {
        <div class="mobile-status-bar">
            <div class="mobile-status-left">
                <span class="mobile-time">{move || time.get()}</span>
            </div>
            <div class="mobile-status-notch">
                // Dynamic Island style notch
            </div>
            <div class="mobile-status-right">
                <span class="mobile-signal">
                    {move || if network.get().online { "5G" } else { "..." }}
                </span>
                <span class="mobile-wifi">
                    <svg class="mobile-wifi-icon" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 18c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm-4.9-2.3l1.4 1.4C9.4 16.4 10.6 16 12 16s2.6.4 3.5 1.1l1.4-1.4c-1.3-1.1-3-1.7-4.9-1.7s-3.6.6-4.9 1.7zm-2.8-2.8l1.4 1.4C7.3 13.1 9.5 12 12 12s4.7 1.1 6.3 2.3l1.4-1.4C17.7 11.1 15 10 12 10s-5.7 1.1-7.7 2.9zM1.5 10.1l1.4 1.4C5.1 9.7 8.4 8 12 8s6.9 1.7 9.1 3.5l1.4-1.4C19.9 7.8 16.1 6 12 6s-7.9 1.8-10.5 4.1z"/>
                    </svg>
                </span>
                <span class=move || format!("mobile-battery {}", battery_icon.get())>
                    {move || format!("{}%", battery.get().percentage())}
                </span>
            </div>
        </div>
    }
}
