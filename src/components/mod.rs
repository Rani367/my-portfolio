//! UI Components for the portfolio application.
//!
//! This module contains all reusable UI components including windows,
//! dock, menu bar, and other interface elements.

pub mod windows;
pub mod menu_bar;
pub mod dock;
pub mod spotlight;
pub mod startup;
pub mod notifications;

pub use windows::*;
pub use menu_bar::MenuBar;
pub use dock::Dock;
pub use spotlight::Spotlight;
pub use startup::StartupScreen;
pub use notifications::NotificationCenter;
