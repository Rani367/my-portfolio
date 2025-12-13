//! Battery status hook using the Web Battery API.
//!
//! This hook provides access to the device's battery status including
//! charge level, charging state, and time estimates.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Object, Reflect, Promise};

/// Battery status information.
#[derive(Clone, Debug, PartialEq)]
pub struct BatteryStatus {
    /// Battery charge level (0.0 to 1.0).
    pub level: f64,
    /// Whether the battery is currently charging.
    pub charging: bool,
    /// Time in seconds until the battery is fully charged (if charging).
    pub charging_time: Option<f64>,
    /// Time in seconds until the battery is fully discharged (if discharging).
    pub discharging_time: Option<f64>,
}

impl Default for BatteryStatus {
    fn default() -> Self {
        Self {
            level: 1.0,
            charging: true,
            charging_time: None,
            discharging_time: None,
        }
    }
}

impl BatteryStatus {
    /// Get battery level as a percentage (0-100).
    pub fn percentage(&self) -> i32 {
        (self.level * 100.0).round() as i32
    }

    /// Get a display string for the battery status.
    pub fn display(&self) -> String {
        let percent = self.percentage();
        if self.charging {
            format!("{}% (Charging)", percent)
        } else {
            format!("{}%", percent)
        }
    }
}

/// Hook to access battery status.
///
/// Uses the Web Battery API to get real-time battery information.
/// Falls back to default values if the API is not available.
///
/// # Returns
/// A  that updates when battery status changes.
pub fn use_battery() -> RwSignal<BatteryStatus> {
    let battery_status = RwSignal::new(BatteryStatus::default());

    // Spawn async task to get battery info
    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(status) = get_battery_status().await {
                battery_status.set(status);
            }
        });
    });

    battery_status
}

/// Get battery status from the Web Battery API.
async fn get_battery_status() -> Option<BatteryStatus> {
    let win = window()?;
    let navigator = win.navigator();

    // Try to get the battery manager
    // Note: getBattery() is not in web-sys, so we use js_sys to call it
    let get_battery = Reflect::get(&navigator, &"getBattery".into()).ok()?;
    
    if get_battery.is_undefined() || get_battery.is_null() {
        return None;
    }

    // Call getBattery() which returns a Promise
    let get_battery_fn = get_battery.dyn_ref::<js_sys::Function>()?;
    let promise = get_battery_fn.call0(&navigator).ok()?;
    let promise = promise.dyn_into::<Promise>().ok()?;

    // Await the promise
    let battery_manager = JsFuture::from(promise).await.ok()?;
    let battery_obj = battery_manager.dyn_ref::<Object>()?;

    // Extract battery properties
    let level = Reflect::get(battery_obj, &"level".into())
        .ok()?
        .as_f64()
        .unwrap_or(1.0);

    let charging = Reflect::get(battery_obj, &"charging".into())
        .ok()?
        .as_bool()
        .unwrap_or(true);

    let charging_time = Reflect::get(battery_obj, &"chargingTime".into())
        .ok()
        .and_then(|v| v.as_f64())
        .filter(|t| t.is_finite());

    let discharging_time = Reflect::get(battery_obj, &"dischargingTime".into())
        .ok()
        .and_then(|v| v.as_f64())
        .filter(|t| t.is_finite());

    Some(BatteryStatus {
        level,
        charging,
        charging_time,
        discharging_time,
    })
}
