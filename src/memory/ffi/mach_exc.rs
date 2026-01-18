//! Mach Exception Port FFI Bindings
//!
//! This module provides FFI bindings for Mach exception handling functions
//! that are not available in existing crates like mach2 or libc.

use mach2::port::mach_port_t;
use std::ffi::c_void;

pub type KernReturnT = i32;
pub type ExceptionMaskT = u32;
pub type ExceptionBehaviorT = i32;
pub type ThreadStateFlavor = i32;

extern "C" {
    /// Sets exception ports for a task
    pub fn task_set_exception_ports(
        task: mach_port_t,
        exception_mask: ExceptionMaskT,
        new_port: mach_port_t,
        behavior: ExceptionBehaviorT,
        new_flavor: ThreadStateFlavor,
    ) -> KernReturnT;

    /// Gets exception ports for a task
    pub fn task_get_exception_ports(
        task: mach_port_t,
        exception_mask: ExceptionMaskT,
        masks: *mut ExceptionMaskT,
        masks_cnt: *mut u32,
        old_handlers: *mut mach_port_t,
        old_behaviors: *mut ExceptionBehaviorT,
        old_flavors: *mut ThreadStateFlavor,
    ) -> KernReturnT;

    /// Sets the state of a task
    pub fn task_set_state(
        task: mach_port_t,
        flavor: ThreadStateFlavor,
        state: *const c_void,
        count: u32,
    ) -> KernReturnT;
}
