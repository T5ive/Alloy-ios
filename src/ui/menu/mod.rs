//! # Menu System
//!
//! This module handles the core menu logic, including:
//! - Item registry and page management
//! - Interaction handling (taps, gestures)
//! - Menu rendering and UI updates
pub mod handler;
pub mod items;
#[allow(dead_code)]
pub mod registry;
#[allow(dead_code)]
pub mod utils;
pub mod view;

pub use registry::*;
pub use view::*;
