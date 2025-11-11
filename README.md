# SecureFabric — Open SDKs & Protocol (Public)

[![CI]][ci-link]
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

[CI]: https://github.com/NodeCube/securefabric-public/actions/workflows/ci.yml/badge.svg
[ci-link]: https://github.com/NodeCube/securefabric-public/actions/workflows/ci.yml

SecureFabric is a secure, low-latency messaging fabric designed for verified
senders and end-to-end confidentiality.

## Repository Scope

**This public repository contains ONLY:**

- Client SDKs (Rust, JavaScript/TypeScript, Python)
- Protocol specifications (protobuf, API docs)
- Example applications and documentation
- Tools for SDK development and testing

**This repository does NOT contain:**

- SecureFabric node/server implementation (maintained privately)
- Infrastructure code, deployment configs, or operational tooling
- Production keys, certificates, or credentials

For node implementation, architecture decisions, or operational questions,
please refer to the private repository. This separation ensures the open-source
client libraries and protocol remain independent from proprietary server code.

## What's here

- **SDKs**: Rust, JS/TS, Python client libraries
- **Specs**: Protocol overview, `.proto`, API docs
- **Examples**: Minimal send/receive apps per language
- **Docs**: Quickstart, architecture

## Quickstart

All examples use placeholder credentials. Replace with your actual endpoint and token.

### Rust

```bash
cd examples/rust
cargo run --bin demo -- --endpoint YOUR_ENDPOINT_HERE --token YOUR_TOKEN_HERE
```

### JavaScript/TypeScript

```bash
cd examples/js
npm install
SF_ENDPOINT=YOUR_ENDPOINT_HERE SF_TOKEN=YOUR_TOKEN_HERE node index.js
```

### Python

```bash
cd examples/python
pip install -r requirements.txt
SF_ENDPOINT=YOUR_ENDPOINT_HERE SF_TOKEN=YOUR_TOKEN_HERE python send_receive.py
```

## Protocol & Architecture

- **Architecture Overview**: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - Repository structure and protocol sync flow
- **Technical Architecture**: [docs/architecture.md](docs/architecture.md)
- **Quickstart**: [docs/quickstart.md](docs/quickstart.md)
- **Protocol Spec**: [specs/securefabric.proto](specs/securefabric.proto)
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

## Trademark Notice

"SecureFabric" is a trademark. This repository contains only the public SDK
and protocol specifications. The production SecureFabric node is a separate
product with its own licensing.

## Support & Contact

- Website: [https://secure-fabric.io](https://secure-fabric.io)
- Documentation: [https://secure-fabric.io/docs](https://secure-fabric.io/docs)
- Email: [contact@secure-fabric.io](mailto:contact@secure-fabric.io)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). By contributing you agree to the Apache-2.0 license.

## Security

Please report vulnerabilities via [SECURITY.md](SECURITY.md) or email
[security@secure-fabric.io](mailto:security@secure-fabric.io).

## License

This repository is licensed under [Apache-2.0](LICENSE).

The SecureFabric production node (distributed separately) may have different
licensing terms. This repository contains only the open-source SDK and protocol
specifications.
