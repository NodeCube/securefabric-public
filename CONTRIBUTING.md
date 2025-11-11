# Contributing to SecureFabric Public SDKs

Thank you for your interest in contributing to SecureFabric! This document
provides guidelines for contributing to the public SDK and documentation
repository.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you agree to uphold a respectful and inclusive environment.

## How to Contribute

### Reporting Issues

- **Bugs**: Use the bug report template
- **Feature Requests**: Use the feature request template
- **Security Issues**: See [SECURITY.md](SECURITY.md)

### Submitting Changes

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/my-feature`
3. **Make your changes**
4. **Test your changes** (see Testing section)
5. **Commit with clear messages**: Follow [Conventional Commits](https://www.conventionalcommits.org/)
6. **Push to your fork**: `git push origin feature/my-feature`
7. **Open a Pull Request**

## Development Setup

### Rust SDK

```bash
cd sdk/rust
cargo build
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

### JavaScript/TypeScript SDK

```bash
cd sdk/js
npm install
npm run build
npm test
npm run lint
```

### Python SDK

```bash
cd sdk/python
python -m venv venv
source venv/bin/activate  # or `venv\Scripts\activate` on Windows
pip install -e '.[dev]'
pytest
black --check .
mypy securefabric
```

## Testing Requirements

All contributions must include tests:

- **Unit tests** for new functionality
- **Integration tests** for SDK methods (where applicable)
- **Documentation** updates if adding/changing APIs
- **Examples** updated if changing interfaces

## Code Style

### Rust

- Follow `rustfmt` defaults
- Use `clippy` with `-D warnings`
- Document public APIs with `///` doc comments

### JavaScript/TypeScript

- Use ESLint + Prettier
- TypeScript strict mode enabled
- JSDoc comments for public APIs

### Python

- Follow PEP 8
- Use Black for formatting
- Type hints required for public APIs
- Docstrings in Google style

## Commit Messages

Follow Conventional Commits:

```text
feat: add support for Python async API
fix: correct error handling in Rust SDK
docs: update quickstart guide
test: add integration tests for JS SDK
chore: update dependencies
```

## Pull Request Guidelines

- **Title**: Use conventional commit format
- **Description**: Explain what and why, not how
- **Link Issues**: Reference related issues
- **Tests**: Ensure all tests pass
- **Documentation**: Update docs if needed
- **Changelog**: Update CHANGELOG.md for notable changes

## Review Process

1. Automated CI checks must pass
2. At least one maintainer approval required
3. All conversations must be resolved
4. Squash merging preferred for clean history

## Documentation

- **API docs**: Auto-generated from code comments
- **Guides**: Markdown in `docs/`
- **Examples**: Self-contained in `examples/`

Update documentation when:

- Adding new SDK methods
- Changing existing interfaces
- Adding new examples
- Updating protocol specs

## Protocol Specifications

Changes to protocol specs (`specs/`) require:

- Clear rationale for changes
- Backward compatibility analysis
- Update to all affected SDKs
- Version bump in proto file

## Licensing

By contributing, you agree that your contributions will be licensed under the
Apache-2.0 license. All source files should include SPDX headers:

```rust
// SPDX-License-Identifier: Apache-2.0
```

Or for Python/shell scripts:

```python
# SPDX-License-Identifier: Apache-2.0
```

## Questions?

- Open a discussion on GitHub Discussions
- Email: [contact@secure-fabric.io](mailto:contact@secure-fabric.io)

Thank you for contributing to SecureFabric!
