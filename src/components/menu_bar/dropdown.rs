//! Dropdown menu components for the menu bar.
//!
//! Provides reusable dropdown menu functionality with support for
//! items, dividers, shortcuts, and disabled states.

use leptos::prelude::*;

/// Represents a menu action that can be triggered from dropdown items.
#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    // File menu
    NewWindow,
    NewFolder,
    GetInfo,
    CloseWindow,
    // Go menu
    GoWork,
    GoAbout,
    GoResume,
    GoTrash,
    // Window menu
    Minimize,
    Zoom,
    BringAllToFront,
    // Help menu
    SearchHelp,
    KeyboardShortcuts,
    AboutPortfolio,
    EasterEggs,
    // Edit menu (mostly disabled)
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    // View menu
    ViewAsIcons,
    ViewAsList,
    ShowSidebar,
    HideDock,
}

/// A single dropdown menu item.
#[component]
pub fn DropdownItem(
    #[prop(into)] label: String,
    #[prop(into, optional)] shortcut: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let class = if disabled {
        "dropdown-item disabled"
    } else {
        "dropdown-item"
    };

    let handle_click = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
        if !disabled {
            if let Some(ref cb) = on_click {
                cb.run(());
            }
        }
    };

    view! {
        <div class=class on:click=handle_click>
            {label.clone()}
            {shortcut.clone().map(|s| view! {
                <span class="dropdown-shortcut">{s}</span>
            })}
        </div>
    }
}

/// A divider line in a dropdown menu.
#[component]
pub fn DropdownDivider() -> impl IntoView {
    view! {
        <div class="dropdown-divider"></div>
    }
}

/// A dropdown menu container that shows on hover.
#[component]
pub fn Dropdown(
    children: Children,
) -> impl IntoView {
    view! {
        <div class="menu-dropdown">
            {children()}
        </div>
    }
}
