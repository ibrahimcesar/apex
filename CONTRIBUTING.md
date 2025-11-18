# Contributing to apex 🎯

Thank you for your interest in contributing to apex!

## 🚧 Project Status

This project is in **early development**. We're building the foundation for cross-compilation automation.

## 🤝 How to Contribute

### Reporting Issues

- **Bugs**: Describe the issue, steps to reproduce, expected vs actual behavior
- **Feature Requests**: Explain the use case and proposed solution
- **Target Support**: Request new platforms with toolchain details

### Code Contributions

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Make your changes
4. Add tests
5. Run `cargo test` and `cargo clippy`
6. Commit (`git commit -m 'Add amazing feature'`)
7. Push (`git push origin feature/amazing`)
8. Open a Pull Request

## 📋 Development Setup
```bash
# Clone
git clone https://github.com/yourusername/apex
cd apex

# Build
cargo build

# Run
cargo run -- --help

# Test
cargo test

# Lint
cargo clippy
```

## 🎯 Areas Needing Help

- [ ] Target detection logic
- [ ] Toolchain installation automation
- [ ] Container runtime integration
- [ ] Native dependency management
- [ ] Documentation and examples
- [ ] Testing on different platforms

## 📝 Code Style

- Follow Rust conventions (`cargo fmt`)
- Pass `cargo clippy` without warnings
- Add docs for public APIs
- Include tests for new features

## 🧪 Testing

Test on multiple platforms when possible:
- Linux (x86_64, ARM)
- Windows (native, WSL)
- macOS (Intel, Apple Silicon)

## 📜 License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
