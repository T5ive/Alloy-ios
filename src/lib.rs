//! Main entry point

mod config;
mod memory;
mod utils;

use std::{ffi::c_void};
use utils::logger;

use dispatch::Queue;

fn _example_patch() -> bool {
    // Example 1: Hex-based patch with revert
    // if let Ok(patch) = memory::patch::apply(0x292d94c, "C0035FD6") {
    //     logger::info("Applied hex patch!");
    //     patch.revert();
    //     logger::info("Hex patch reverted");
    //     return true;
    // }

    // Example 2: ASM-based patch with revert
    if let Ok(patch) = memory::patch::apply_asm(0x292d94c, |b| b.ret()) {
        logger::info("Applied ASM patch!");
        // patch.revert();
        // logger::info("ASM patch reverted");
        return true;
    }

    false
}

static HOOK: std::sync::OnceLock<memory::hook::Hook> = std::sync::OnceLock::new();

fn _example_hook() -> bool {
    type UpdateFn = fn(*mut c_void, f32);

    fn update_hook(this: *mut c_void, delta_time: f32) {
        if let Some(hook) = HOOK.get() {
            let original: UpdateFn = unsafe { hook.trampoline_as() };
            
            // Read and log values at offsets 0xa8 and 0xb0
            unsafe {
                let coins_addr = this as usize + 0xa8;
                
                if let Ok(coins_value) = memory::rw::read::<f64>(coins_addr) {
                    logger::info(&format!("this+0xa8 (double): {}", coins_value));
                    let _ = memory::rw::write::<f64>(coins_addr, 99999.0);
                    logger::info("Modified this+0xa8 to 99999.0");
                }
            }
            
            original(this, delta_time);
        }
    }

    unsafe {
        if let Ok(hook) = memory::hook::install(0x292d94c, update_hook as usize) {
            logger::info("Hook installed!");
            let _ = HOOK.set(hook);
            // To remove later: HOOK.get().unwrap().remove();
            return true;
        }
    }

    false
}

#[ctor::ctor]
fn init() {
    logger::info("rust_igmm initializing...");
    Queue::main().exec_async(|| {
        if _example_hook() {
            return;
        }
        // if _example_patch() {
        //     return;
        // }
    });
}
