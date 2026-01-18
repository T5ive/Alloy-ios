# Rust iOS Tweak

A high-performance iOS modding framework built entirely from scratch in Rust—no Substrate, no Dobby, no external libraries.

## Features

- **ARM64 Inline Hooking**: Custom trampoline generation and instruction relocation (Jailbreak only).
- **Hardware Breakpoints**: Non-jailbreak hooking via debug registers (Both).
- **W^X Memory Patching**: Safe writes with thread management and cache invalidation (Jailbreak only).
- **Memory Utilities**: Type-safe RVA conversion and pointer chain traversal.
- **Memory Scanning**: IDA-style signature finding with wildcards.
- **Zero Dependencies**: Pure Rust using direct `mach2` syscalls.
- **In-Game UI**: Responsive menu built with `objc2`.


## Configuration

You can customize the tweak behavior in `src/config.rs`:

```rust
pub const TARGET_IMAGE_NAME: &str = "UnityFramework";             // Binary to hook
pub const DEBUG: bool = true;                                     // Toggle detailed logging
pub const SELECTED_THEME: ThemeVariant = ThemeVariant::Default;   // Theme variant
```

## Building & Deploying

1. **Prerequisites**:
   ```bash
   rustup target add aarch64-apple-ios
   brew install sshpass
   https://theos.dev/docs/installation-ios
   cargo install sccache # for better perfomance
   ```

2. **Deploy to Device**:
   ```bash
   # Edit Makefile to set your DEVICE_IP
   make deploy
   ```

## Viewing Logs

Logs are sent to the Apple Unified Logging System. You can view them using **Console.app** on macOS:
- filter for: `RGG` or `subsystem:com.rust_tweak`.

## Roadmap

Planned features and improvements for future releases:

- [x] **In-Game UI Menu**: SwiftUI or Metal-based overlay for runtime mod control *(foundation implemented)*
- [x] **Memory Scanning**: Pattern scanning and signature-based function finding
- [x] **Breakpoint Hooks**: Hardware breakpoint support for non-jailbroken devices
- [x] **Symbol Resolution**: Automatic symbol lookup and caching
- [ ] **Hot Reloading**: Dynamic mod loading without reinjection
- [ ] **Il2cpp Resolver**: Automatic il2cpp class and method resolution

## Contributing

Contributions are welcome! If you find any issues or have suggestions, please [open an issue](https://github.com/Batchhh/Rust-ios-tweak/issues).

For collaboration, feel free to submit [pull requests](https://github.com/Batchhh/Rust-ios-tweak/pulls).

Follow Rust conventions (`cargo fmt`, `cargo clippy`) and document your changes.

### Commit Guidelines

Use clear, descriptive commit messages:
Format: `type: brief description`
Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

## Legal

Educational purposes only. Modifying games may violate Terms of Service. Use at your own risk.

## License

[MIT License](https://github.com/Batchhh/Rust-ios-tweak/blob/main/LICENSE) - See LICENSE file for details.

## Credits

- [Batch](https://github.com/Batchhh) - Creator
- [Titanox](https://github.com/Ragekill3377/Titanox) - Inspiration for breakpoint hooks