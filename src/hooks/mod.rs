//! Custom hooks for the portfolio application.
//!
//! This module contains reusable hooks for common functionality like
//! keyboard events, battery status, network status, clock, and mobile detection.

pub mod use_keyboard;
pub mod use_battery;
pub mod use_network;
pub mod use_clock;
pub mod use_mobile;
pub mod use_scroll;

pub use use_keyboard::*;
pub use use_battery::*;
pub use use_network::*;
pub use use_clock::*;
pub use use_mobile::*;
pub use use_scroll::*;
