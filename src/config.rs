//! Configuration constants for target binary

use crate::ui::theme::ThemeVariant;
use once_cell::sync::OnceCell;

extern "C" {
    fn _dyld_image_count() -> u32;
    fn _dyld_get_image_name(image_index: u32) -> *const std::ffi::c_char;
}

#[allow(dead_code)]
static TARGET_IMAGE_OVERRIDE: OnceCell<String> = OnceCell::new();

/// Override the target image name. Must be called before the first `target_image_name()` call.
#[allow(dead_code)]
pub fn set_target_image_name(name: &str) {
    let _ = TARGET_IMAGE_OVERRIDE.set(name.to_string());
}

/// Returns the target image name. Uses the override if set, otherwise prefers
/// "UnityFramework" if loaded, falling back to the main executable.
#[allow(dead_code)]
pub fn target_image_name() -> &'static str {
    if let Some(name) = TARGET_IMAGE_OVERRIDE.get() {
        return name.as_str();
    }

    unsafe {
        let count = _dyld_image_count();
        for i in 0..count {
            let name_ptr = _dyld_get_image_name(i);
            if name_ptr.is_null() {
                continue;
            }
            let name = std::ffi::CStr::from_ptr(name_ptr).to_str().unwrap_or("");
            if name.contains("UnityFramework") {
                return "UnityFramework";
            }
        }
        // Fallback: main executable
        let main_ptr = _dyld_get_image_name(0);
        if !main_ptr.is_null() {
            let full = std::ffi::CStr::from_ptr(main_ptr).to_str().unwrap_or("");
            if let Some(base) = full.rsplit('/').next() {
                return Box::leak(base.to_string().into_boxed_str());
            }
        }
        "unknown"
    }
}

pub const SELECTED_THEME: ThemeVariant = ThemeVariant::Nord;
