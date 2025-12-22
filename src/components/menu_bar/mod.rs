//! Menu bar components for the macOS-style menu bar.
//!
//! This module contains all components needed to render the top menu bar
//! including the clock, status icons, dropdown menus, and the main menu bar.

pub mod clock;
pub mod status_icons;
pub mod dropdown;
#[allow(clippy::module_inception)]
pub mod menu_bar;

pub use clock::Clock;
pub use status_icons::{StatusIcons, WifiIcon, BatteryIcon};
pub use dropdown::{Dropdown, DropdownItem, DropdownDivider};
pub use menu_bar::MenuBar;
