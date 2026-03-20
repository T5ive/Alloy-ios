//! # Native UI Implementation
//!
//! This module implements the native iOS UI overlay for the mod menu.
//! It handles:
//! - Window management and overlay initialization
//! - UI Components (Buttons, Toggles, Sliders)
//! - Menu structure and navigation
//! - Theming and Preferences
//! - Interactions (Touch, Drag, etc.)
pub mod assets;
pub mod components;
pub mod menu;
pub mod pref;
#[allow(dead_code)]
pub mod theme;
#[allow(dead_code)]
pub mod utils;
#[allow(dead_code)]
pub mod window;
pub mod native {
    pub use super::window::init_overlay;
}
pub use menu::{add_section_header, add_tab};
