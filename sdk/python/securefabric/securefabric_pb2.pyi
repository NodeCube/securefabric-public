from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Envelope(_message.Message):
    __slots__ = (
        "pubkey",
        "sig",
        "nonce",
        "aad",
        "payload",
        "seq",
        "msg_id",
        "key_version",
        "topic",
    )
    PUBKEY_FIELD_NUMBER: _ClassVar[int]
    SIG_FIELD_NUMBER: _ClassVar[int]
    NONCE_FIELD_NUMBER: _ClassVar[int]
    AAD_FIELD_NUMBER: _ClassVar[int]
    PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    SEQ_FIELD_NUMBER: _ClassVar[int]
    MSG_ID_FIELD_NUMBER: _ClassVar[int]
    KEY_VERSION_FIELD_NUMBER: _ClassVar[int]
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    pubkey: bytes
    sig: bytes
    nonce: bytes
    aad: bytes
    payload: bytes
    seq: int
    msg_id: str
    key_version: int
    topic: str
    def __init__(
        self,
        pubkey: _Optional[bytes] = ...,
        sig: _Optional[bytes] = ...,
        nonce: _Optional[bytes] = ...,
        aad: _Optional[bytes] = ...,
        payload: _Optional[bytes] = ...,
        seq: _Optional[int] = ...,
        msg_id: _Optional[str] = ...,
        key_version: _Optional[int] = ...,
        topic: _Optional[str] = ...,
    ) -> None: ...

class SendReq(_message.Message):
    __slots__ = ("envelope",)
    ENVELOPE_FIELD_NUMBER: _ClassVar[int]
    envelope: Envelope
    def __init__(
        self, envelope: _Optional[_Union[Envelope, _Mapping]] = ...
    ) -> None: ...

class SendResp(_message.Message):
    __slots__ = ("ok", "msg_id")
    OK_FIELD_NUMBER: _ClassVar[int]
    MSG_ID_FIELD_NUMBER: _ClassVar[int]
    ok: bool
    msg_id: str
    def __init__(self, ok: bool = ..., msg_id: _Optional[str] = ...) -> None: ...

class SubscribeReq(_message.Message):
    __slots__ = ("topic",)
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    topic: bytes
    def __init__(self, topic: _Optional[bytes] = ...) -> None: ...

class StatsReq(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class StatsResp(_message.Message):
    __slots__ = ("peers", "p95_latency_ms", "version", "git_sha", "built", "rustc")
    PEERS_FIELD_NUMBER: _ClassVar[int]
    P95_LATENCY_MS_FIELD_NUMBER: _ClassVar[int]
    VERSION_FIELD_NUMBER: _ClassVar[int]
    GIT_SHA_FIELD_NUMBER: _ClassVar[int]
    BUILT_FIELD_NUMBER: _ClassVar[int]
    RUSTC_FIELD_NUMBER: _ClassVar[int]
    peers: int
    p95_latency_ms: float
    version: str
    git_sha: str
    built: str
    rustc: str
    def __init__(
        self,
        peers: _Optional[int] = ...,
        p95_latency_ms: _Optional[float] = ...,
        version: _Optional[str] = ...,
        git_sha: _Optional[str] = ...,
        built: _Optional[str] = ...,
        rustc: _Optional[str] = ...,
    ) -> None: ...

class NodeInfo(_message.Message):
    __slots__ = ("node_id", "addr", "pubkey")
    NODE_ID_FIELD_NUMBER: _ClassVar[int]
    ADDR_FIELD_NUMBER: _ClassVar[int]
    PUBKEY_FIELD_NUMBER: _ClassVar[int]
    node_id: str
    addr: str
    pubkey: bytes
    def __init__(
        self,
        node_id: _Optional[str] = ...,
        addr: _Optional[str] = ...,
        pubkey: _Optional[bytes] = ...,
    ) -> None: ...

class JoinResp(_message.Message):
    __slots__ = ("ok",)
    OK_FIELD_NUMBER: _ClassVar[int]
    ok: bool
    def __init__(self, ok: bool = ...) -> None: ...

class NodeId(_message.Message):
    __slots__ = ("node_id",)
    NODE_ID_FIELD_NUMBER: _ClassVar[int]
    node_id: str
    def __init__(self, node_id: _Optional[str] = ...) -> None: ...
