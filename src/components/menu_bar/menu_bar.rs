//! Main menu bar component.
//!
//! This is the macOS-style menu bar that appears at the top of the screen.
//! It includes the Apple logo, app name, menus, status icons, and clock.

use leptos::prelude::*;
use crate::state::{use_app_state, use_notification_state, WindowId};
use super::clock::Clock;
use super::status_icons::StatusIcons;
use super::dropdown::{Dropdown, DropdownItem, DropdownDivider, MenuAction};

/// Handle menu actions by updating app state.
fn handle_menu_action(action: MenuAction, app_state: &crate::state::AppState) {
    match action {
        MenuAction::NewWindow => {
            app_state.open_window(WindowId::Finder);
        }
        MenuAction::CloseWindow => {
            // Find focused window and close it
            let windows = app_state.windows.get();
            for (id, state) in windows.iter() {
                if state.is_focused && state.is_open {
                    app_state.close_window(*id);
                    break;
                }
            }
        }
        MenuAction::GoWork => {
            app_state.open_window(WindowId::Finder);
            // Navigation will be handled by Finder component
        }
        MenuAction::GoAbout => {
            app_state.open_window(WindowId::Finder);
        }
        MenuAction::GoResume => {
            app_state.open_window(WindowId::Resume);
        }
        MenuAction::GoTrash => {
            app_state.open_window(WindowId::Finder);
        }
        MenuAction::Minimize => {
            let windows = app_state.windows.get();
            for (id, state) in windows.iter() {
                if state.is_focused && state.is_open {
                    app_state.minimize_window(*id);
                    break;
                }
            }
        }
        MenuAction::Zoom => {
            let windows = app_state.windows.get();
            for (id, state) in windows.iter() {
                if state.is_focused && state.is_open {
                    app_state.toggle_maximize(*id);
                    break;
                }
            }
        }
        MenuAction::AboutPortfolio => {
            app_state.open_window(WindowId::About);
        }
        MenuAction::KeyboardShortcuts => {
            let notifications = use_notification_state();
            notifications.show(
                "/public/images/finder.png",
                "Help",
                "Keyboard Shortcuts",
                "⌘+Space: Spotlight\n⌘+N: New Window\n⌘+W: Close Window\n⌘+M: Minimize"
            );
        }
        MenuAction::EasterEggs => {
            // Easter eggs removed
        }
        _ => {
            // Other actions are not implemented or disabled
        }
    }
}

/// File menu with window and folder actions.
#[component]
fn FileMenu() -> impl IntoView {
    let app_state = use_app_state();

    let new_window = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::NewWindow, &app_state))
    };

    let close_window = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::CloseWindow, &app_state))
    };

    view! {
        <span class="menu-item">
            "File"
            <Dropdown>
                <DropdownItem
                    label="New Window"
                    shortcut="⌘N"
                    on_click=new_window
                />
                <DropdownItem
                    label="New Folder"
                    shortcut="⇧⌘N"
                    disabled=true
                />
                <DropdownDivider />
                <DropdownItem
                    label="Get Info"
                    shortcut="⌘I"
                    disabled=true
                />
                <DropdownDivider />
                <DropdownItem
                    label="Close Window"
                    shortcut="⌘W"
                    on_click=close_window
                />
            </Dropdown>
        </span>
    }
}

/// Edit menu (mostly disabled for this portfolio).
#[component]
fn EditMenu() -> impl IntoView {
    view! {
        <span class="menu-item">
            "Edit"
            <Dropdown>
                <DropdownItem label="Undo" shortcut="⌘Z" disabled=true />
                <DropdownItem label="Redo" shortcut="⇧⌘Z" disabled=true />
                <DropdownDivider />
                <DropdownItem label="Cut" shortcut="⌘X" disabled=true />
                <DropdownItem label="Copy" shortcut="⌘C" disabled=true />
                <DropdownItem label="Paste" shortcut="⌘V" disabled=true />
                <DropdownItem label="Select All" shortcut="⌘A" disabled=true />
            </Dropdown>
        </span>
    }
}

