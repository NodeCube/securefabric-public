# Key Management Guide

This guide explains how to generate, store, and use Ed25519 signing keys with the SecureFabric SDKs.

## Overview

SecureFabric uses Ed25519 digital signatures to verify message authenticity. Each client needs:
- **Private Key (Signing Key)**: 32 bytes, keep secret
- **Public Key (Verifying Key)**: 32 bytes, derived from private key, shared publicly

## Security Best Practices

### 1. Key Generation

Always generate keys using cryptographically secure random number generators:

**Rust:**
```rust
use securefabric_sdk::SigningKey;
use rand::rngs::OsRng;

let signing_key = SigningKey::generate(&mut OsRng);
```

**JavaScript/TypeScript:**
```typescript
import * as ed25519 from '@noble/ed25519';

const signingKey = ed25519.utils.randomPrivateKey();
```

**Python:**
```python
import nacl.signing

signing_key = nacl.signing.SigningKey.generate()
```

### 2. Key Storage

**DO NOT:**
- ❌ Store keys in source code
- ❌ Commit keys to version control
- ❌ Share private keys over insecure channels
- ❌ Store keys in plain text files in production

**DO:**

- ✅ Use environment variables for development
- ✅ Use secret management systems in production (AWS KMS, HashiCorp Vault,
  etc.)
- ✅ Use hardware security modules (HSM) for high-security applications
- ✅ Set restrictive file permissions (0600) for key files
- ✅ Encrypt keys at rest

### 3. File-Based Storage (Development Only)

If using file-based keys for development:

**Generate and save a key:**

```bash
# Rust example
cargo run --example securefabric-demo -- --mode send > /dev/null
# This will generate a new key

# Or generate manually
dd if=/dev/urandom of=signing_key.bin bs=1 count=32
chmod 600 signing_key.bin
```

**Load from file:**

```bash
# Set environment variable
export SF_KEY_PATH=./signing_key.bin

# Run with key
cargo run --example securefabric-demo -- --key-path signing_key.bin
```

**Add to .gitignore:**

```gitignore
*.bin
signing_key*
private_key*
```

## SDK-Specific Examples

### Rust SDK

```rust
use securefabric_sdk::{Client, ClientConfig, SigningKey};
use std::fs;

// Option 1: Generate new key
let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);

// Option 2: Load from file
let key_bytes = fs::read("signing_key.bin")?;
let signing_key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());

// Option 3: Load from environment variable (base64)
let key_b64 = std::env::var("SF_SIGNING_KEY")?;
let key_bytes = base64::decode(&key_b64)?;
let signing_key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());

let config = ClientConfig {
    endpoint: "https://api.securefabric.io:50051".to_string(),
    bearer_token: std::env::var("SF_TOKEN")?,
    signing_key,
};

let client = Client::connect(config).await?;
```

### JavaScript/TypeScript SDK

```typescript
import { SecureFabricClient } from '@securefabric/sdk';
import * as ed25519 from '@noble/ed25519';
import * as fs from 'fs';

// Option 1: Generate new key
const signingKey = ed25519.utils.randomPrivateKey();

// Option 2: Load from file
const signingKey = new Uint8Array(fs.readFileSync('signing_key.bin'));

// Option 3: Load from environment variable (hex)
const signingKey = new Uint8Array(
  Buffer.from(process.env.SF_SIGNING_KEY!, 'hex')
);

const client = new SecureFabricClient({
  endpoint: 'https://api.securefabric.io:50051',
  bearerToken: process.env.SF_TOKEN!,
  signingKey,
});
```

### Python SDK

```python
from securefabric import Client, ClientConfig
import nacl.signing
import os
from pathlib import Path

# Option 1: Generate new key
signing_key = nacl.signing.SigningKey.generate()

# Option 2: Load from file
key_bytes = Path('signing_key.bin').read_bytes()
signing_key = nacl.signing.SigningKey(key_bytes)

# Option 3: Load from environment variable (hex)
key_hex = os.environ['SF_SIGNING_KEY']
signing_key = nacl.signing.SigningKey(bytes.fromhex(key_hex))

config = ClientConfig(
    endpoint="https://api.securefabric.io:50051",
    bearer_token=os.environ['SF_TOKEN'],
    signing_key=signing_key,
)

client = Client(config)
```

## Production Key Management

For production deployments, integrate with enterprise key management:

### AWS KMS Example (Conceptual)

```python
import boto3

kms = boto3.client('kms')

# Encrypt key at rest
response = kms.encrypt(
    KeyId='alias/securefabric-keys',
    Plaintext=signing_key.encode()
)
encrypted_key = response['CiphertextBlob']

# Decrypt when needed
response = kms.decrypt(CiphertextBlob=encrypted_key)
signing_key = nacl.signing.SigningKey(response['Plaintext'])
```

### HashiCorp Vault Example (Conceptual)

```python
import hvac

client = hvac.Client(url='https://vault.example.com')
client.token = os.environ['VAULT_TOKEN']

# Store key
client.secrets.kv.v2.create_or_update_secret(
    path='securefabric/signing-key',
    secret={'key': signing_key.encode().hex()},
)

# Retrieve key
secret = client.secrets.kv.v2.read_secret_version(path='securefabric/signing-key')
key_hex = secret['data']['data']['key']
signing_key = nacl.signing.SigningKey(bytes.fromhex(key_hex))
```

## Key Rotation

To rotate keys without service disruption:

1. Generate new signing key
1. Update node with both old and new public keys
1. Deploy clients with new signing key
1. Wait for all clients to update
1. Remove old public key from node configuration

## Troubleshooting

### "Invalid signature" errors

- Verify you're using the correct private key
- Ensure key hasn't been corrupted (check file size = 32 bytes)
- Check that public key matches what's registered with the node

### "Key file must be exactly 32 bytes"

- Ed25519 private keys are always 32 bytes
- If loading from hex/base64, decode first
- Check file wasn't corrupted or truncated

### Permission denied errors

```bash
# Fix file permissions
chmod 600 signing_key.bin
```

## Related Documentation

- [SECURITY.md](./SECURITY.md) - Security policy and best practices
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Development guidelines
- [specs/api.md](./specs/api.md) - API reference with signature details
