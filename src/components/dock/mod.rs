//! Dock component with macOS-style magnification effect.

#[allow(clippy::module_inception)]
mod dock;
mod dock_item;

pub use dock::Dock;
pub use dock_item::DockItem;
