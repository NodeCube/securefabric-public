# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in SecureFabric SDKs or protocol specifications, please report it responsibly:

1. **DO NOT** open a public GitHub issue
2. Email us at [security@nodecube.io](mailto:security@nodecube.io)
3. Use GitHub's private security advisory feature
4. Include detailed information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fixes (if any)

## Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 5 business days
- **Resolution**: Coordinated with reporter
- **Disclosure**: Coordinated timing with reporter

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Considerations

### SDK Security

- All SDKs communicate with SecureFabric nodes over TLS
- Bearer token authentication required
- No plaintext credential storage in examples
- Input validation on all SDK methods

### Protocol Security

- End-to-end encryption using ChaCha20-Poly1305 AEAD
- Ed25519 signature verification (when enabled on node)
- Replay protection via nonce management
- Message integrity via authenticated encryption

### Best Practices for SDK Users

1. **Credentials**: Store tokens in environment variables or secure vaults, never in code
2. **TLS**: Always use HTTPS/TLS endpoints in production
3. **Validation**: Validate all inputs before sending messages
4. **Updates**: Keep SDKs updated to latest versions
5. **Monitoring**: Log and monitor API errors and authentication failures

## Known Limitations

- SDKs rely on node-side security controls
- No built-in rate limiting in client libraries
- Token refresh must be handled by application

## Security Updates

Security updates will be announced via:

- GitHub Security Advisories
- Release notes
- Email notification to registered users

Thank you for helping keep SecureFabric secure!
