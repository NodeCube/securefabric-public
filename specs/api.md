# SecureFabric API Reference

This document describes the SecureFabric gRPC API. For protocol buffer definitions, see [securefabric.proto](securefabric.proto).

## Overview

SecureFabric provides a gRPC-based API for secure, low-latency messaging with end-to-end encryption and cryptographic authentication.

**Base URL**: Configured per deployment (e.g., `https://api.securefabric.io:50051`)

**Protocol**: gRPC over HTTP/2 with TLS

**Authentication**: Bearer token in `authorization` metadata header

## Authentication

All requests require a bearer token:

```text
authorization: Bearer <your-token-here>
```

Obtain tokens from your SecureFabric administrator or through the management API.

## Methods

### Send

Send a message to the fabric.

**RPC**: `securefabric.FabricNode/Send`

**Request**: `SendReq`

**Response**: `SendResp`

**Description**: Publishes a message envelope to the fabric. The envelope must be properly signed and include a unique nonce. Messages can be sent in plaintext or encrypted end-to-end.

**Example Request** (conceptual):

```json
{
  "envelope": {
    "pubkey": "<32-byte Ed25519 public key>",
    "sig": "<64-byte signature>",
    "nonce": "<24-byte unique nonce>",
    "aad": "<serialized additional authenticated data>",
    "payload": "<message payload - plaintext or ciphertext>",
    "seq": 1,
    "msg_id": "<blake3 hash>",
    "key_version": 0,
    "topic": "notifications.alerts"
  }
}
```

**Response**:

```json
{
  "ok": true,
  "msg_id": "<echo of message ID>"
}
```

**Errors**:

- `UNAUTHENTICATED` (16): Invalid bearer token
- `INVALID_ARGUMENT` (3): Malformed envelope or invalid signature
- `RESOURCE_EXHAUSTED` (8): Rate limit exceeded
- `UNAVAILABLE` (14): Node temporarily unavailable

### Subscribe

Subscribe to messages on a topic.

**RPC**: `securefabric.FabricNode/Subscribe`

**Request**: `SubscribeReq`

**Response**: Stream of `Envelope` messages

**Description**: Opens a streaming connection to receive messages matching the specified topic pattern. The stream remains open until the client closes it or the connection is lost.

**Example Request**:

```json
{
  "topic": "notifications.alerts"
}
```

**Response**: Stream of envelopes as they arrive

**Errors**:

- `UNAUTHENTICATED` (16): Invalid bearer token
- `INVALID_ARGUMENT` (3): Invalid topic pattern
- `UNAVAILABLE` (14): Node temporarily unavailable

### Stats

Get node statistics and metadata.

**RPC**: `securefabric.FabricNode/Stats`

**Request**: `StatsReq` (empty)

**Response**: `StatsResp`

**Description**: Returns operational statistics and build information about the node.

**Example Response**:

```json
{
  "peers": 3,
  "p95_latency_ms": 2.5,
  "version": "0.1.0",
  "git_sha": "a1b2c3d",
  "built": "2025-01-09T12:00:00Z",
  "rustc": "1.75.0"
}
```

**Errors**:

- `UNAUTHENTICATED` (16): Invalid bearer token

### Join

Connect this node to a peer node.

**RPC**: `securefabric.FabricNode/Join`

**Request**: `NodeInfo`

**Response**: `JoinResp`

**Description**: Establishes a peer connection for message forwarding. Requires administrative privileges.

**Example Request**:

```json
{
  "node_id": "node-2",
  "addr": "10.0.1.100:50051",
  "pubkey": "<32-byte Ed25519 public key>"
}
```

**Response**:

```json
{
  "ok": true
}
```

**Errors**:

- `PERMISSION_DENIED` (7): Insufficient privileges
- `ALREADY_EXISTS` (6): Peer already joined
- `UNAVAILABLE` (14): Cannot reach peer

### Unjoin

Remove a peer connection.

**RPC**: `securefabric.FabricNode/Unjoin`

**Request**: `NodeId`

**Response**: `JoinResp`

**Description**: Removes a peer connection. Requires administrative privileges.

**Example Request**:

```json
{
  "node_id": "node-2"
}
```

**Response**:

```json
{
  "ok": true
}
```

**Errors**:

- `PERMISSION_DENIED` (7): Insufficient privileges
- `NOT_FOUND` (5): Peer not found

## Message Envelope Structure

### Envelope Fields

| Field | Type | Description |
|-------|------|-------------|
| `pubkey` | bytes (32) | Ed25519 public key of the sender |
| `sig` | bytes (64) | Ed25519 signature over AAD + payload |
| `nonce` | bytes (24) | Unique XChaCha20 nonce (must never repeat for a given key) |
| `aad` | bytes | Additional Authenticated Data (topic, metadata) |
| `payload` | bytes | Message content (plaintext or E2E encrypted) |
| `seq` | uint64 | Monotonically increasing sequence number |
| `msg_id` | string | BLAKE3 hash: `hex(blake3(pubkey\|\|seq\|\|nonce))` |
| `key_version` | uint32 | E2E encryption key version (0 for plaintext) |
| `topic` | string | Message topic/channel |

### Signature Verification

Signatures are computed over:

```text
signature = Ed25519.sign(signing_key, aad || payload)
```

The node verifies signatures on ingress to prevent replay and ensure authenticity.

### Nonce Management

- **Length**: 24 bytes (XChaCha20)
- **Uniqueness**: Must be unique per sender public key
- **Generation**: Use a cryptographically secure RNG
- **Best Practice**: Combine counter + random bytes

### Topic Naming

Topics use dot-separated hierarchical naming:

```text
notifications.alerts.critical
users.alice.messages
system.events.audit
```

**Rules**:

- Lowercase alphanumeric + dots and hyphens
- Max length: 255 characters
- No leading/trailing dots

## Rate Limits

Default limits (configurable per deployment):

- **Send**: 100 messages/second per client
- **Subscribe**: 10 concurrent subscriptions per client
- **Burst**: 2x sustained rate for 10 seconds

Exceeding limits returns `RESOURCE_EXHAUSTED` error.

## Error Codes

SecureFabric uses standard gRPC status codes:

| Code | Name | Description |
|------|------|-------------|
| 0 | OK | Success |
| 3 | INVALID_ARGUMENT | Invalid request parameters |
| 5 | NOT_FOUND | Resource not found |
| 6 | ALREADY_EXISTS | Resource already exists |
| 7 | PERMISSION_DENIED | Insufficient permissions |
| 8 | RESOURCE_EXHAUSTED | Rate limit or quota exceeded |
| 14 | UNAVAILABLE | Service temporarily unavailable |
| 16 | UNAUTHENTICATED | Authentication required or failed |

## Security Considerations

1. **TLS Required**: All production deployments must use TLS 1.2+
2. **Token Security**: Store bearer tokens securely, rotate regularly
3. **Nonce Uniqueness**: Reusing nonces compromises security
4. **Signature Verification**: Always verify signatures on receive
5. **Message Size**: Limit payload sizes to prevent DoS (default: 1MB)

## SDK Support

Official SDKs handle authentication, signing, and envelope construction:

- **Rust**: `securefabric-sdk`
- **JavaScript/TypeScript**: `@securefabric/sdk`
- **Python**: `securefabric-sdk`

See [examples/](../examples/) for usage.

## Versioning

API version is included in the proto package and enforced by the server. Breaking changes increment the major version.

**Current Version**: `v1` (implied by `package securefabric`)

## Further Reading

- [Protocol Overview](protocol-overview.md)
- [Security Architecture](../docs/architecture.md)
- [SDK Documentation](../docs/sdk/)
