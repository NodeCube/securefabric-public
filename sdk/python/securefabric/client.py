# SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
# SPDX-License-Identifier: Apache-2.0

"""
SecureFabric Python SDK Client
"""

import json
import os
import struct
from typing import Iterator, Optional, Dict, Any

import grpc
import nacl.signing
import blake3
from google.protobuf import empty_pb2

# Import generated protobuf modules
from . import securefabric_pb2
from . import securefabric_pb2_grpc


class Aad:
    """Additional Authenticated Data for message envelopes"""

    def __init__(
        self,
        topic: str,
        key_version: int = 0,
        tenant_id: Optional[str] = None,
        content_type: Optional[str] = None,
    ):
        self.topic = topic
        self.key_version = key_version
        self.tenant_id = tenant_id
        self.content_type = content_type

    def to_dict(self) -> Dict[str, Any]:
        """Convert AAD to dictionary for JSON serialization"""
        result: Dict[str, Any] = {
            "topic": self.topic,
            "key_version": self.key_version,
        }
        if self.tenant_id is not None:
            result["tenant_id"] = self.tenant_id
        if self.content_type is not None:
            result["content_type"] = self.content_type
        return result

    def to_bytes(self) -> bytes:
        """Serialize AAD to JSON bytes"""
        return json.dumps(self.to_dict()).encode("utf-8")


class ClientConfig:
    """Configuration for SecureFabric client"""

    def __init__(
        self,
        endpoint: str,
        bearer_token: str,
        signing_key: nacl.signing.SigningKey,
    ):
        """
        Initialize client configuration.

        Args:
            endpoint: gRPC endpoint (e.g., "https://api.securefabric.io:50051")
            bearer_token: Bearer token for authentication
            signing_key: Ed25519 signing key for message authentication
        """
        self.endpoint = endpoint
        self.bearer_token = bearer_token
        self.signing_key = signing_key


class Client:
    """SecureFabric client for secure messaging"""

    def __init__(self, config: ClientConfig):
        """
        Initialize a SecureFabric client.

        Args:
            config: Client configuration

        Example:
            >>> from securefabric import Client, ClientConfig
            >>> import nacl.signing
            >>>
            >>> signing_key = nacl.signing.SigningKey.generate()
            >>> config = ClientConfig(
            ...     endpoint="https://api.securefabric.io:50051",
            ...     bearer_token="your-token-here",
            ...     signing_key=signing_key
            ... )
            >>> client = Client(config)
        """
        self._bearer_token = config.bearer_token
        self._signing_key = config.signing_key
        self._verify_key = config.signing_key.verify_key
        self._sequence = 1

        # Parse endpoint to determine if TLS is needed
        if config.endpoint.startswith("https://"):
            endpoint = config.endpoint.replace("https://", "")
            credentials = grpc.ssl_channel_credentials()
            self._channel = grpc.secure_channel(endpoint, credentials)
        else:
            endpoint = config.endpoint.replace("http://", "")
            self._channel = grpc.insecure_channel(endpoint)

        self._stub = securefabric_pb2_grpc.FabricNodeStub(self._channel)

    def send(
        self,
        topic: str,
        payload: bytes,
        tenant_id: Optional[str] = None,
        content_type: Optional[str] = None,
    ) -> str:
        """
        Send a message to a topic.

        Args:
            topic: The topic to send to
            payload: The message payload
            tenant_id: Optional tenant identifier
            content_type: Optional content type (e.g., "application/json")

        Returns:
            Message ID

        Example:
            >>> msg_id = client.send("notifications.alerts", b"Server is down!")
        """
        envelope = self._build_envelope(topic, payload, tenant_id, content_type)

        request = securefabric_pb2.SendReq(envelope=envelope)
        metadata = self._get_auth_metadata()

        response = self._stub.Send(request, metadata=metadata)
        return envelope.msg_id

    def subscribe(self, topic: str) -> Iterator[securefabric_pb2.Envelope]:
        """
        Subscribe to messages on a topic.

        Args:
            topic: The topic to subscribe to

        Returns:
            Iterator of message envelopes

        Example:
            >>> for envelope in client.subscribe("notifications.alerts"):
            ...     print(f"Received: {envelope.payload.decode()}")
        """
        request = securefabric_pb2.SubscribeReq(topic=topic.encode("utf-8"))
        metadata = self._get_auth_metadata()

        stream = self._stub.Subscribe(request, metadata=metadata)
        return stream

    def stats(self) -> securefabric_pb2.StatsResp:
        """
        Get node statistics.

        Returns:
            Statistics response

        Example:
            >>> stats = client.stats()
            >>> print(f"Peers: {stats.peers}, Latency: {stats.p95_latency_ms}ms")
        """
        request = securefabric_pb2.StatsReq()
        metadata = self._get_auth_metadata()

        response = self._stub.Stats(request, metadata=metadata)
        return response

    def public_key(self) -> bytes:
        """
        Get the public key for this client.

        Returns:
            32-byte Ed25519 public key
        """
        return bytes(self._verify_key)

    def public_key_hex(self) -> str:
        """
        Get the public key as a hex string.

        Returns:
            Hex-encoded public key
        """
        return self.public_key().hex()

    def close(self) -> None:
        """Close the gRPC channel."""
        self._channel.close()

    def __enter__(self) -> "Client":
        """Context manager entry."""
        return self

    def __exit__(self, *args: Any) -> None:
        """Context manager exit."""
        self.close()

    def _build_envelope(
        self,
        topic: str,
        payload: bytes,
        tenant_id: Optional[str] = None,
        content_type: Optional[str] = None,
    ) -> securefabric_pb2.Envelope:
        """Build a signed message envelope."""
        # Get next sequence number
        seq = self._sequence
        self._sequence += 1

        # Generate unique nonce (24 bytes for XChaCha20)
        nonce = os.urandom(24)

        # Get public key bytes
        pubkey = bytes(self._verify_key)

        # Build AAD (Additional Authenticated Data)
        aad = Aad(
            topic=topic,
            key_version=0,  # No E2E encryption for now
            tenant_id=tenant_id,
            content_type=content_type,
        )
        aad_bytes = aad.to_bytes()

        # Sign: signature = Ed25519(aad || payload)
        message_to_sign = aad_bytes + payload
        signed = self._signing_key.sign(message_to_sign)
        signature = signed.signature  # Extract just the signature bytes

        # Compute message ID: BLAKE3(pubkey || seq || nonce)
        msg_id = self._compute_msg_id(pubkey, seq, nonce)

        return securefabric_pb2.Envelope(
            pubkey=pubkey,
            sig=signature,
            nonce=nonce,
            aad=aad_bytes,
            payload=payload,
            seq=seq,
            msg_id=msg_id,
            key_version=0,
            topic=topic,
        )

    def _compute_msg_id(self, pubkey: bytes, seq: int, nonce: bytes) -> str:
        """
        Compute message ID using BLAKE3.

        Message ID = hex(blake3(pubkey || seq || nonce))
        """
        # Pack sequence number as little-endian 64-bit integer
        seq_bytes = struct.pack("<Q", seq)

        # Compute BLAKE3 hash
        hasher = blake3.blake3()
        hasher.update(pubkey)
        hasher.update(seq_bytes)
        hasher.update(nonce)

        return hasher.hexdigest()

    def _get_auth_metadata(self) -> list[tuple[str, str]]:
        """Get gRPC metadata with authorization header."""
        return [("authorization", f"Bearer {self._bearer_token}")]
