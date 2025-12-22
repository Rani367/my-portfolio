//! Global application state management.

use leptos::prelude::*;
use std::collections::HashMap;
use super::window_state::{WindowId, WindowState};
use crate::data::FileItem;

/// Data associated with a file viewer window.
#[derive(Clone, Debug, PartialEq)]
pub struct FileViewerData {
    pub file: &'static FileItem,
}

/// Global application state.
#[derive(Clone)]
pub struct AppState {
    /// State for all windows.
    pub windows: RwSignal<HashMap<WindowId, WindowState>>,
    /// Current highest z-index.
    pub current_z_index: RwSignal<i32>,
    /// Currently active app name (shown in menu bar).
    pub active_app: RwSignal<String>,
    /// Whether spotlight search is open.
    pub spotlight_open: RwSignal<bool>,
    /// Whether Konami code easter egg is active.
    pub konami_active: RwSignal<bool>,
    /// Data for the text file viewer.
    pub txt_file_data: RwSignal<Option<FileViewerData>>,
    /// Data for the image file viewer.
    pub img_file_data: RwSignal<Option<FileViewerData>>,
    /// Whether the app is in mobile mode.
    pub is_mobile: RwSignal<bool>,
    /// Currently active app in mobile mode (single app view).
    pub mobile_active_app: RwSignal<Option<WindowId>>,
    /// Whether a mobile app transition (open/close animation) is in progress.
    pub is_mobile_transitioning: RwSignal<bool>,
}

impl AppState {
    /// Create a new AppState with default values.
    pub fn new() -> Self {
        let mut windows = HashMap::new();
        for id in WindowId::all() {
            windows.insert(*id, WindowState::default());
        }

        Self {
            windows: RwSignal::new(windows),
            current_z_index: RwSignal::new(1000),
            active_app: RwSignal::new("Finder".to_string()),
            spotlight_open: RwSignal::new(false),
            konami_active: RwSignal::new(false),
            txt_file_data: RwSignal::new(None),
            img_file_data: RwSignal::new(None),
            is_mobile: RwSignal::new(false),
            mobile_active_app: RwSignal::new(None),
            is_mobile_transitioning: RwSignal::new(false),
        }
    }

    /// Get the next z-index and increment the counter.
    pub fn next_z_index(&self) -> i32 {
        let current = self.current_z_index.get();
        self.current_z_index.set(current + 1);
        current + 1
    }

    /// Open a window.
    pub fn open_window(&self, id: WindowId) {
        let z = self.next_z_index();
        self.windows.update(|windows| {
            // Unfocus all other windows
            for (_, state) in windows.iter_mut() {
                state.unfocus();
            }
            // Open and focus the target window
            if let Some(state) = windows.get_mut(&id) {
                state.open(z);
            }
        });
        self.active_app.set(self.get_app_name(id));
    }

    /// Close a window.
    pub fn close_window(&self, id: WindowId) {
        self.windows.update(|windows| {
            if let Some(state) = windows.get_mut(&id) {
                state.close();
            }
        });
    }

    /// Complete window close after animation.
    pub fn close_window_complete(&self, id: WindowId) {
        self.windows.update(|windows| {
            if let Some(state) = windows.get_mut(&id) {
                state.close_complete();
            }
        });
    }

    /// Minimize a window.
    pub fn minimize_window(&self, id: WindowId) {
        self.windows.update(|windows| {
            if let Some(state) = windows.get_mut(&id) {
                state.minimize();
            }
        });
    }

    /// Complete window minimize after animation.
    pub fn minimize_window_complete(&self, id: WindowId) {
        self.windows.update(|windows| {
            if let Some(state) = windows.get_mut(&id) {
                state.minimize_complete();
            }
        });
    }

    /// Focus a window (also restores from minimized state).
    pub fn focus_window(&self, id: WindowId) {
        let z = self.next_z_index();
        self.windows.update(|windows| {
            // Unfocus all other windows
            for (_, state) in windows.iter_mut() {
                state.unfocus();
            }
            // Focus the target window and restore if minimized
            if let Some(state) = windows.get_mut(&id) {
                state.is_minimized = false;
                state.is_minimizing = false;
                state.focus(z);
            }
        });
        self.active_app.set(self.get_app_name(id));
    }

    /// Toggle window maximize state.
    pub fn toggle_maximize(&self, id: WindowId) {
        self.windows.update(|windows| {
            if let Some(state) = windows.get_mut(&id) {
                state.toggle_maximize();
            }
        });
    }

    /// Check if a window is open.
    pub fn is_window_open(&self, id: WindowId) -> bool {
        self.windows.get().get(&id).map(|s| s.is_open).unwrap_or(false)
    }

    /// Open a text file viewer with data.
    pub fn open_txt_file(&self, file: &'static FileItem) {
        self.txt_file_data.set(Some(FileViewerData { file }));
        self.open_window(WindowId::TxtFile);
    }

    /// Open an image file viewer with data.
    pub fn open_img_file(&self, file: &'static FileItem) {
        self.img_file_data.set(Some(FileViewerData { file }));
        self.open_window(WindowId::ImgFile);
    }

    /// Toggle spotlight search.
    pub fn toggle_spotlight(&self) {
        self.spotlight_open.update(|open| *open = !*open);
    }

    /// Close spotlight search.
    pub fn close_spotlight(&self) {
        self.spotlight_open.set(false);
    }

    /// Toggle Konami code easter egg.
    pub fn toggle_konami(&self) {
        self.konami_active.update(|active| *active = !*active);
    }

    /// Open an app in mobile mode (single full-screen view).
    /// Returns false if a transition is already in progress.
    pub fn mobile_open_app(&self, id: WindowId) -> bool {
        // Prevent rapid double-taps during animation
        if self.is_mobile_transitioning.get_untracked() {
            return false;
        }
        self.is_mobile_transitioning.set(true);
        self.mobile_active_app.set(Some(id));
        self.open_window(id);
        true
    }

    /// Close the current mobile app (return to home screen).
    /// Returns false if a transition is already in progress.
    pub fn mobile_close_app(&self) -> bool {
        // Prevent rapid double-taps during animation
        if self.is_mobile_transitioning.get_untracked() {
            return false;
        }
        self.is_mobile_transitioning.set(true);
        if let Some(id) = self.mobile_active_app.get_untracked() {
            self.close_window(id);
        }
        self.mobile_active_app.set(None);
        true
    }

    /// Mark mobile transition as complete (call after animation ends).
    pub fn mobile_transition_complete(&self) {
        self.is_mobile_transitioning.set(false);
    }

    /// Get app name for menu bar display.
    fn get_app_name(&self, id: WindowId) -> String {
        match id {
            WindowId::Finder => "Finder",
            WindowId::Contact => "Contact",
            WindowId::Resume => "Preview",
            WindowId::Safari => "Safari",
            WindowId::Photos => "Photos",
            WindowId::Terminal => "Terminal",
            WindowId::TxtFile => "TextEdit",
            WindowId::ImgFile => "Preview",
            WindowId::About => "Finder",
        }.to_string()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide AppState to the component tree.
pub fn provide_app_state() {
    let state = AppState::new();
    provide_context(state);
}

/// Get AppState from context.
pub fn use_app_state() -> AppState {
    expect_context::<AppState>()
}
