//! iOS-style home screen with app grid.
//!
//! Displays apps in a grid layout similar to iOS home screen.

use leptos::prelude::*;
use crate::data::dock_apps::DOCK_APPS;
use crate::state::{use_app_state, WindowId};
use super::status_bar::MobileStatusBar;

/// Individual app icon on the home screen.
#[component]
fn MobileAppIcon(
    id: &'static str,
    label: &'static str,
    icon: &'static str,
    #[prop(default = true)] show_label: bool,
) -> impl IntoView {
    let app_state = use_app_state();

    let on_tap = move |_| {
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

    // Filter apps that can be opened
    let apps: Vec<_> = DOCK_APPS.iter()
        .filter(|app| app.can_open)
        .collect();

    view! {
        <div
            class="mobile-home-screen"
            class:visible=move || is_visible.get()
        >
            <MobileStatusBar />

            <div class="mobile-app-grid">
                {apps.iter().map(|app| view! {
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
                    {DOCK_APPS.iter()
                        .take(4)
                        .filter(|app| app.can_open)
                        .map(|app| view! {
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
