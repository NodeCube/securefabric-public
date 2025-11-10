import asyncio
from typing import AsyncIterator, Optional
import grpc
from grpc import aio
import os

# Import generated protobuf stubs (assumes you ran python -m grpc_tools.protoc to generate under securefabric_pb2/_pb2_grpc etc.)
# For the example we will refer to them as `pb2` and `pb2_grpc` — you should generate them from proto/securefabric.proto
try:
    from securefabric_pb2 import SendReq, SubscribeReq, Envelope
    from securefabric_pb2_grpc import FabricNodeStub
except Exception:
    # placeholders for type checking if stubs are not generated in this environment
    SendReq = None
    SubscribeReq = None
    Envelope = None
    FabricNodeStub = None


class SecureFabricClient:
    def __init__(
        self, target: str, tls: Optional[dict] = None, bearer: Optional[str] = None
    ):
        """
        Async SecureFabric client.

        target: "host:port"
        tls: Optional dict with keys: ca_cert (bytes), client_cert (bytes), client_key (bytes)
        bearer: optional bearer token string for Authorization header
        """
        self._target = target
        self._tls = tls
        self._bearer = bearer
        self._channel: Optional[aio.Channel] = None
        self._stub: Optional[FabricNodeStub] = None

    async def _build_channel(self):
        if self._channel:
            return
        options = [
            ("grpc.max_receive_message_length", 20 * 1024 * 1024),
            ("grpc.max_send_message_length", 20 * 1024 * 1024),
        ]
        if self._tls:
            # load credentials
            creds = grpc.ssl_channel_credentials(
                root_certificates=self._tls.get("ca_cert"),
                private_key=self._tls.get("client_key"),
                certificate_chain=self._tls.get("client_cert"),
            )
            self._channel = aio.secure_channel(self._target, creds, options=options)
        else:
            self._channel = aio.insecure_channel(self._target, options=options)
        # attach metadata interceptor for bearer
        if self._bearer:
            call_credentials = grpc.metadata_call_credentials(
                lambda context, callback: callback(
                    (("authorization", f"Bearer {self._bearer}"),), None
                )
            )
            # combine with ssl if secure channel
            if self._tls:
                composite = grpc.composite_channel_credentials(creds, call_credentials)
                self._channel = aio.secure_channel(
                    self._target, composite, options=options
                )
        self._stub = FabricNodeStub(self._channel)

    async def close(self):
        if self._channel:
            await self._channel.close()
            self._channel = None
            self._stub = None

    async def send(self, topic: bytes, to: bytes, payload: bytes) -> bool:
        """Send a message to `to` under `topic`"""
        await self._build_channel()
        req = SendReq(to=to, topic=topic, payload=payload)
        resp = await self._stub.Send(req)
        return getattr(resp, "ok", False)

    async def subscribe(self, topic: bytes) -> AsyncIterator[Envelope]:
        """Subscribe to a topic; yields Envelope messages asynchronously."""
        await self._build_channel()
        req = SubscribeReq(topic=topic)
        call = self._stub.Subscribe(req)
        async for env in call:
            yield env

    async def stats(self):
        await self._build_channel()
        # Note: Empty() is not imported, this may fail at runtime
        # TODO: Import Empty from securefabric_pb2
        from google.protobuf.empty_pb2 import Empty

        return await self._stub.Stats(Empty())
