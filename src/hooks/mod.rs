//! Custom hooks for the portfolio application.
//!
//! This module contains reusable hooks for common functionality like
//! window dragging, keyboard events, battery status, network status, clock, and animations.

pub mod use_draggable;
pub mod use_keyboard;
pub mod use_battery;
pub mod use_network;
pub mod use_clock;
pub mod use_animation_frame;

pub use use_draggable::*;
pub use use_keyboard::*;
pub use use_battery::*;
pub use use_network::*;
pub use use_clock::*;
pub use use_animation_frame::*;
