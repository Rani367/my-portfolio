//! Spotlight search overlay component.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::{use_app_state, use_navigation_state, WindowId};
use crate::data::{DOCK_APPS, LOCATIONS};

/// Search result item.
#[derive(Clone, Debug, PartialEq)]
struct SearchResult {
    name: String,
    icon: String,
    subtitle: String,
    action: SearchAction,
}

/// Action to perform when selecting a search result.
#[derive(Clone, Debug, PartialEq)]
enum SearchAction {
    OpenWindow(WindowId),
    NavigateToLocation(&'static str),
}

/// Spotlight search overlay component.
#[component]
pub fn Spotlight() -> impl IntoView {
    let app_state = use_app_state();
    let nav_state = use_navigation_state();

    let (query, set_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0usize);

    // Check if spotlight is open
    let is_open = Memo::new({
        let app_state = app_state.clone();
        move |_| app_state.spotlight_open.get()
    });

    // Compute search results - show all apps when empty, filter when typing
    let results = Memo::new(move |_| {
        let q = query.get().to_lowercase();
        let mut results = Vec::new();

        // Search dock apps
        for app in DOCK_APPS.iter() {
            if q.is_empty() || app.name.to_lowercase().contains(&q) {
                if let Some(window_id) = WindowId::from_dock_id(app.id) {
                    results.push(SearchResult {
                        name: app.name.to_string(),
                        icon: format!("/public/images/{}", app.icon),
                        subtitle: "Application".to_string(),
                        action: SearchAction::OpenWindow(window_id),
                    });
                }
            }
        }

        // Search locations (only when typing)
        if !q.is_empty() {
            for location in LOCATIONS.iter() {
                if location.name.to_lowercase().contains(&q) {
                    results.push(SearchResult {
                        name: location.name.to_string(),
                        icon: location.icon.to_string(),
                        subtitle: "Folder".to_string(),
                        action: SearchAction::NavigateToLocation(location.location_type),
                    });
                }
            }
        }

        // Limit to 8 results
        results.truncate(8);
        results
    });

    // Handle input change
    let on_input = move |e: web_sys::Event| {
        let target = e.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        set_query.set(input.value());
        set_selected_index.set(0);
    };

    // Handle keyboard navigation
    let on_keydown = {
        let app_state = app_state.clone();
        let nav_state = nav_state.clone();
        move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            let results_len = results.get().len();

            match key.as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if results_len > 0 {
                        set_selected_index.update(|i| {
                            *i = (*i + 1) % results_len;
                        });
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if results_len > 0 {
                        set_selected_index.update(|i| {
                            *i = if *i == 0 { results_len - 1 } else { *i - 1 };
                        });
                    }
                }
                "Enter" => {
                    e.prevent_default();
                    let results_vec = results.get();
                    if let Some(result) = results_vec.get(selected_index.get()) {
                        match &result.action {
                            SearchAction::OpenWindow(window_id) => {
                                app_state.open_window(*window_id);
                            }
                            SearchAction::NavigateToLocation(loc_type) => {
                                app_state.open_window(WindowId::Finder);
                                nav_state.navigate_to_location(loc_type);
                            }
                        }
                        app_state.close_spotlight();
                        set_query.set(String::new());
                    }
                }
                "Escape" => {
                    app_state.close_spotlight();
                    set_query.set(String::new());
                }
                _ => {}
            }
        }
    };

    // Handle overlay click (close spotlight)
    let app_state_close = app_state.clone();
    let on_overlay_click = move |_| {
        app_state_close.close_spotlight();
        set_query.set(String::new());
    };

    // Handle result click
    let execute_result = {
        let app_state = app_state.clone();
        let nav_state = nav_state.clone();
        move |result: SearchResult| {
            match result.action {
                SearchAction::OpenWindow(window_id) => {
                    app_state.open_window(window_id);
                }
                SearchAction::NavigateToLocation(loc_type) => {
                    app_state.open_window(WindowId::Finder);
                    nav_state.navigate_to_location(loc_type);
                }
            }
            app_state.close_spotlight();
            set_query.set(String::new());
        }
    };

    view! {
        <div
            id="spotlight-overlay"
            class=move || if is_open.get() { "active" } else { "" }
            on:click=on_overlay_click
        >
            <div id="spotlight" on:click=|e| e.stop_propagation()>
                <div class="spotlight-input-wrapper">
                    <svg viewBox="0 0 20 20" fill="currentColor">
                        <path
                            fill-rule="evenodd"
                            d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                            clip-rule="evenodd"
                        />
                    </svg>
                    <input
                        type="text"
                        id="spotlight-input"
                        placeholder="Spotlight Search"
                        autocomplete="off"
                        prop:value=move || query.get()
                        on:input=on_input
                        on:keydown=on_keydown
                    />
                </div>
                <div class="spotlight-results" id="spotlight-results">
                    {move || {
                        let results_vec = results.get();
                        let selected = selected_index.get();

                        if results_vec.is_empty() {
                            view! {}.into_any()
                        } else {
                            // Group by type
                            let apps: Vec<_> = results_vec.iter()
                                .enumerate()
                                .filter(|(_, r)| r.subtitle == "Application")
                                .collect();
                            let folders: Vec<_> = results_vec.iter()
                                .enumerate()
                                .filter(|(_, r)| r.subtitle == "Folder")
                                .collect();

                            view! {
                                {if !apps.is_empty() {
                                    Some(view! {
                                        <div class="spotlight-section">
                                            <div class="spotlight-section-title">"Applications"</div>
                                            {apps.iter().map(|(idx, result)| {
                                                let is_selected = *idx == selected;
                                                let result_clone = (*result).clone();
                                                let exec = execute_result.clone();
                                                view! {
                                                    <div
                                                        class=if is_selected { "spotlight-item selected" } else { "spotlight-item" }
                                                        on:click=move |_| exec(result_clone.clone())
                                                    >
                                                        <img src=result.icon.clone() alt="" class="spotlight-item-icon" />
                                                        <div class="spotlight-item-text">
                                                            <div class="spotlight-item-title">{result.name.clone()}</div>
                                                            <div class="spotlight-item-subtitle">{result.subtitle.clone()}</div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    })
                                } else { None }}

                                {if !folders.is_empty() {
                                    Some(view! {
                                        <div class="spotlight-section">
                                            <div class="spotlight-section-title">"Folders"</div>
                                            {folders.iter().map(|(idx, result)| {
                                                let is_selected = *idx == selected;
                                                let result_clone = (*result).clone();
                                                let exec = execute_result.clone();
                                                view! {
                                                    <div
                                                        class=if is_selected { "spotlight-item selected" } else { "spotlight-item" }
                                                        on:click=move |_| exec(result_clone.clone())
                                                    >
                                                        <img src=result.icon.clone() alt="" class="spotlight-item-icon" />
                                                        <div class="spotlight-item-text">
                                                            <div class="spotlight-item-title">{result.name.clone()}</div>
                                                            <div class="spotlight-item-subtitle">{result.subtitle.clone()}</div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    })
                                } else { None }}
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
