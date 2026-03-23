# Getting Started

## Prerequisites

- **Rust** toolchain with the `aarch64-apple-ios` target:
  ```bash
  rustup target add aarch64-apple-ios
  ```
- **Theos** build system installed and configured (`$THEOS` set)
- A jailbroken iOS device (arm64) or non jailbreak with https://sideloadly.io/ or https://github.com/CLARATION/Impactor
- (Optional) [sccache](https://github.com/mozilla/sccache) for faster rebuilds

## Project Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/Batchhh/Alloy-ios.git
   cd Alloy-ios
   ```

2. Point the tweak at the process(es) that should load it by editing **`alloy.plist`** in the repo root. This file is the Theos [filter plist](https://theos.dev/docs/tweak-plists): it lists bundle identifiers whose processes get `alloy.dylib` injected.

   The default only targets SpringBoard:

   ```text
   { Filter = { Bundles = ( "com.apple.springboard" ); }; }
   ```

   For a specific app, replace that bundle ID with yours (e.g. `com.example.mygame`). You can list several:

   ```text
   { Filter = { Bundles = ( "com.example.a", "com.example.b" ); }; }
   ```

   Change this whenever you switch target apps so injection matches the binary you hook in code.

3. Configure your target in `src/config.rs`:
   ```rust
   // Override the target binary (optional — defaults to UnityFramework or main executable)
   config::set_target_image_name("YourBinary");

   // Set the menu theme
   pub const SELECTED_THEME: ThemeVariant = ThemeVariant::Nord;
   ```

4. Write your logic in `src/lib.rs`. This is the main entry point where you register UI elements and set up hooks/patches.

## Building

### Release build (default)

```bash
make deploy
```

This runs `cargo fmt`, builds the Rust static library, links via Theos, and produces a `.deb` package.

### Debug build (with logging)

```bash
make deploy RUST_PROFILE=dev-release
```

The `dev-release` profile enables logging through Apple's Unified Logging System while keeping other release optimizations disabled for faster iteration.

### Individual targets

```bash
make fmt          # Format code
make clippy       # Run linter
make rust-build   # Build Rust library only
make clean        # Clean all build artifacts
```

## Deploying to Device

The `make deploy` command produces a `.deb` package in the `packages/` directory. Install it on your device using your preferred method (e.g., Filza, `dpkg -i`, or Theos's built-in install target).

## Viewing Logs

Logs are only emitted in the `dev-release` profile. View them with **Console.app** on macOS:

- Filter by process or subsystem: `com.ios.alloy`
- Or search for: `Alloy`

Available log levels: `info`, `debug`, `warning`, `error`.

```rust
use crate::utils::logger;

logger::info("Something happened");
logger::debug("Debug detail");
logger::warning("Heads up");
logger::error("Something went wrong");
```
