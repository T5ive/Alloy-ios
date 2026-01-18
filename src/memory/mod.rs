//! Memory manipulation and introspection utilities

pub mod ffi;
pub mod info;
pub mod manipulation;
pub mod platform;

// Re-export commonly used items for convenience
pub use info::{image, scan, symbol};
pub use manipulation::{hook, patch, rw};
pub use platform::{breakpoint, thread};