/// View menu for display options.
#[component]
fn ViewMenu() -> impl IntoView {
    view! {
        <span class="menu-item">
            "View"
            <Dropdown>
                <DropdownItem label="as Icons" disabled=true />
                <DropdownItem label="as List" disabled=true />
                <DropdownDivider />
                <DropdownItem label="Show Sidebar" disabled=true />
                <DropdownItem label="Hide Dock" disabled=true />
            </Dropdown>
        </span>
    }
}

/// Go menu for navigation.
#[component]
fn GoMenu() -> impl IntoView {
    let app_state = use_app_state();

    let go_work = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::GoWork, &app_state))
    };

    let go_about = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::GoAbout, &app_state))
    };

    let go_resume = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::GoResume, &app_state))
    };

    let go_trash = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::GoTrash, &app_state))
    };

    view! {
        <span class="menu-item">
            "Go"
            <Dropdown>
                <DropdownItem
                    label="Work"
                    shortcut="⇧⌘W"
                    on_click=go_work
                />
                <DropdownItem
                    label="About Me"
                    shortcut="⇧⌘A"
                    on_click=go_about
                />
                <DropdownItem
                    label="Resume"
                    shortcut="⇧⌘R"
                    on_click=go_resume
                />
                <DropdownDivider />
                <DropdownItem
                    label="Trash"
                    on_click=go_trash
                />
            </Dropdown>
        </span>
    }
}

/// Window menu for window management.
#[component]
fn WindowMenu() -> impl IntoView {
    let app_state = use_app_state();

    let minimize = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::Minimize, &app_state))
    };

    let zoom = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::Zoom, &app_state))
    };

    view! {
        <span class="menu-item">
            "Window"
            <Dropdown>
                <DropdownItem
                    label="Minimize"
                    shortcut="⌘M"
                    on_click=minimize
                />
                <DropdownItem
                    label="Zoom"
                    on_click=zoom
                />
                <DropdownDivider />
                <DropdownItem label="Bring All to Front" disabled=true />
            </Dropdown>
        </span>
    }
}

/// Help menu with info and easter eggs.
#[component]
fn HelpMenu() -> impl IntoView {
    let app_state = use_app_state();

    let about_portfolio = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::AboutPortfolio, &app_state))
    };

    let keyboard_shortcuts = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::KeyboardShortcuts, &app_state))
    };

    let easter_eggs = {
        let app_state = app_state.clone();
        Callback::new(move |_| handle_menu_action(MenuAction::EasterEggs, &app_state))
    };

    view! {
        <span class="menu-item">
            "Help"
            <Dropdown>
                <DropdownItem label="Search" disabled=true />
                <DropdownDivider />
                <DropdownItem
                    label="Keyboard Shortcuts"
                    shortcut="⌘/"
                    on_click=keyboard_shortcuts
                />
                <DropdownItem
                    label="About This Portfolio"
                    on_click=about_portfolio
                />
                <DropdownDivider />
                <DropdownItem
                    label="Find Easter Eggs..."
                    on_click=easter_eggs
                />
            </Dropdown>
        </span>
    }
}

/// Spotlight search trigger icon.
#[component]
fn SpotlightTrigger() -> impl IntoView {
    let app_state = use_app_state();

    let toggle_spotlight = move |_| {
        app_state.toggle_spotlight();
    };

    view! {
        <img
            src="/public/icons/search.svg"
            alt="Search"
            class="menu-icon"
            on:click=toggle_spotlight
            style="cursor: pointer;"
        />
    }
}

/// The main menu bar component.
#[component]
pub fn MenuBar() -> impl IntoView {
    let app_state = use_app_state();
    let active_app = move || app_state.active_app.get();

    let open_about = move |_| {
        app_state.open_window(WindowId::About);
    };

    view! {
        <div id="menu-bar">
            <div class="menu-left">
                // Apple logo - opens About window
                <img
                    src="/public/images/logo.svg"
                    alt="Apple"
                    class="apple-logo"
                    on:click=open_about
                    tabindex="0"
                />
                // Active app name
                <span class="menu-item active-app">
                    {active_app}
                </span>
                // Dropdown menus
                <FileMenu />
                <EditMenu />
                <ViewMenu />
                <GoMenu />
                <WindowMenu />
                <HelpMenu />
            </div>
            <div class="menu-right">
                <StatusIcons />
                <SpotlightTrigger />
                <Clock />
            </div>
        </div>
    }
}
