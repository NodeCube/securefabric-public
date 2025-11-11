# SecureFabric SDK Conformance Tests

This directory contains conformance tests and test vectors for validating SecureFabric SDK implementations.

## Test Vectors

The `test_vectors.json` file contains standardized test cases that all SDK implementations must pass:

### Encryption Tests (XChaCha20-Poly1305)

- Basic encryption/decryption
- Empty plaintext handling
- Additional authenticated data (AAD)
- Round-trip verification

### Signature Tests (Ed25519)

- Signature generation
- Signature verification
- Empty message handling
- Long message handling

### Replay Protection Tests

- Sequential counter acceptance
- Duplicate counter rejection
- Out-of-order handling within window
- Counter window overflow

### Tamper Detection Tests

- Modified ciphertext rejection
- Modified authentication tag rejection
- Integrity verification

## Implementation Requirements

Each SDK must implement conformance tests that:

1. Load test vectors from `test_vectors.json`
2. Execute all test cases
3. Validate results match expected outcomes
4. Report failures with detailed diagnostics

### Rust SDK

Location: `sdk/rust/tests/conformance_tests.rs`

Run with:

```bash
cd sdk/rust
cargo test conformance
```

### Python SDK

Location: `sdk/python/tests/test_conformance.py`

Run with:

```bash
cd sdk/python
pytest tests/test_conformance.py -v
```

### JavaScript/TypeScript SDK

Location: `sdk/js/tests/conformance.test.ts`

Run with:

```bash
cd sdk/js
npm test -- conformance
```

## Adding New Test Vectors

When adding protocol features:

1. Add test vectors to `test_vectors.json`
2. Update all SDK conformance tests
3. Ensure all tests pass before merging
4. Document any breaking changes

## Test Vector Format

### Encryption Test

```json
{
  "description": "Human-readable description",
  "key": "hex-encoded 32-byte key",
  "nonce": "hex-encoded 24-byte nonce",
  "plaintext": "hex-encoded plaintext",
  "aad": "hex-encoded additional authenticated data",
  "ciphertext": "hex-encoded expected ciphertext",
  "tag": "hex-encoded authentication tag"
}
```

### Signature Test

```json
{
  "description": "Human-readable description",
  "secret_key": "hex-encoded 32-byte secret key",
  "public_key": "hex-encoded 32-byte public key",
  "message": "hex-encoded message",
  "signature": "hex-encoded 64-byte signature"
}
```

## CI Integration

Conformance tests run automatically in CI for all SDKs. PRs must pass all conformance tests before merge.

See `.github/workflows/ci.yml` for details.
