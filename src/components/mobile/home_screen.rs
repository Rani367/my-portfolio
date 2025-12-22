//! iOS-style home screen with app grid.
//!
//! Displays apps in a grid layout similar to iOS home screen.
//! Dock and grid apps are separated to prevent duplicate click handlers.

use leptos::prelude::*;
use crate::data::dock_apps::DOCK_APPS;
use crate::state::{use_app_state, WindowId};
use super::status_bar::MobileStatusBar;

/// App IDs that appear in the dock (bottom bar).
/// These apps are excluded from the main grid to prevent duplicates.
const DOCK_APP_IDS: &[&str] = &["finder", "safari", "contact", "terminal"];

/// Additional apps for the grid that aren't in DOCK_APPS.
struct GridApp {
    id: &'static str,
    name: &'static str,
    icon: &'static str,
}

const EXTRA_GRID_APPS: &[GridApp] = &[
    GridApp { id: "resume", name: "Resume", icon: "pdf.png" },
    GridApp { id: "about", name: "About", icon: "info.png" },
];

/// Individual app icon on the home screen.
#[component]
fn MobileAppIcon(
    id: &'static str,
    label: &'static str,
    icon: &'static str,
    #[prop(default = true)] show_label: bool,
) -> impl IntoView {
    let app_state = use_app_state();

    let on_tap = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
        if let Some(window_id) = WindowId::from_dock_id(id) {
            app_state.mobile_open_app(window_id);
        }
    };

    view! {
        <button class="mobile-app-icon" on:click=on_tap>
            <div class="mobile-app-icon-wrapper">
                <img src=format!("/public/images/{}", icon) alt=label />
            </div>
            {show_label.then(|| view! { <span class="mobile-app-label">{label}</span> })}
        </button>
    }
}

/// iOS-style home screen grid.
#[component]
pub fn MobileHomeScreen() -> impl IntoView {
    let app_state = use_app_state();

    // Only show when no app is active
    let is_visible = Memo::new(move |_| {
        app_state.mobile_active_app.get().is_none()
    });

    // Grid apps: DOCK_APPS that can_open but NOT in dock, plus extra apps
    let grid_apps: Vec<_> = DOCK_APPS.iter()
        .filter(|app| app.can_open && !DOCK_APP_IDS.contains(&app.id))
        .collect();

    // Dock apps: only those in DOCK_APP_IDS
    let dock_apps: Vec<_> = DOCK_APP_IDS.iter()
        .filter_map(|id| DOCK_APPS.iter().find(|app| app.id == *id && app.can_open))
        .collect();

    view! {
        <div
            class="mobile-home-screen"
            class:visible=move || is_visible.get()
        >
            <MobileStatusBar />

            <div class="mobile-app-grid">
                // Apps from DOCK_APPS that are NOT in dock
                {grid_apps.iter().map(|app| view! {
                    <MobileAppIcon
                        id=app.id
                        label=app.name
                        icon=app.icon
                    />
                }).collect_view()}
                // Extra grid apps (Resume, About)
                {EXTRA_GRID_APPS.iter().map(|app| view! {
                    <MobileAppIcon
                        id=app.id
                        label=app.name
                        icon=app.icon
                    />
                }).collect_view()}
            </div>

            <div class="mobile-dock-bar">
                <div class="mobile-dock-bar-inner">
                    // Primary apps in the dock (no labels)
                    {dock_apps.iter().map(|app| view! {
                        <MobileAppIcon
                            id=app.id
                            label=app.name
                            icon=app.icon
                            show_label=false
                        />
                    }).collect_view()}
                </div>
            </div>

            <div class="mobile-home-indicator"></div>
        </div>
    }
}
