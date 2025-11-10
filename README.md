# SecureFabric — Open SDKs & Protocol (Public)

[![CI](https://github.com/NodeCube/securefabric-public/actions/workflows/ci.yml/badge.svg)](
https://github.com/NodeCube/securefabric-public/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

SecureFabric is a secure, low-latency messaging fabric designed for verified
senders and end-to-end confidentiality. This public repo hosts SDKs, examples,
and protocol specs. The production node is available as a signed binary under
a commercial license.

## What's here

- **SDKs**: Rust, JS/TS, Python
- **Specs**: Protocol overview, `.proto`, API docs
- **Examples**: Minimal send/receive apps per language
- **Docs**: Quickstart, architecture

## Quickstart (Rust)

```bash
cd examples/rust
cargo run --bin demo -- --endpoint https://api.securefabric.io --token $SF_TOKEN
```

## Quickstart (JS/TS)

```bash
cd examples/js
npm install
SF_TOKEN=your-token node index.js
```

## Quickstart (Python)

```bash
cd examples/python
pip install -r requirements.txt
SF_TOKEN=your-token python send_receive.py
```

## Protocol

- **Overview**: [docs/architecture.md](docs/architecture.md)
- **Messages**: [specs/securefabric.proto](specs/securefabric.proto)
- **API Reference**: [specs/api.md](specs/api.md)

## Using the SDKs in Your Project

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
securefabric-sdk = "0.1"
```

### JavaScript/TypeScript

```bash
npm install @securefabric/sdk
```

### Python

```bash
pip install securefabric-sdk
```

## Running your own node

The production-grade node is distributed as a signed binary:

1. Download from [SecureFabric Releases](https://github.com/NodeCube/securefabric-core/releases)
1. Verify signature with `tools/verify-release-signature.sh`
1. Follow deployment guide in the private repository documentation

## Commercial node & hosting

To join the private beta or request support:

- Website: [https://securefabric.io](https://securefabric.io)
- Email: [legal@nodecube.io](mailto:legal@nodecube.io)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). By contributing you agree to the Apache-2.0 license.

## Security

Please report vulnerabilities via [SECURITY.md](SECURITY.md).

## License

This repository is licensed under [Apache-2.0](LICENSE).
