# SecureFabric Architecture Overview

This document explains the architecture of SecureFabric, the relationship between the public SDK
repository and the private node implementation, and how protocol releases flow between them.

## Repository Split: Public vs. Private

SecureFabric uses a split repository architecture to balance open-source transparency with operational security:

### Public Repository (This Repo)

**Repository:** `NodeCube/securefabric-public`

**Contents:**

- Client SDKs (Rust, JavaScript/TypeScript, Python)
- Protocol specifications (protobuf, API documentation)
- Example applications
- SDK documentation and guides
- Test vectors and conformance tests

**License:** Apache-2.0

**Purpose:** Enable developers to build applications using SecureFabric with fully open-source client libraries.

### Private Repository

**Repository:** `NodeCube/securefabric-private` (restricted access)

**Contents:**

- SecureFabric node/server implementation
- Infrastructure and deployment code
- Operational tooling and monitoring
- Private protocol development and testing
- Production configurations

**License:** Proprietary

**Purpose:** Maintain security and operational control over the production SecureFabric network.

## Architecture Diagram

```text
┌─────────────────────────────────────────────────────────────┐
│                    Public Repository                        │
│                (securefabric-public)                        │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  Rust SDK    │  │   JS/TS SDK  │  │  Python SDK  │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│                                                             │
│  ┌──────────────────────────────────────────────────┐      │
│  │         Protocol Specifications                  │      │
│  │  - securefabric.proto                            │      │
│  │  - API documentation                             │      │
│  │  - Test vectors                                  │      │
│  └──────────────────────────────────────────────────┘      │
│                                                             │
│  ┌──────────────────────────────────────────────────┐      │
│  │         Examples & Documentation                 │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ Protocol Sync
                           │ (GitHub Releases)
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   Private Repository                        │
│               (securefabric-private)                        │
│                                                             │
│  ┌──────────────────────────────────────────────────┐      │
│  │      SecureFabric Node Implementation            │      │
│  │  - gRPC server                                   │      │
│  │  - Message routing                               │      │
│  │  - Authentication & authorization                │      │
│  │  - Cryptographic operations                      │      │
│  └──────────────────────────────────────────────────┘      │
│                                                             │
│  ┌──────────────────────────────────────────────────┐      │
│  │      Infrastructure & Deployment                 │      │
│  │  - Kubernetes configs                            │      │
│  │  - Terraform/Ansible                             │      │
│  │  - Monitoring & logging                          │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## Protocol Release Flow

Protocol changes originate in the private repository and flow to the public SDK repository through a controlled release process:

### 1. Private Repository: Protocol Development

1. Protocol changes are developed and tested in the private repository
2. Changes are reviewed and approved by the protocol team
3. Node implementation is updated to support new protocol features
4. Integration tests verify compatibility

### 2. Private Repository: Release Creation

When ready to publish:

1. Tag a release in the private repository: `v0.2.0`
2. GitHub workflow `sync-proto.yml` triggers automatically
3. Workflow packages protocol artifacts:
   - `specs/securefabric.proto`
   - `schemas/*.json` (if applicable)
   - Relevant section from `CHANGELOG.md`
4. Creates a GitHub Release with artifacts
5. Dispatches webhook to public repository

### 3. Public Repository: Protocol Sync

The public repository receives the update:

1. GitHub workflow `pull-proto.yml` receives webhook
2. Downloads protocol artifacts from private repo release
3. Updates `/specs/proto/` and `/specs/schemas/`
4. Runs SDK codegen via `tools/sync-proto.sh`
5. Creates a PR with title `chore(specs): sync protocol vX.Y.Z`

### 4. Public Repository: Integration

1. CI runs automatically on the sync PR
2. All SDK tests must pass
3. Conformance tests validate protocol compliance
4. Maintainers review breaking changes
5. PR is merged to `main`
6. SDKs are updated in public package registries

## Workflow Diagram

```text
Private Repo                              Public Repo
────────────                              ───────────

  Protocol                                    │
  Changes                                     │
     │                                        │
     ▼                                        │
  Review &                                    │
  Testing                                     │
     │                                        │
     ▼                                        │
Tag Release ─── GitHub Release ────────────► │
   v0.2.0       (Artifacts)                   │
     │                                        ▼
     │                                   pull-proto.yml
     │                                   (workflow)
     │                                        │
     └─────── Webhook Dispatch ─────────────►│
              (repository_dispatch)           │
                                              ▼
                                        Download
                                        Artifacts
                                              │
                                              ▼
                                        Update Specs
                                              │
                                              ▼
                                        Run Codegen
                                        (sync-proto.sh)
                                              │
                                              ▼
                                        Create PR
                                              │
                                              ▼
                                        CI Validation
                                              │
                                              ▼
                                        Maintainer
                                        Review
                                              │
                                              ▼
                                        Merge to main
```

## SDK Code Generation

SDKs use protocol-specific code generation tools:

### Rust SDK

- **Tool:** `prost` (Protocol Buffers) + `tonic` (gRPC)
- **Build:** Code generation happens during `cargo build` via `build.rs`
- **Location:** Generated code in `sdk/rust/src/generated/`

### Python SDK

- **Tool:** `grpcio-tools`
- **Build:** Manual generation via `python -m grpc_tools.protoc`
- **Location:** Generated code in `sdk/python/securefabric/*_pb2.py`

### JavaScript/TypeScript SDK

- **Tool:** `ts-proto` or `protoc-gen-ts`
- **Build:** `npm run codegen` script in package.json
- **Location:** Generated code in `sdk/js/src/generated/`

All codegen is automated via `tools/sync-proto.sh`.

## Issue Routing: Where to File Issues

### File Issues in Public Repo (This Repo) For

- SDK bugs or feature requests
- Client library performance issues
- Documentation improvements
- Protocol specification clarifications
- Example application problems
- Build/CI issues with SDKs

### Contact Private Repo Team For

- Node/server bugs or crashes
- Authentication/authorization issues
- Production outages or performance degradation
- Infrastructure or deployment questions
- Security vulnerabilities in the node
- Operational concerns

**Security Issues:** Always report security vulnerabilities via `security@secure-fabric.io` or GitHub Security Advisories.

## Development Workflow

### Contributing to SDKs (Public)

1. Fork the public repository
2. Create a feature branch
3. Make SDK improvements
4. Add/update tests
5. Update documentation
6. Open a pull request
7. Pass CI checks
8. Get maintainer approval
9. Merge to main

### Contributing to Protocol (Private → Public)

1. Develop protocol changes in private repo
2. Update node implementation
3. Test thoroughly with integration tests
4. Create protocol release in private repo
5. Automated sync to public repo
6. Update all SDKs to support new protocol
7. Merge sync PR in public repo

## Security Model

### Public Repository Security

- All code is open-source and auditable
- No production credentials or keys
- Protocol specifications are public
- Test vectors use synthetic data only
- Secret scanning enabled in CI

### Private Repository Security

- Restricted access to core team
- Production credentials secured
- Infrastructure code protected
- Operational monitoring data private
- Incident response procedures confidential

## Testing Strategy

### Public Repository Tests

- **Unit Tests:** Test individual SDK functions
- **Conformance Tests:** Validate against test vectors
- **Integration Tests:** Test SDK against mock servers
- **Example Tests:** Verify example apps work

### Private Repository Tests

- **Unit Tests:** Test node components
- **Integration Tests:** Full stack testing
- **Load Tests:** Performance and scalability
- **Security Tests:** Penetration testing and audits

## Release Cadence

- **Protocol Releases:** As needed (typically quarterly)
- **SDK Releases:** Following protocol updates + bug fixes
- **Node Releases:** Independent of public releases

## Contact & Support

- **Public SDK Issues:** GitHub Issues in this repository
- **Private Node Issues:** Contact `support@secure-fabric.io`
- **Security Issues:** `security@secure-fabric.io`
- **General Inquiries:** `contact@secure-fabric.io`

## Further Reading

- [README.md](../README.md) - Getting started
- [CONTRIBUTING.md](../CONTRIBUTING.md) - How to contribute
- [SECURITY.md](../SECURITY.md) - Security policy
- [quickstart.md](quickstart.md) - Quick start guide
- [api.md](../specs/api.md) - API reference
