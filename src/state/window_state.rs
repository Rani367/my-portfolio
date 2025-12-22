//! Window state management.

/// Represents the state of a single window.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowState {
    pub is_open: bool,
    pub z_index: i32,
    pub is_focused: bool,
    pub is_maximized: bool,
    pub is_minimized: bool,
    pub is_minimizing: bool,
    pub is_closing: bool,
    pub position: (i32, i32),
    pub size: (i32, i32),
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            is_open: false,
            z_index: 1000,
            is_focused: false,
            is_maximized: false,
            is_minimized: false,
            is_minimizing: false,
            is_closing: false,
            position: (100, 100),
            size: (800, 500),
        }
    }
}

impl WindowState {
    /// Create a new window state with custom position and size.
    pub fn new(position: (i32, i32), size: (i32, i32)) -> Self {
        Self {
            position,
            size,
            ..Default::default()
        }
    }

    /// Open the window (or restore from minimized).
    pub fn open(&mut self, z_index: i32) {
        self.is_open = true;
        self.is_focused = true;
        self.is_closing = false;
        self.is_minimizing = false;
        self.is_minimized = false;
        self.z_index = z_index;
    }

    /// Close the window with animation.
    pub fn close(&mut self) {
        self.is_closing = true;
    }

    /// Complete the close after animation.
    pub fn close_complete(&mut self) {
        self.is_open = false;
        self.is_focused = false;
        self.is_closing = false;
    }

    /// Minimize the window with animation.
    pub fn minimize(&mut self) {
        self.is_minimizing = true;
    }

    /// Complete minimize after animation.
    pub fn minimize_complete(&mut self) {
        self.is_minimized = true;
        self.is_focused = false;
        self.is_minimizing = false;
    }

    /// Focus the window.
    pub fn focus(&mut self, z_index: i32) {
        self.is_focused = true;
        self.z_index = z_index;
    }

    /// Unfocus the window.
    pub fn unfocus(&mut self) {
        self.is_focused = false;
    }

    /// Toggle maximize state.
    pub fn toggle_maximize(&mut self) {
        self.is_maximized = !self.is_maximized;
    }

    /// Update position.
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }
}

/// Window identifiers for all application windows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WindowId {
    Finder,
    Contact,
    Resume,
    Safari,
    Photos,
    Terminal,
    TxtFile,
    ImgFile,
    About,
}

impl WindowId {
    /// Get all window IDs.
    pub fn all() -> &'static [WindowId] {
        &[
            WindowId::Finder,
            WindowId::Contact,
            WindowId::Resume,
            WindowId::Safari,
            WindowId::Photos,
            WindowId::Terminal,
            WindowId::TxtFile,
            WindowId::ImgFile,
            WindowId::About,
        ]
    }

    /// Get the string identifier.
    pub fn as_str(&self) -> &'static str {
        match self {
            WindowId::Finder => "finder",
            WindowId::Contact => "contact",
            WindowId::Resume => "resume",
            WindowId::Safari => "safari",
            WindowId::Photos => "photos",
            WindowId::Terminal => "terminal",
            WindowId::TxtFile => "txtfile",
            WindowId::ImgFile => "imgfile",
            WindowId::About => "about",
        }
    }

    /// Get window from dock app ID.
    pub fn from_dock_id(id: &str) -> Option<WindowId> {
        match id {
            "finder" => Some(WindowId::Finder),
            "contact" => Some(WindowId::Contact),
            "safari" => Some(WindowId::Safari),
            "photos" => Some(WindowId::Photos),
            "terminal" => Some(WindowId::Terminal),
            _ => None,
        }
    }
}
