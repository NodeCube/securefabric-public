# Changelog

All notable changes to the SecureFabric public SDKs and specifications will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security
- Comprehensive security hardening of public repository
- Added automated secret scanning with gitleaks and trufflehog
- Added dependency vulnerability scanning
- Removed any sensitive artifacts from history

### Added
- CODE_OF_CONDUCT.md with Contributor Covenant 2.1
- CHANGELOG.md for tracking changes
- Comprehensive .gitignore for secrets and build artifacts
- SPDX license headers on all source files
- Documentation quality gates (markdownlint, yamllint, link checker)
- Dependency security checks (cargo audit, cargo deny, npm audit)
- Public audit CI job to prevent sensitive data commits

### Changed
- Updated all contact emails to public addresses (secure-fabric.io domain)
- Minimized examples to use placeholder credentials only
- Streamlined CI pipeline for faster execution (<5 min target)
- Updated documentation to remove internal references

### Removed
- Any internal endpoints, credentials, or proprietary code
- Build artifacts and generated code from repository

## [0.1.0] - 2025-01-11

### Added
- Initial public release of SDKs (Rust, JavaScript/TypeScript, Python)
- Protocol specifications (Protocol Buffers schema)
- API documentation and examples
- Basic CI pipeline for linting and testing

[Unreleased]: https://github.com/NodeCube/securefabric-public/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/NodeCube/securefabric-public/releases/tag/v0.1.0
