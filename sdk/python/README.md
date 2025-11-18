# SecureFabric Python Client SDK

[![PyPI version](https://badge.fury.io/py/securefabric-client.svg)](https://badge.fury.io/py/securefabric-client)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

Official Python client library for SecureFabric - a secure, low-latency
messaging fabric designed for verified senders and end-to-end confidentiality.

## Features

- **Async/await API** - Built on asyncio and gRPC for high performance
- **TLS/mTLS support** - Secure connections with optional mutual authentication
- **Bearer token auth** - Simple token-based authentication
- **Type hints** - Full type annotations for better IDE support
- **Production-ready** - Comprehensive error handling and logging

## Installation

Install from PyPI:

```bash
pip install securefabric-client
```

Install with development dependencies:

```bash
pip install securefabric-client[dev]
```

## Quick Start

### Basic Publishing

```python
import asyncio
from securefabric import SecureFabricClient

async def main():
    # Connect to a SecureFabric node
    client = SecureFabricClient(
        target="node.example.com:50051",
        bearer_token="your-token-here"
    )

    # Publish a message
    success = await client.publish("sensors/temperature", b"22.5C")
    print(f"Message published: {success}")

    await client.close()

asyncio.run(main())
```

### Subscribing to Topics

```python
import asyncio
from securefabric import SecureFabricClient

async def main():
    client = SecureFabricClient(
        target="node.example.com:50051",
        bearer_token="your-token-here"
    )

    # Subscribe with callback
    def handle_message(envelope):
        print(f"Received from {envelope.topic}: {envelope.payload.decode()}")

    # This runs indefinitely - use Ctrl+C to stop
    await client.subscribe("sensors/#", handle_message)

asyncio.run(main())
```

### Streaming Subscription

```python
import asyncio
from securefabric import SecureFabricClient

async def main():
    client = SecureFabricClient(
        target="node.example.com:50051",
        bearer_token="your-token-here"
    )

    # Subscribe as async iterator
    async for envelope in client.subscribe_stream("sensors/#"):
        print(f"Topic: {envelope.topic}")
        print(f"Payload: {envelope.payload.decode()}")
        print(f"Sender: {envelope.pubkey.hex()}")
        print(f"Sequence: {envelope.seq}")
        print("---")

asyncio.run(main())
```

### Using mTLS (Mutual TLS)

```python
import asyncio
from securefabric import SecureFabricClient

async def main():
    # Load certificates
    with open("ca.crt", "rb") as f:
        ca_cert = f.read()
    with open("client.crt", "rb") as f:
        client_cert = f.read()
    with open("client.key", "rb") as f:
        client_key = f.read()

    # Connect with mTLS
    client = SecureFabricClient(
        target="node.example.com:50051",
        ca_cert=ca_cert,
        client_cert=client_cert,
        client_key=client_key,
    )

    await client.publish("secure/topic", b"sensitive data")
    await client.close()

asyncio.run(main())
```

### Getting Node Statistics

```python
import asyncio
from securefabric import SecureFabricClient

async def main():
    client = SecureFabricClient(
        target="node.example.com:50051",
        bearer_token="your-token-here"
    )

    stats = await client.stats()
    print(f"Connected peers: {stats.peers}")
    print(f"P95 latency: {stats.p95_latency_ms}ms")
    print(f"Node version: {stats.version}")
    print(f"Git SHA: {stats.git_sha}")

    await client.close()

asyncio.run(main())
```

## Configuration

### Environment Variables

The client can be configured via environment variables:

```bash
export SF_ENDPOINT="node.example.com:50051"
export SF_TOKEN="your-bearer-token"
export SF_CA_CERT_PATH="/path/to/ca.crt"
export SF_CLIENT_CERT_PATH="/path/to/client.crt"
export SF_CLIENT_KEY_PATH="/path/to/client.key"
```

### Connection Options

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `target` | str | required | Node address as "host:port" |
| `tls` | bool | True | Enable TLS encryption |
| `ca_cert` | bytes | None | CA certificate in PEM format |
| `client_cert` | bytes | None | Client certificate for mTLS |
| `client_key` | bytes | None | Client private key for mTLS |
| `bearer_token` | str | None | Bearer token for authentication |
| `insecure` | bool | False | Allow insecure connections (not recommended) |

## Development

### Setup

```bash
# Clone the repository
git clone https://github.com/NodeCube/securefabric-public.git
cd securefabric-public/sdk/python

# Install in editable mode with dev dependencies
pip install -e ".[dev]"
```

### Generate Protobuf Stubs

The repository includes pre-generated protobuf files, but you can regenerate them:

```bash
python -m grpc_tools.protoc \
    -I../../specs \
    --python_out=securefabric \
    --grpc_python_out=securefabric \
    --pyi_out=securefabric \
    ../../specs/securefabric.proto
```

### Running Tests

```bash
# Run unit tests
pytest tests/

# Run with coverage
pytest --cov=securefabric tests/

# Type checking
mypy securefabric/

# Linting
black --check securefabric/
```

### Building and Publishing

```bash
# Install build tools
pip install build twine

# Build distribution packages
python -m build

# Upload to TestPyPI
python -m twine upload --repository testpypi dist/*

# Upload to PyPI
python -m twine upload dist/*
```

## API Reference

### SecureFabricClient

The main client class for interacting with SecureFabric nodes.

#### Methods

##### `__init__(target, tls=True, ca_cert=None, client_cert=None, client_key=None, bearer_token=None, insecure=False)`

Initialize a new client instance.

##### `async publish(topic: str, payload: Union[bytes, str], aad: Optional[bytes] = None) -> bool`

Publish a message to a topic. Returns `True` if successful.

##### `async subscribe(topic: str, callback: Callable[[Envelope], None]) -> None`

Subscribe to a topic with a callback function. Runs indefinitely.

##### `async subscribe_stream(topic: str) -> AsyncIterator[Envelope]`

Subscribe to a topic as an async iterator.

##### `async stats() -> StatsResp`

Get node statistics and metadata.

##### `async close() -> None`

Close the connection and cleanup resources.

## Envelope Structure

Messages are wrapped in an `Envelope` protobuf with the following fields:

- `pubkey` (bytes): Ed25519 public key of sender (32 bytes)
- `sig` (bytes): Signature over aad||header||payload (64 bytes)
- `nonce` (bytes): XChaCha nonce (24 bytes)
- `aad` (bytes): Additional Authenticated Data
- `payload` (bytes): Message content (plaintext or encrypted)
- `seq` (int): Strictly increasing sequence number
- `msg_id` (str): Unique message identifier
- `key_version` (int): E2E topic key version
- `topic` (str): Message topic

## Security Considerations

1. **Always use TLS in production** - Set `tls=True` and provide `ca_cert`
2. **Protect bearer tokens** - Never commit tokens to version control
3. **Use mTLS for sensitive applications** - Provides mutual authentication
4. **Validate envelope signatures** - Check `sig` field against `pubkey`
5. **Handle sequence numbers** - Detect replay attacks using `seq` field

## Troubleshooting

### Connection Errors

```text
grpc._channel._InactiveRpcError: <_InactiveRpcError of RPC that terminated with: UNAVAILABLE>
```

**Solution**: Check that:

- The node address is correct
- TLS certificates are valid
- Network connectivity is available
- The node is running

### Authentication Errors

```text
grpc._channel._InactiveRpcError: <_InactiveRpcError of RPC that terminated with: UNAUTHENTICATED>
```

**Solution**: Verify your bearer token or client certificates are correct.

### Import Errors

```text
ImportError: cannot import name 'securefabric_pb2'
```

**Solution**: Regenerate protobuf stubs using the command in the Development section.

## Support

- **Documentation**: [https://secure-fabric.io/docs](https://secure-fabric.io/docs)
- **Issues**: [https://github.com/NodeCube/securefabric-public/issues](https://github.com/NodeCube/securefabric-public/issues)
- **Email**: [contact@secure-fabric.io](mailto:contact@secure-fabric.io)

## License

This SDK is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

The SecureFabric production node (distributed separately) may have different licensing terms.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
