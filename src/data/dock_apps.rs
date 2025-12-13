//! Dock application configuration data.

/// Represents an application in the dock.
#[derive(Clone, Debug, PartialEq)]
pub struct DockApp {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub can_open: bool,
}

/// All dock applications.
pub const DOCK_APPS: &[DockApp] = &[
    DockApp {
        id: "finder",
        name: "Portfolio",
        icon: "finder.png",
        can_open: true,
    },
    DockApp {
        id: "safari",
        name: "Safari",
        icon: "safari.png",
        can_open: true,
    },
    DockApp {
        id: "photos",
        name: "Gallery",
        icon: "photos.png",
        can_open: true,
    },
    DockApp {
        id: "contact",
        name: "Contact",
        icon: "contact.png",
        can_open: true,
    },
    DockApp {
        id: "terminal",
        name: "Skills",
        icon: "terminal.png",
        can_open: true,
    },
    DockApp {
        id: "trash",
        name: "Archive",
        icon: "trash.png",
        can_open: false,
    },
];
