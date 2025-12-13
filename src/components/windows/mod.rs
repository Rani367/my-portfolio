//! Window components for the portfolio application.
//!
//! This module contains the base window component and window controls
//! that are used by all window types in the application.

pub mod window_controls;
pub mod window;
pub mod finder;
pub mod txtfile;
pub mod imgfile;
pub mod terminal;
pub mod contact;
pub mod photos;
pub mod about;
pub mod safari;
pub mod resume;

pub use window_controls::*;
pub use window::*;
pub use finder::FinderContent;
pub use txtfile::TxtFileContent;
pub use imgfile::ImgFileContent;
pub use terminal::TerminalContent;
pub use contact::ContactContent;
pub use photos::PhotosContent;
pub use about::AboutContent;
pub use safari::SafariContent;
pub use resume::ResumeContent;
