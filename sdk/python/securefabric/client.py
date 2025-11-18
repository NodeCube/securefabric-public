# SPDX-License-Identifier: Apache-2.0

"""
SecureFabric Python client implementation.

Provides high-level async API for connecting to SecureFabric nodes with support for:
- TLS/mTLS connections
- Bearer token authentication
- Client certificate authentication
- Message publishing and subscription
"""

import asyncio
from typing import Any, AsyncIterator, Optional, Callable, Union
import grpc
from grpc import aio
import os

# Import generated protobuf stubs
try:
    from .securefabric_pb2 import SendReq, SubscribeReq, Envelope
    from .securefabric_pb2_grpc import FabricNodeStub
except ImportError:
    try:
        from securefabric_pb2 import SendReq, SubscribeReq, Envelope  # type: ignore[no-redef]
        from securefabric_pb2_grpc import FabricNodeStub  # type: ignore[no-redef]
    except Exception:
        # Fallback placeholders for type checking when stubs are not available
        SendReq = SubscribeReq = Envelope = FabricNodeStub = Any  # type: ignore[misc,assignment,no-redef]


class SecureFabricClient:
    """
    High-level async client for SecureFabric messaging.

    Supports TLS/mTLS connections, bearer token auth, and client certificate auth.
    All methods are async and should be awaited.

    Example:
        ```python
        import asyncio
        from securefabric import SecureFabricClient

        async def main():
            # Connect with TLS and bearer token
            client = SecureFabricClient(
                target="node.example.com:50051",
                bearer_token="your-token-here"
            )

            # Publish a message
            await client.publish("sensors/temp", b"22.5C")

            # Subscribe to messages
            async def handle_message(envelope):
                print(f"Received: {envelope.payload}")

            await client.subscribe("sensors/#", handle_message)
            await client.close()

        asyncio.run(main())
        ```
    """

    def __init__(
        self,
        target: str,
        tls: bool = True,
        ca_cert: Optional[bytes] = None,
        client_cert: Optional[bytes] = None,
        client_key: Optional[bytes] = None,
        bearer_token: Optional[str] = None,
        insecure: bool = False,
    ):
        """
        Initialize SecureFabric client.

        Args:
            target: Node address as "host:port" (e.g., "node.example.com:50051")
            tls: Enable TLS (default: True)
            ca_cert: Optional CA certificate in PEM format (bytes)
            client_cert: Optional client certificate for mTLS (bytes)
            client_key: Optional client private key for mTLS (bytes)
            bearer_token: Optional bearer token for Authorization header
            insecure: Allow insecure connections (not recommended for production)
        """
        self._target = target
        self._tls = tls and not insecure
        self._ca_cert = ca_cert
        self._client_cert = client_cert
        self._client_key = client_key
        self._bearer_token = bearer_token
        self._channel: Optional[aio.Channel] = None
        self._stub: Optional[FabricNodeStub] = None

    async def _build_channel(self):
        """Build gRPC channel with appropriate credentials."""
        if self._channel:
            return

        options = [
            ("grpc.max_receive_message_length", 20 * 1024 * 1024),
            ("grpc.max_send_message_length", 20 * 1024 * 1024),
        ]

        if self._tls:
            # Create SSL credentials
            creds = grpc.ssl_channel_credentials(
                root_certificates=self._ca_cert,
                private_key=self._client_key,
                certificate_chain=self._client_cert,
            )

            # Add bearer token if provided
            if self._bearer_token:
                call_credentials = grpc.metadata_call_credentials(
                    lambda context, callback: callback(
                        (("authorization", f"Bearer {self._bearer_token}"),), None
                    )
                )
                composite = grpc.composite_channel_credentials(creds, call_credentials)
                self._channel = aio.secure_channel(
                    self._target, composite, options=options
                )
            else:
                self._channel = aio.secure_channel(self._target, creds, options=options)
        else:
            # Insecure channel (not recommended for production)
            self._channel = aio.insecure_channel(self._target, options=options)

        self._stub = FabricNodeStub(self._channel)

    async def close(self):
        """Close the gRPC channel and cleanup resources."""
        if self._channel:
            await self._channel.close()
            self._channel = None
            self._stub = None

    async def publish(
        self,
        topic: str,
        payload: Union[bytes, str],
        aad: Optional[bytes] = None,
    ) -> bool:
        """
        Publish a message to a topic.

        Args:
            topic: Topic string (e.g., "sensors/temp")
            payload: Message payload as bytes or string
            aad: Optional Additional Authenticated Data

        Returns:
            bool: True if message was accepted by the node

        Raises:
            grpc.RpcError: If the RPC fails
        """
        await self._build_channel()
        assert self._stub is not None, "Stub should be initialized after _build_channel"

        # Convert string payload to bytes
        if isinstance(payload, str):
            payload = payload.encode("utf-8")

        # Convert topic to bytes for protobuf
        topic_bytes = topic.encode("utf-8") if isinstance(topic, str) else topic

        # Build envelope (simplified - production would include signing)
        envelope = Envelope(
            topic=topic,
            payload=payload,
            aad=aad or b"",
        )

        req = SendReq(envelope=envelope)
        resp = await self._stub.Send(req)
        return getattr(resp, "ok", False)

    async def subscribe(
        self,
        topic: str,
        callback: Callable[[Envelope], None],
    ) -> None:
        """
        Subscribe to a topic and invoke callback for each message.

        Args:
            topic: Topic pattern to subscribe to (e.g., "sensors/#")
            callback: Function to call for each received envelope

        The callback receives an Envelope object with fields:
        - payload: Message content (bytes)
        - topic: Message topic (str)
        - pubkey: Sender's public key (bytes)
        - sig: Message signature (bytes)
        - seq: Sequence number (int)
        - msg_id: Message ID (str)

        Note: This method runs indefinitely. Cancel the task to stop.
        """
        await self._build_channel()
        assert self._stub is not None, "Stub should be initialized after _build_channel"

        # Convert topic to bytes for protobuf
        topic_bytes = topic.encode("utf-8") if isinstance(topic, str) else topic
        req = SubscribeReq(topic=topic_bytes)

        call = self._stub.Subscribe(req)
        async for envelope in call:
            try:
                callback(envelope)
            except Exception as e:
                # Log but don't crash the subscription loop
                print(f"Error in subscription callback: {e}")

    async def subscribe_stream(self, topic: str) -> AsyncIterator[Envelope]:
        """
        Subscribe to a topic and yield envelopes as an async iterator.

        Args:
            topic: Topic pattern to subscribe to (e.g., "sensors/#")

        Yields:
            Envelope: Each message received on the topic

        Example:
            ```python
            async for envelope in client.subscribe_stream("sensors/#"):
                print(f"Received: {envelope.payload}")
            ```
        """
        await self._build_channel()
        assert self._stub is not None, "Stub should be initialized after _build_channel"

        topic_bytes = topic.encode("utf-8") if isinstance(topic, str) else topic
        req = SubscribeReq(topic=topic_bytes)
        call = self._stub.Subscribe(req)

        async for envelope in call:
            yield envelope

    async def stats(self):
        """
        Get node statistics and metadata.

        Returns:
            StatsResp: Node statistics including peer count, latency, version info
        """
        await self._build_channel()
        assert self._stub is not None, "Stub should be initialized after _build_channel"

        from google.protobuf.empty_pb2 import Empty

        return await self._stub.Stats(Empty())
