# SPDX-License-Identifier: Apache-2.0

#!/usr/bin/env python3

"""
SecureFabric Python SDK Example

Usage:
    python send_receive.py send
    python send_receive.py subscribe
    python send_receive.py stats

Environment variables:
    SF_ENDPOINT: SecureFabric node endpoint (default: localhost:50051)
    SF_TOKEN: Bearer token for authentication (required)
    SF_CA_CERT: Path to CA certificate file (optional, for TLS)
    SF_CLIENT_CERT: Path to client certificate file (optional, for mTLS)
    SF_CLIENT_KEY: Path to client key file (optional, for mTLS)
"""

import asyncio
import os
import sys
from pathlib import Path

# Add SDK to path for local development
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "sdk" / "python"))

from securefabric import SecureFabricClient


async def main() -> None:
    """Main entry point"""
    endpoint = os.getenv("SF_ENDPOINT", "localhost:50051")
    token = os.getenv("SF_TOKEN")
    ca_cert_path = os.getenv("SF_CA_CERT")
    client_cert_path = os.getenv("SF_CLIENT_CERT")
    client_key_path = os.getenv("SF_CLIENT_KEY")

    if not token:
        print("Error: SF_TOKEN environment variable not set", file=sys.stderr)
        sys.exit(1)

    print("SecureFabric Python SDK Demo")
    print("============================")
    print(f"Endpoint: {endpoint}")

    # Load TLS certificates if provided
    tls = None
    if ca_cert_path or client_cert_path or client_key_path:
        tls = {}
        if ca_cert_path:
            tls['ca_cert'] = Path(ca_cert_path).read_bytes()
            print(f"Loaded CA certificate from {ca_cert_path}")
        if client_cert_path:
            tls['client_cert'] = Path(client_cert_path).read_bytes()
            print(f"Loaded client certificate from {client_cert_path}")
        if client_key_path:
            tls['client_key'] = Path(client_key_path).read_bytes()
            print(f"Loaded client key from {client_key_path}")
    print()

    client = SecureFabricClient(endpoint, tls=tls, bearer=token)

    mode = sys.argv[1] if len(sys.argv) > 1 else "send"

    try:
        if mode == "send":
            topic = b"demo.messages"
            to = b"node-B"
            message = b"Hello from Python!"

            print(f"Sending message to topic: {topic.decode('utf-8')}")
            print(f"To: {to.decode('utf-8')}")
            print(f"Message: {message.decode('utf-8')}")

            ok = await client.send(topic, to, message)
            if ok:
                print("✓ Message sent successfully!")
            else:
                print("✗ Message send failed")

        elif mode == "subscribe":
            topic = b"demo.messages"
            print(f"Subscribing to topic: {topic.decode('utf-8')}")
            print("Waiting for messages (Ctrl+C to exit)...")
            print()

            try:
                async for envelope in client.subscribe(topic):
                    payload = envelope.payload.decode("utf-8", errors="replace")
                    print("📨 Received message:")
                    print(f"  From: {envelope.from_node.decode('utf-8', errors='replace')}")
                    print(f"  Topic: {envelope.topic.decode('utf-8', errors='replace')}")
                    print(f"  Message ID: {envelope.msg_id.hex()}")
                    print(f"  Payload: {payload}")
                    print()
            except KeyboardInterrupt:
                print("\nSubscription stopped")

        elif mode == "stats":
            print("Fetching node statistics...")
            stats = await client.stats()
            print("Node Statistics:")
            print(f"  {stats}")

        else:
            print(
                f"Invalid mode: {mode}. Use 'send', 'subscribe', or 'stats'",
                file=sys.stderr
            )
            sys.exit(1)

    finally:
        await client.close()


if __name__ == "__main__":
    asyncio.run(main())
