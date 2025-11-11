# SecureFabric Quickstart

This guide will help you get started with SecureFabric SDKs.

## Prerequisites

- Access to a SecureFabric node endpoint
- Bearer authentication token
- TLS certificates (if using mTLS)

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
securefabric-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

### JavaScript/TypeScript

```bash
npm install @securefabric/sdk
```

### Python

```bash
pip install securefabric-sdk
```

## Basic Usage

### Rust Example

```rust
use securefabric_sdk::{SecureFabricClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig {
        endpoint: "YOUR_ENDPOINT_HERE".to_string(),
        bearer_token: Some("YOUR_TOKEN_HERE".to_string()),
        ..Default::default()
    };

    let client = SecureFabricClient::connect(config).await?;

    // Send a message
    client.send(
        b"test-topic",
        b"recipient-id",
        b"Hello, SecureFabric!"
    ).await?;

    // Subscribe to messages
    let mut stream = client.subscribe(b"test-topic").await?;
    while let Some(envelope) = stream.next().await {
        println!("Received message: {:?}", envelope);
    }

    Ok(())
}
```

### JavaScript Example

```javascript
import { SecureFabricClient } from '@securefabric/sdk';

const client = new SecureFabricClient({
  endpoint: process.env.SF_ENDPOINT || 'YOUR_ENDPOINT_HERE',
  bearerToken: process.env.SF_TOKEN || 'YOUR_TOKEN_HERE'
});

// Send a message
await client.send(
  'test-topic',
  'recipient-id',
  Buffer.from('Hello, SecureFabric!')
);

// Subscribe to messages
const stream = await client.subscribe('test-topic');
for await (const envelope of stream) {
  console.log('Received message:', envelope);
}
```

### Python Example

```python
import asyncio
from securefabric import SecureFabricClient

async def main():
    client = SecureFabricClient(
        target='YOUR_ENDPOINT_HERE',
        bearer='YOUR_TOKEN_HERE'
    )

    # Send a message
    ok = await client.send(
        topic=b'test-topic',
        to=b'recipient-id',
        payload=b'Hello, SecureFabric!'
    )
    print(f"Message sent: {ok}")

    # Subscribe to messages
    async for envelope in client.subscribe(topic=b'test-topic'):
        print(f"Received: {envelope}")

    await client.close()

if __name__ == '__main__':
    asyncio.run(main())
```

## Configuration

### Environment Variables

It's recommended to use environment variables for sensitive configuration:

```bash
export SF_ENDPOINT="YOUR_ENDPOINT_HERE"
export SF_TOKEN="YOUR_TOKEN_HERE"
```

### TLS Configuration

For production, always use TLS:

```rust
let config = ClientConfig {
    endpoint: "YOUR_ENDPOINT_HERE".to_string(),
    bearer_token: Some(std::env::var("SF_TOKEN")?),
    tls_enabled: true,
    ..Default::default()
};
```

## Next Steps

- Read the [Architecture](architecture.md) documentation
- Explore the [examples/](../examples/) directory
- Review the [API specifications](../specs/api.md)
- Check the [Protocol Buffers schema](../specs/securefabric.proto)

## Getting Help

- Documentation: [https://secure-fabric.io/docs](https://secure-fabric.io/docs)
- Issues: [GitHub Issues](https://github.com/NodeCube/securefabric-public/issues)
- Security: See [SECURITY.md](../SECURITY.md)

## Production Deployment

For production deployment of SecureFabric nodes, contact [https://secure-fabric.io](https://secure-fabric.io).
