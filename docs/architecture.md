# SecureFabric Architecture

## Overview

SecureFabric is a secure messaging fabric that provides:

- **End-to-end encryption** using ChaCha20-Poly1305 AEAD
- **Message authentication** with Ed25519 digital signatures
- **Replay protection** via nonce management
- **Low-latency delivery** with streaming gRPC protocol

## Components

### SDKs (Client Libraries)

Public SDKs are available in multiple languages:

- **Rust SDK**: High-performance, zero-copy operations
- **JavaScript/TypeScript SDK**: Browser and Node.js support
- **Python SDK**: Async/await with asyncio

### Protocol

Communication uses gRPC with Protocol Buffers for serialization:

- **Transport**: HTTP/2 with TLS 1.3
- **Authentication**: Bearer token in metadata
- **Format**: Protocol Buffers v3

### Message Envelope

Each message includes:

```protobuf
Envelope {
  pubkey: bytes        // Ed25519 public key (32 bytes)
  sig: bytes           // Ed25519 signature (64 bytes)
  nonce: bytes         // Random nonce for replay protection (32 bytes)
  aad: bytes           // Additional authenticated data (JSON)
  payload: bytes       // Encrypted message payload
  seq: uint64          // Sequence number
  msg_id: string       // BLAKE3 hash of (pubkey || seq || nonce)
  key_version: uint32  // Key rotation version
  topic: string        // Message topic/channel
}
```

## Security Model

### Encryption

- **Algorithm**: ChaCha20-Poly1305 AEAD
- **Key derivation**: Application-provided keys
- **IV/Nonce**: 192-bit random nonce per message

### Signatures

- **Algorithm**: Ed25519 (EdDSA on Curve25519)
- **Signed data**: AAD || payload
- **Verification**: Server-side optional, client-side mandatory

### Authentication

- **Method**: Bearer token in gRPC metadata
- **Token format**: Opaque string provided by SecureFabric service
- **Scope**: Per-connection authentication

## Message Flow

1. Client creates message with payload
2. Client generates random nonce
3. Client signs (AAD || payload) with Ed25519 private key
4. Client computes message ID: BLAKE3(pubkey || seq || nonce)
5. Client sends Envelope to server via gRPC
6. Server validates signature (optional)
7. Server routes message to subscribers
8. Subscribers receive Envelope
9. Subscribers verify signature
10. Subscribers decrypt payload

## Deployment

SecureFabric nodes are deployed separately from client SDKs. This repository
contains only the client libraries and protocol specifications.

For production deployment, contact [https://secure-fabric.io](https://secure-fabric.io).

## Performance Characteristics

- **Latency**: Sub-millisecond message delivery (local network)
- **Throughput**: Depends on node deployment
- **Message size**: Up to 16MB per message (configurable)
- **Connections**: HTTP/2 multiplexing for efficiency

## Best Practices

1. **Rotate keys periodically** using key_version field
2. **Use TLS** for all connections
3. **Store tokens securely** (never hardcode)
4. **Validate all inputs** before sending
5. **Monitor for errors** and handle reconnection
6. **Rate limit** to avoid overwhelming the node

## Reference Implementation

See `examples/` directory for minimal working implementations in each supported language.
