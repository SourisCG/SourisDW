# Contributing to SourisDW

Thank you for your interest in contributing to SourisDW!

## How to Contribute

### Reporting Bugs

1. Check existing [issues](https://github.com/SourisCG/SourisDW/issues) first
2. Open a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - OS and architecture
   - SourisDW version (`souris-dw --version`)

### Suggesting Features

1. Open an issue with the `enhancement` label
2. Describe the feature and use case
3. Wait for discussion before implementing

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run checks: `cargo fmt && cargo clippy && cargo test`
5. Commit with clear messages
6. Push and open a PR

## Development Setup

```bash
git clone https://github.com/SourisCG/SourisDW.git
cd SourisDW
cargo build
cargo test
```

## Code Standards

- **Formatting:** `cargo fmt` (enforced in CI)
- **Linting:** `cargo clippy -- -D warnings` (no warnings allowed)
- **Tests:** All new features must include tests
- **Cross-platform:** Code must work on Linux, macOS, and Windows

## SDK Contributions

Each SDK lives in `sdks/<language>/`. When contributing:

1. Follow the fluent API pattern (builder + chainable methods)
2. Maintain consistency with other SDKs
3. Include a README with usage examples
4. Add tests if the language has good testing support

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
