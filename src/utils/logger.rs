//! Logging via Apple Unified Logging System

#[cfg(dev_release)]
use log::LevelFilter;
#[cfg(dev_release)]
use oslog::OsLogger;
#[cfg(dev_release)]
use std::sync::Once;

#[cfg(dev_release)]
static INIT: Once = Once::new();

#[cfg(dev_release)]
fn ensure_initialized() {
    INIT.call_once(|| {
        OsLogger::new("com.ios.alloy")
            .level_filter(LevelFilter::Debug)
            .init()
            .ok();
    });
}

/// Logs an informational message
pub fn info(_msg: &str) {
    #[cfg(dev_release)]
    {
        ensure_initialized();
        log::info!("{}", _msg);
    }
}

/// Logs a debug message
pub fn debug(_msg: &str) {
    #[cfg(dev_release)]
    {
        ensure_initialized();
        log::debug!("{}", _msg);
    }
}

/// Logs a warning message
pub fn warning(_msg: &str) {
    #[cfg(dev_release)]
    {
        ensure_initialized();
        log::warn!("{}", _msg);
    }
}

/// Logs an error message
pub fn error(_msg: &str) {
    #[cfg(dev_release)]
    {
        ensure_initialized();
        log::error!("{}", _msg);
    }
}
