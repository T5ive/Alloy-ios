# Alloy ios tweak

High-performance iOS modding framework built in Rust.

## Features
- **Hooking**: ARM64 Inline & Hardware Breakpoints.
- **Memory**: Safe patching, scanning, and symbol resolution.
- **UI**: Native iOS Menu (objc2) & content-rich Mod Menu.
- **Stealth**: Stealthy hooking and patching. (ShellCode Injection + Code Caves)

## Usage

1. **Configure**: Edit `src/config.rs` to set target binary and options.
2. **Deploy**:
   ```bash
   make deploy
   ```

## Documentation

- [Getting started](docs/getting-started.md) — prerequisites, `alloy.plist`, `config.rs`, build, deploy, logs
- [Architecture](docs/architecture.md)
- [Memory](docs/memory.md)
- [UI](docs/ui.md)

The codebase is also commented; read sources alongside these guides when diving deeper.

## Viewing Logs

Logs are sent to the Apple Unified Logging System. You can view them using **Console.app** on macOS:
- filter for: `Alloy` or `subsystem:com.ios.alloy`.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

If you find any issues or have suggestions, please [open an issue](https://github.com/Batchhh/Alloy-ios/issues).

## Legal

Educational purposes only. Modifying games may violate Terms of Service. Use at your own risk.

## License

[MIT License](https://github.com/Batchhh/Alloy-ios/blob/main/LICENSE) - See LICENSE file for details.

## Credits

- [Batch](https://github.com/Batchhh) - Creator
- [Titanox](https://github.com/Ragekill3377/Titanox) - Inspiration for breakpoint hooks