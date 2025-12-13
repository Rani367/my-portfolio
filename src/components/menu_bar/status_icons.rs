//! Status icons for the menu bar (WiFi and Battery).
//!
//! These components display status icons with tooltips that show
//! detailed information when clicked.

use leptos::prelude::*;
use crate::hooks::use_battery;

/// WiFi icon with tooltip showing connection info.
#[component]
pub fn WifiIcon() -> impl IntoView {
    let (show_tooltip, set_show_tooltip) = signal(false);

    let toggle_tooltip = move |_: web_sys::MouseEvent| {
        set_show_tooltip.update(|v| *v = !*v);
    };

    view! {
        <div
            class="menu-icon-wrapper"
            on:click=toggle_tooltip
        >
            <img
                src="/public/icons/wifi.svg"
                alt="WiFi"
                class="menu-icon"
            />
            <div class=move || {
                if show_tooltip.get() {
                    "status-tooltip active"
                } else {
                    "status-tooltip"
                }
            }>
                <div class="status-row">
                    <span class="status-label">"Wi-Fi"</span>
                    <span class="status-value">"Connected"</span>
                </div>
                <div class="status-row">
                    <span class="status-label">"Network"</span>
                    <span class="status-value">"Portfolio_5G"</span>
                </div>
                <div class="status-row">
                    <span class="status-label">"IP Address"</span>
                    <span class="status-value">"192.168.1.42"</span>
                </div>
            </div>
        </div>
    }
}

/// Battery icon with tooltip showing battery status.
#[component]
pub fn BatteryIcon() -> impl IntoView {
    let battery_status = use_battery();
    let (show_tooltip, set_show_tooltip) = signal(false);

    let toggle_tooltip = move |_: web_sys::MouseEvent| {
        set_show_tooltip.update(|v| *v = !*v);
    };

    let battery_percentage = move || battery_status.get().percentage();
    let battery_display = move || battery_status.get().display();
    let is_charging = move || battery_status.get().charging;
    let battery_width = move || format!("{}%", battery_percentage());

    // Calculate time remaining display
    let time_remaining = move || {
        let status = battery_status.get();
        if status.charging {
            status.charging_time
                .map(|t| {
                    let hours = (t / 3600.0) as i32;
                    let mins = ((t % 3600.0) / 60.0) as i32;
                    format!("{}:{:02}", hours, mins)
                })
                .unwrap_or_else(|| "Calculating...".to_string())
        } else {
            status.discharging_time
                .map(|t| {
                    let hours = (t / 3600.0) as i32;
                    let mins = ((t % 3600.0) / 60.0) as i32;
                    format!("{}:{:02}", hours, mins)
                })
                .unwrap_or_else(|| "Calculating...".to_string())
        }
    };

    let power_source = move || {
        if is_charging() { "Power Adapter" } else { "Battery" }
    };

    view! {
        <div
            class="menu-icon-wrapper"
            on:click=toggle_tooltip
        >
            <img
                src="/public/icons/battery.svg"
                alt="Battery"
                class="menu-icon"
            />
            <div class=move || {
                if show_tooltip.get() {
                    "status-tooltip active"
                } else {
                    "status-tooltip"
                }
            }>
                <div class="status-row">
                    <span class="status-label">"Battery"</span>
                    <div class="status-battery">
                        <div class="battery-icon">
                            <div
                                class="battery-level"
                                style:width=battery_width
                            ></div>
                        </div>
                        <span class="status-value">{battery_display}</span>
                    </div>
                </div>
                <div class="status-row">
                    <span class="status-label">"Power Source"</span>
                    <span class="status-value">{power_source}</span>
                </div>
                <div class="status-row">
                    <span class="status-label">"Time Remaining"</span>
                    <span class="status-value">{time_remaining}</span>
                </div>
            </div>
        </div>
    }
}

/// Container for all status icons in the menu bar.
#[component]
pub fn StatusIcons() -> impl IntoView {
    view! {
        <WifiIcon />
        <BatteryIcon />
    }
}
