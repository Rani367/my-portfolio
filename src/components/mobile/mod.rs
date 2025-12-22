//! Mobile-specific components for iOS-style interface.
//!
//! This module contains components that render on mobile devices,
//! providing an iOS-like experience with an app grid and full-screen app views.

pub mod home_screen;
pub mod app_container;
pub mod status_bar;

pub use home_screen::MobileHomeScreen;
pub use app_container::MobileAppContainer;
pub use status_bar::MobileStatusBar;
