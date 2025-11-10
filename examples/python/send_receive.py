#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
# SPDX-License-Identifier: Apache-2.0

"""
SecureFabric Python SDK Example

Usage:
    python send_receive.py send
    python send_receive.py subscribe
    python send_receive.py stats

Environment variables:
    SF_ENDPOINT: SecureFabric node endpoint (default: https://localhost:50051)
    SF_TOKEN: Bearer token for authentication (required)
    SF_KEY_PATH: Path to Ed25519 private key file (32 bytes, optional)
"""

import os
import sys
from pathlib import Path

import nacl.signing

# Add SDK to path for local development
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "sdk" / "python"))

from securefabric import Client, ClientConfig


def main() -> None:
    """Main entry point"""
    endpoint = os.getenv("SF_ENDPOINT", "https://localhost:50051")
    token = os.getenv("SF_TOKEN")
    key_path = os.getenv("SF_KEY_PATH")

    if not token:
        print("Error: SF_TOKEN environment variable not set", file=sys.stderr)
        sys.exit(1)

    print("SecureFabric Python SDK Demo")
    print("============================")
    print(f"Endpoint: {endpoint}")

    # Load or generate signing key
    if key_path:
        # Load from file
        key_bytes = Path(key_path).read_bytes()
        if len(key_bytes) != 32:
            print("Error: Key file must contain exactly 32 bytes", file=sys.stderr)
            sys.exit(1)
        signing_key = nacl.signing.SigningKey(key_bytes)
        print("Loaded signing key from file")
    else:
        # Generate new key
        print("No key provided, generating new Ed25519 keypair...")
        signing_key = nacl.signing.SigningKey.generate()

    pubkey_hex = signing_key.verify_key.encode().hex()
    print(f"Public key: {pubkey_hex}")
    print()

    config = ClientConfig(
        endpoint=endpoint,
        bearer_token=token,
        signing_key=signing_key,
    )

    mode = sys.argv[1] if len(sys.argv) > 1 else "send"

    with Client(config) as client:
        if mode == "send":
            topic = "demo.messages"
            message = "Hello from Python!"

            print(f"Sending message to topic: {topic}")
            print(f"Message: {message}")

            msg_id = client.send(topic, message.encode("utf-8"))
            print("✓ Message sent successfully!")
            print(f"  Message ID: {msg_id}")

        elif mode == "subscribe":
            topic = "demo.messages"
            print(f"Subscribing to topic: {topic}")
            print("Waiting for messages (Ctrl+C to exit)...")
            print()

            try:
                for envelope in client.subscribe(topic):
                    payload = envelope.payload.decode("utf-8")
                    print("📨 Received message:")
                    print(f"  Topic: {envelope.topic}")
                    print(f"  Message ID: {envelope.msg_id}")
                    print(f"  Payload: {payload}")
                    print()
            except KeyboardInterrupt:
                print("\nSubscription stopped")

        elif mode == "stats":
            print("Fetching node statistics...")
            stats = client.stats()
            print("Node Statistics:")
            print(f"  Peers: {stats.peers}")
            print(f"  P95 Latency: {stats.p95_latency_ms:.2f}ms")
            print(f"  Version: {stats.version}")
            print(f"  Git SHA: {stats.git_sha}")
            print(f"  Built: {stats.built}")
            print(f"  Rust: {stats.rustc}")

        else:
            print(f"Invalid mode: {mode}. Use 'send', 'subscribe', or 'stats'", file=sys.stderr)
            sys.exit(1)


if __name__ == "__main__":
    main()
