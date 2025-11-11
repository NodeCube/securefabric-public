# SecureFabric SDKs

This directory contains client SDKs for SecureFabric in multiple languages.

## Available SDKs

### Rust SDK

**Location:** `sdk/rust/`

**Install:**

```toml
[dependencies]
securefabric-sdk = "0.1"
```

**Build & Test:**

```bash
cd sdk/rust
make help       # Show available commands
make build      # Build the SDK
make test       # Run tests
make lint       # Run clippy
make fmt        # Format code
make check      # Run all checks
```

**Documentation:** [sdk/rust/README.md](rust/README.md)

### JavaScript/TypeScript SDK

**Location:** `sdk/js/`

**Install:**

```bash
npm install @securefabric/sdk
```

**Build & Test:**

```bash
cd sdk/js
make help       # Show available commands
make install    # Install dependencies
make build      # Build the SDK
make test       # Run tests
make lint       # Run linter
```

**Documentation:** [sdk/js/README.md](js/README.md)

### Python SDK

**Location:** `sdk/python/`

**Install:**

```bash
pip install securefabric-sdk
```

**Development:**

```bash
cd sdk/python
make help           # Show available commands
make dev-install    # Install in development mode
make test           # Run tests
make lint           # Run linters
make fmt            # Format code
make check          # Run all checks
```

**Documentation:** [sdk/python/README.md](python/README.md)

## Code Generation

All SDKs generate code from the protocol specification at `specs/securefabric.proto`.

### Automatic Codegen

Run codegen for all SDKs:

```bash
bash tools/sync-proto.sh
```

### Per-SDK Codegen

Run codegen for a specific SDK:

```bash
cd sdk/rust && make codegen
cd sdk/python && make codegen
cd sdk/js && make codegen
```

## Conformance Tests

All SDKs must pass conformance tests defined in `sdk/tests/test_vectors.json`.

**Run conformance tests:**

```bash
# Rust
cd sdk/rust && make test-conformance

# Python
cd sdk/python && make test-conformance

# JavaScript/TypeScript
cd sdk/js && make test-conformance
```

See [sdk/tests/README.md](tests/README.md) for details.

## Development Workflow

### Adding a New Feature

1. Update protocol spec if needed (`specs/securefabric.proto`)
2. Run codegen: `bash tools/sync-proto.sh`
3. Implement feature in all SDKs
4. Add tests (unit + conformance if applicable)
5. Update documentation
6. Run `make check` in each SDK directory
7. Open a pull request

### Adding a New Language SDK

1. Create directory: `sdk/new-language/`
2. Add `Makefile` with standard targets: `codegen`, `build`, `test`, `lint`, `check`
3. Implement crypto primitives (XChaCha20-Poly1305, Ed25519)
4. Implement conformance tests using `sdk/tests/test_vectors.json`
5. Add CI job in `.github/workflows/ci.yml`
6. Update this README
7. Update CODEOWNERS

## SDK Requirements

All SDKs must:

1. **Protocol Compliance:** Match the protobuf specification exactly
2. **Crypto Primitives:**
   - XChaCha20-Poly1305 for encryption
   - Ed25519 for signatures
   - Secure random number generation
3. **Conformance Tests:** Pass all test vectors
4. **Error Handling:** Proper error types and messages
5. **Documentation:** API docs, examples, and guides
6. **CI Integration:** Automated build and test
7. **License Headers:** SPDX-License-Identifier: Apache-2.0

## Testing Strategy

### Unit Tests

Test individual SDK functions in isolation.

### Integration Tests

Test SDK against mock/test servers (where applicable).

### Conformance Tests

Validate cryptographic operations against standard test vectors.

### Example Tests

Ensure example applications work end-to-end.

## Performance Considerations

- **Zero-copy:** Minimize data copying where possible
- **Async I/O:** Use async/await patterns (Rust, JS/TS, Python)
- **Connection pooling:** Reuse connections when appropriate
- **Buffering:** Efficient buffering for bulk operations

## Security Best Practices

- **No hardcoded credentials:** Always use environment variables or secure vaults
- **TLS required:** Enforce HTTPS/TLS in production
- **Input validation:** Validate all inputs before sending
- **Secure defaults:** Safe defaults for all configuration options
- **Dependency auditing:** Regular security audits of dependencies

## Versioning

SDKs follow [Semantic Versioning](https://semver.org/):

- **Major:** Breaking API changes
- **Minor:** New features, backward compatible
- **Patch:** Bug fixes, backward compatible

Protocol version is tracked separately in the proto file.

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

## License

Apache-2.0 - See [LICENSE](../LICENSE)
