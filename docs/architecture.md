# Architecture

## Overview

Alloy is a Rust-based iOS tweak framework that compiles to a static library (`liballoy.a`), which Theos links into a dynamic library (`.dylib`) and packages as a `.deb` for deployment. The library is loaded at process startup via the `#[ctor::ctor]` attribute.

## Lifecycle

```
Process launch
  → dyld loads alloy.dylib
    → #[ctor::ctor] fn init()         (src/lib.rs)
      → logger::info("initializing")
      → entry::init()                  (src/entry.rs)
        → Queue::main().exec_async()
          → init_ui()                  register menu items & init overlay
```

All UI work is dispatched to the main queue since UIKit requires main-thread access.

## Module Structure

```
src/
├── lib.rs          # Crate root, #[ctor] entry point
├── config.rs       # Target binary config, theme, menu metadata
├── entry.rs        # User-facing init (UI setup, hooks, patches)
├── ui/             # Native iOS UI system (see docs/ui.md)
└── utils/
    ├── logger.rs   # Apple Unified Logging wrapper
    └── macros.rs   # Utility macros
```

### External: specter-mem

Memory operations (hooking, patching, scanning, shellcode) are provided by the [`specter-mem`](https://crates.io/crates/specter-mem) crate. This was previously vendored under `src/memory/` but is now an external dependency.

## Build Pipeline

```
cargo build --target aarch64-apple-ios
  → produces target/aarch64-apple-ios/{profile}/liballoy.a

Theos links liballoy.a into alloy.dylib
  → -force_load ensures all Rust symbols are included
  → Frameworks: UIKit, Foundation, CoreGraphics, QuartzCore

make package
  → produces packages/com.ios.alloy_{version}_iphoneos-arm64.deb
```

## Build Profiles

| Profile | Logging | Optimizations | Use |
|---------|---------|---------------|-----|
| `release` | Disabled | `opt-level=z`, LTO, stripped | Production |
| `dev-release` | Enabled (`#[cfg(dev_release)]`) | None, incremental | Development |

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `ctor` | `#[ctor]` constructor for library init |
| `specter-mem` | Memory hooking, patching, scanning, shellcode |
| `jit-assembler` | ARM64 assembly generation for patches |
| `objc2` / `objc2-*` | Rust bindings to Objective-C and UIKit |
| `block2` | Objective-C block support |
| `dispatch` | GCD (Grand Central Dispatch) bindings |
| `oslog` | Apple Unified Logging |
| `glam` | Math types (vectors, matrices) |
| `parking_lot` | Fast synchronization primitives |
| `once_cell` | Lazy/one-time initialization |
| `zip` | Archive handling |
