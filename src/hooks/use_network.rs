//! Network status hook using the Network Information API.
//!
//! This hook provides access to the device's network status including
//! online state and connection type.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use js_sys::Reflect;

/// Network status information.
#[derive(Clone, Debug, PartialEq)]
pub struct NetworkStatus {
    /// Whether the device is online.
    pub online: bool,
    /// Connection type (wifi, cellular, ethernet, etc.).
    pub connection_type: String,
    /// Effective connection type (4g, 3g, 2g, slow-2g).
    pub effective_type: String,
    /// Downlink speed in Mbps.
    pub downlink: Option<f64>,
}

impl Default for NetworkStatus {
    fn default() -> Self {
        Self {
            online: true,
            connection_type: "wifi".to_string(),
            effective_type: "4g".to_string(),
            downlink: None,
        }
    }
}

impl NetworkStatus {
    /// Get a display string for the connection status.
    pub fn status_display(&self) -> &'static str {
        if self.online { "Connected" } else { "Not Connected" }
    }

    /// Get a display string for the connection type.
    pub fn type_display(&self) -> String {
        match self.connection_type.as_str() {
            "wifi" => "Wi-Fi".to_string(),
            "cellular" => "Cellular".to_string(),
            "ethernet" => "Ethernet".to_string(),
            "none" => "None".to_string(),
            _ => "Wi-Fi".to_string(),
        }
    }
}

/// Hook to access network status.
///
/// Uses navigator.onLine and Network Information API for real-time network info.
/// Falls back to default values if APIs are not available.
pub fn use_network() -> RwSignal<NetworkStatus> {
    let network_status = RwSignal::new(get_network_status());

    Effect::new(move |_| {
        let win = match window() {
            Some(w) => w,
            None => return,
        };

        // Update on online/offline events
        let update_online = Closure::wrap(Box::new(move || {
            network_status.set(get_network_status());
        }) as Box<dyn Fn()>);

        let _ = win.add_event_listener_with_callback(
            "online",
            update_online.as_ref().unchecked_ref(),
        );
        let _ = win.add_event_listener_with_callback(
            "offline",
            update_online.as_ref().unchecked_ref(),
        );

        // Try to add connection change listener
        if let Some(connection) = get_connection_object() {
            let update_connection = Closure::wrap(Box::new(move || {
                network_status.set(get_network_status());
            }) as Box<dyn Fn()>);

            let _ = Reflect::set(
                &connection,
                &"onchange".into(),
                update_connection.as_ref(),
            );

            update_connection.forget();
        }

        update_online.forget();
    });

    network_status
}

/// Get current network status.
fn get_network_status() -> NetworkStatus {
    let win = match window() {
        Some(w) => w,
        None => return NetworkStatus::default(),
    };

    let online = win.navigator().on_line();

    let (connection_type, effective_type, downlink) = get_connection_info();

    NetworkStatus {
        online,
        connection_type,
        effective_type,
        downlink,
    }
}

/// Get the Network Information API connection object.
fn get_connection_object() -> Option<js_sys::Object> {
    let win = window()?;
    let navigator = win.navigator();

    // Try navigator.connection
    let connection = Reflect::get(&navigator, &"connection".into()).ok()?;
    if !connection.is_undefined() && !connection.is_null() {
        return connection.dyn_into::<js_sys::Object>().ok();
    }

    // Try navigator.mozConnection (Firefox)
    let connection = Reflect::get(&navigator, &"mozConnection".into()).ok()?;
    if !connection.is_undefined() && !connection.is_null() {
        return connection.dyn_into::<js_sys::Object>().ok();
    }

    // Try navigator.webkitConnection (older Chrome)
    let connection = Reflect::get(&navigator, &"webkitConnection".into()).ok()?;
    if !connection.is_undefined() && !connection.is_null() {
        return connection.dyn_into::<js_sys::Object>().ok();
    }

    None
}

/// Get connection info from Network Information API.
fn get_connection_info() -> (String, String, Option<f64>) {
    let connection = match get_connection_object() {
        Some(c) => c,
        None => return ("wifi".to_string(), "4g".to_string(), None),
    };

    let connection_type = Reflect::get(&connection, &"type".into())
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| "wifi".to_string());

    let effective_type = Reflect::get(&connection, &"effectiveType".into())
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| "4g".to_string());

    let downlink = Reflect::get(&connection, &"downlink".into())
        .ok()
        .and_then(|v| v.as_f64());

    (connection_type, effective_type, downlink)
}
