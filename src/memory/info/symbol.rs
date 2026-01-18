//! Symbol resolution and caching utilities

use crate::utils::logger;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::{collections::HashMap, ffi::CString};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SymbolError {
    #[error("Symbol not found: {0}")]
    NotFound(String),
    #[error("CString error")]
    StringError,
}

static CACHE: Lazy<RwLock<HashMap<String, usize>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn resolve_symbol(symbol: &str) -> Result<usize, SymbolError> {
    if let Some(&addr) = CACHE.read().get(symbol) {
        return Ok(addr);
    }
    let c_str = CString::new(symbol).map_err(|_| SymbolError::StringError)?;
    unsafe {
        let addr_ptr = libc::dlsym(libc::RTLD_DEFAULT, c_str.as_ptr());
        if addr_ptr.is_null() {
            return Err(SymbolError::NotFound(symbol.into()));
        }
        let addr = addr_ptr as usize;
        CACHE.write().insert(symbol.into(), addr);
        logger::info(&format!("Resolved {} to {:#x}", symbol, addr));
        Ok(addr)
    }
}

pub fn cache_symbol(s: &str, a: usize) {
    CACHE.write().insert(s.into(), a);
}
pub fn clear_cache() {
    CACHE.write().clear();
}
