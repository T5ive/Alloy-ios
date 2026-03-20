# Contributing to Alloy

Thanks for your interest in contributing! Here's how to get started.

## Getting Started

1. Fork the repository at [github.com/Batchhh/Alloy-ios](https://github.com/Batchhh/Alloy-ios)
2. Clone your fork:
   ```bash
   git clone https://github.com/<your-username>/Alloy-ios.git
   ```
3. Create a new branch for your changes:
   ```bash
   git checkout -b feat/your-feature
   ```

## Development

- Follow Rust conventions: run `cargo fmt` and `cargo clippy` before submitting.
- Document your changes where necessary.
- Test your changes on-device when possible.

## Commit Guidelines

- Use clear, descriptive messages.
- Format: `type: brief description`
- Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

Examples:
```
feat: add memory region scanning
fix: resolve crash on arm64e devices
docs: update hooking usage examples
```

## Pull Requests

1. Push your branch to your fork.
2. Open a [pull request](https://github.com/Batchhh/Alloy-ios/pulls) against `main`.
3. Describe what your PR does and why.
4. Keep PRs focused — one feature or fix per PR.

## Reporting Issues

Found a bug or have a suggestion? [Open an issue](https://github.com/Batchhh/Alloy-ios/issues) with:
- A clear description of the problem or idea.
- Steps to reproduce (for bugs).
- Device/iOS version if relevant.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
