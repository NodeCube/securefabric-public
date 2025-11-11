# Repository Scope

This document clarifies what is and is not included in the `securefabric-public`
repository.

## What IS in This Repository

### Client SDKs (Open Source)

- **Rust SDK** (`sdk/rust/`): High-performance client library
- **JavaScript/TypeScript SDK** (`sdk/js/`): Browser and Node.js support
- **Python SDK** (`sdk/python/`): Async/await client library

These SDKs are licensed under Apache-2.0 and are free to use, modify, and
distribute.

### Protocol Specifications

- **Protocol Buffers Schema** (`specs/securefabric.proto`): Message format definitions
- **API Documentation** (`specs/api.md`): gRPC service interface
- **Architecture Overview** (`docs/architecture.md`): System design and security model

These specifications are open and can be used to implement compatible clients or
servers.

### Examples

- **Minimal working examples** in each language demonstrating:
  - Client initialization with placeholders
  - Sending messages
  - Subscribing to topics
  - Basic error handling

All examples use `YOUR_ENDPOINT_HERE` and `YOUR_TOKEN_HERE` placeholders.

### Documentation

- **Quickstart guides** for each SDK
- **Contributing guidelines**
- **Security policy** for reporting vulnerabilities
- **Code of Conduct** for community standards

## What is NOT in This Repository

### SecureFabric Node (Production Server)

The production-grade SecureFabric node is **not** included in this repository. The
node is:

- Distributed as a separate signed binary
- May have different licensing terms
- Includes additional features not described in public specs
- Contains proprietary optimizations and security features

For information about obtaining the SecureFabric node, visit
[https://secure-fabric.io](https://secure-fabric.io).

### Deployment Infrastructure

This repository does not include:

- Infrastructure-as-Code (Terraform, Ansible, etc.)
- Kubernetes manifests or Helm charts
- Docker Compose files for production
- Deployment scripts or automation
- Configuration management
- Monitoring and logging setups

These are provided separately to SecureFabric customers.

### Internal Development Tools

Not included:

- Internal build pipelines
- Release signing keys
- SBOMs (Software Bill of Materials) for the node
- Performance benchmarking tools
- Load testing infrastructure
- Internal runbooks and documentation

### Credentials and Secrets

This repository contains **no**:

- API keys or bearer tokens
- TLS certificates or private keys
- Service account credentials
- Database connection strings
- Internal endpoints or IP addresses
- Production configuration

All examples use placeholder values like `YOUR_TOKEN_HERE`.

### Node Implementation

The SecureFabric node implementation is separate and not open source. This includes:

- Message routing logic
- Persistence layer
- Clustering and high availability
- Rate limiting and throttling
- Metrics and observability
- Admin APIs
- Multi-tenancy support

## Why This Separation?

### Open SDK, Proprietary Node Model

SecureFabric uses an **open SDK, proprietary node** model similar to:

- **MongoDB**: Open drivers, proprietary database
- **Kafka**: Open clients, commercial Confluent platform
- **Redis**: Open client libraries, commercial Redis Enterprise
- **Elasticsearch**: Open APIs, commercial features

This approach provides:

1. **Transparency**: Protocol and client behavior are fully documented
2. **Compatibility**: Anyone can implement compatible clients
3. **Community**: Open-source SDKs benefit from community contributions
4. **Sustainability**: Commercial node sales fund continued development

### Security Through Separation

Keeping node implementation separate:

- **Reduces attack surface**: No production code in public repos
- **Prevents information disclosure**: Security measures not revealed
- **Allows rapid patching**: Security fixes without public disclosure
- **Protects IP**: Core innovations remain proprietary

## Building Your Own Node

While the reference node is proprietary, the protocol specifications in this
repository are sufficient to implement a compatible server. If you build your own
node implementation:

- It must correctly implement the protocol in `specs/securefabric.proto`
- SDK tests can be used to verify compatibility
- Consider contributing protocol improvements via issues/PRs

## Public vs. Private Releases

| Component | Public (this repo) | Private | Where to Get |
|-----------|-------------------|---------|--------------|
| Rust SDK | ✅ Yes | ❌ No | crates.io, GitHub |
| JS/TS SDK | ✅ Yes | ❌ No | npm, GitHub |
| Python SDK | ✅ Yes | ❌ No | PyPI, GitHub |
| Protocol specs | ✅ Yes | ❌ No | This repo |
| Examples | ✅ Yes (sanitized) | ❌ No | This repo |
| Node binary | ❌ No | ✅ Yes | secure-fabric.io |
| Deployment tools | ❌ No | ✅ Yes (for customers) | Customer portal |
| SBOMs | ❌ No | ✅ Yes (for customers) | Customer portal |
| Release signatures | Public key only | ❌ Signing key private | `tools/verify-release-signature.sh` |

## Obtaining the Production Node

To run SecureFabric in production:

1. Visit [https://secure-fabric.io](https://secure-fabric.io)
2. Contact sales for licensing information
3. Download signed binaries from the customer portal
4. Verify signatures using `tools/verify-release-signature.sh`
5. Follow deployment guide (provided to customers)

## Contributing

Contributions to the **public SDKs and documentation** are welcome! See
[CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

For issues or questions about the **production node**, contact
[support@secure-fabric.io](mailto:support@secure-fabric.io).

## License Summary

- **This repository (SDKs, specs, docs)**: Apache-2.0
- **SecureFabric node**: Proprietary license (see customer agreement)
- **SecureFabric trademark**: ™ All rights reserved

## Questions?

- **SDK/Protocol questions**: Open a GitHub issue
- **Node/Deployment questions**: [contact@secure-fabric.io](mailto:contact@secure-fabric.io)
- **Security issues**: [security@secure-fabric.io](mailto:security@secure-fabric.io)
- **Sales inquiries**: [https://secure-fabric.io/contact](https://secure-fabric.io/contact)
