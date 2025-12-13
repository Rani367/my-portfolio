//! Finder window component with file navigation.

use leptos::prelude::*;
use web_sys::window;

use crate::data::{FileItem, FileType, ItemKind, LOCATIONS, get_location};
use crate::state::{use_app_state, use_navigation_state, WindowId};

/// The Finder window content component.
/// This renders the sidebar and main content area.
#[component]
pub fn FinderContent() -> impl IntoView {
    let nav_state = use_navigation_state();
    let app_state = use_app_state();

    // Get current location
    let nav_state_location = nav_state.clone();
    let current_location = Memo::new(move |_| {
        let current = nav_state_location.current();
        get_location(current.location_type)
    });

    // Get current items to display
    let nav_state_items = nav_state.clone();
    let current_items = Memo::new(move |_| {
        if let Some(location) = current_location.get() {
            nav_state_items.get_current_items(location)
        } else {
            &[] as &[FileItem]
        }
    });

    view! {
        // Sidebar
        <div class="finder-sidebar">
            <div class="sidebar-section">
                <span class="sidebar-title">"Favorites"</span>
                <div class="sidebar-items">
                    <SidebarLocations />
                </div>
            </div>
        </div>

        // Main content area
        <div class="finder-content">
            <FileGrid items=current_items app_state=app_state nav_state=nav_state />
        </div>
    }
}

/// Sidebar locations component.
#[component]
fn SidebarLocations() -> impl IntoView {
    let nav_state = use_navigation_state();

    view! {
        {LOCATIONS.iter().map(|location| {
            let nav_state = nav_state.clone();
            let location_type = location.location_type;
            let nav_state_active = nav_state.clone();
            let is_active = move || nav_state_active.current().location_type == location_type;
            let on_click = move |_| {
                nav_state.navigate_to_location(location_type);
            };
            view! {
                <div
                    class=move || if is_active() { "sidebar-item active" } else { "sidebar-item" }
                    on:click=on_click
                >
                    <img src=location.icon alt="" />
                    <span>{location.name}</span>
                </div>
            }
        }).collect_view()}
    }
}

/// File grid component.
#[component]
fn FileGrid(
    items: Memo<&'static [FileItem]>,
    app_state: crate::state::AppState,
    nav_state: crate::state::NavigationState,
) -> impl IntoView {
    view! {
        {move || {
            items.get().iter().map(|item| {
                let nav_state = nav_state.clone();
                let app_state = app_state.clone();
                let on_click = move |_| {
                    match item.kind {
                        ItemKind::Folder => {
                            nav_state.navigate_into_folder(item.id);
                        }
                        ItemKind::File => {
                            match item.file_type {
                                Some(FileType::Txt) => {
                                    app_state.open_txt_file(item);
                                }
                                Some(FileType::Img) => {
                                    app_state.open_img_file(item);
                                }
                                Some(FileType::Url) | Some(FileType::Fig) => {
                                    if let Some(href) = item.href {
                                        if let Some(win) = window() {
                                            let _ = win.open_with_url_and_target(href, "_blank");
                                        }
                                    }
                                }
                                Some(FileType::Pdf) => {
                                    app_state.open_window(WindowId::Resume);
                                }
                                None => {}
                            }
                        }
                    }
                };

                let item_style = item.position.map(|pos| {
                    let mut style = String::new();
                    for part in pos.split_whitespace() {
                        let parts: Vec<&str> = part.split('-').collect();
                        if parts.len() == 2 {
                            let prop = parts[0];
                            let val: i32 = parts[1].parse().unwrap_or(0);
                            let px = val * 4;
                            style.push_str(&format!("{}: {}px; ", prop, px));
                        }
                    }
                    if !style.is_empty() {
                        style.push_str("position: absolute;");
                    }
                    style
                }).unwrap_or_default();

                let class_name = "file-item";

                view! {
                    <div
                        class=class_name
                        style=item_style
                        on:click=on_click
                    >
                        <img src=item.icon alt="" class="file-item-icon" />
                        <span class="file-item-name">{item.name}</span>
                    </div>
                }
            }).collect_view()
        }}
    }
}
