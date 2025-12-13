//! Navigation state for Finder window history.

use leptos::prelude::*;
use crate::data::{Location, FileItem};

/// A navigation entry representing a location in the file system.
#[derive(Clone, Debug, PartialEq)]
pub struct NavEntry {
    /// The current location type (work, about, resume, trash).
    pub location_type: &'static str,
    /// The path of folder IDs from the location root.
    pub path: Vec<u32>,
}

impl NavEntry {
    /// Create a new navigation entry at a location root.
    pub fn new(location_type: &'static str) -> Self {
        Self {
            location_type,
            path: Vec::new(),
        }
    }

    /// Create a navigation entry with a path.
    pub fn with_path(location_type: &'static str, path: Vec<u32>) -> Self {
        Self {
            location_type,
            path,
        }
    }
}

/// Navigation state for the Finder window.
#[derive(Clone)]
pub struct NavigationState {
    /// History of navigation entries.
    history: RwSignal<Vec<NavEntry>>,
    /// Current index in the history.
    current_index: RwSignal<usize>,
}

impl NavigationState {
    /// Create a new navigation state starting at "work".
    pub fn new() -> Self {
        let initial = NavEntry::new("work");
        Self {
            history: RwSignal::new(vec![initial]),
            current_index: RwSignal::new(0),
        }
    }

    /// Get the current navigation entry.
    pub fn current(&self) -> NavEntry {
        let history = self.history.get();
        let index = self.current_index.get();
        history.get(index).cloned().unwrap_or_else(|| NavEntry::new("work"))
    }

    /// Navigate to a new entry, clearing forward history.
    pub fn navigate(&self, entry: NavEntry) {
        let current_idx = self.current_index.get();
        self.history.update(|history| {
            // Remove any forward history
            history.truncate(current_idx + 1);
            // Add new entry
            history.push(entry);
        });
        self.current_index.update(|idx| *idx += 1);
    }

    /// Navigate to a location root.
    pub fn navigate_to_location(&self, location_type: &'static str) {
        self.navigate(NavEntry::new(location_type));
    }

    /// Navigate into a folder.
    pub fn navigate_into_folder(&self, folder_id: u32) {
        let current = self.current();
        let mut new_path = current.path.clone();
        new_path.push(folder_id);
        self.navigate(NavEntry::with_path(current.location_type, new_path));
    }

    /// Go back in history.
    pub fn go_back(&self) -> bool {
        let current_idx = self.current_index.get();
        if current_idx > 0 {
            self.current_index.set(current_idx - 1);
            true
        } else {
            false
        }
    }

    /// Go forward in history.
    pub fn go_forward(&self) -> bool {
        let current_idx = self.current_index.get();
        let history_len = self.history.get().len();
        if current_idx < history_len - 1 {
            self.current_index.set(current_idx + 1);
            true
        } else {
            false
        }
    }

    /// Check if we can go back.
    pub fn can_go_back(&self) -> bool {
        self.current_index.get() > 0
    }

    /// Check if we can go forward.
    pub fn can_go_forward(&self) -> bool {
        let current_idx = self.current_index.get();
        let history_len = self.history.get().len();
        current_idx < history_len - 1
    }

    /// Get the breadcrumb path as strings.
    pub fn get_breadcrumbs(&self, location: &'static Location) -> Vec<(&'static str, Option<u32>)> {
        let current = self.current();
        let mut crumbs = vec![(location.name, None)];
        
        let mut current_children = location.children;
        for &folder_id in &current.path {
            if let Some(folder) = current_children.iter().find(|f| f.id == folder_id) {
                crumbs.push((folder.name, Some(folder_id)));
                if let Some(children) = folder.children {
                    current_children = children;
                }
            }
        }
        
        crumbs
    }

    /// Get the current folder's children.
    pub fn get_current_items(&self, location: &'static Location) -> &'static [FileItem] {
        let current = self.current();
        let mut current_children = location.children;
        
        for &folder_id in &current.path {
            if let Some(folder) = current_children.iter().find(|f| f.id == folder_id) {
                if let Some(children) = folder.children {
                    current_children = children;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        current_children
    }
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide NavigationState to the component tree.
pub fn provide_navigation_state() {
    let state = NavigationState::new();
    provide_context(state);
}

/// Get NavigationState from context.
pub fn use_navigation_state() -> NavigationState {
    expect_context::<NavigationState>()
}
