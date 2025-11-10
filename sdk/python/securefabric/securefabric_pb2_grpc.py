# SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
# SPDX-License-Identifier: Apache-2.0

"""
Generated gRPC code for SecureFabric.

IMPORTANT: This file is a stub placeholder. For production use, generate the
actual gRPC files by running:

    cd sdk/python && python -m grpc_tools.protoc -I../../specs \
        --python_out=securefabric --grpc_python_out=securefabric \
        --pyi_out=securefabric ../../specs/securefabric.proto

This stub exists to allow the SDK to be imported for development and testing
without requiring protoc to be installed locally.
"""

from typing import Iterator

import grpc

from . import securefabric_pb2


class FabricNodeStub:
    """Stub for FabricNode service"""

    def __init__(self, channel: grpc.Channel):
        """Initialize the stub with a gRPC channel"""
        self.channel = channel
        # In the real generated code, these would be proper gRPC method descriptors
        self.Send = channel.unary_unary(
            "/securefabric.FabricNode/Send",
            request_serializer=lambda x: b"",  # Would be proper serializer
            response_deserializer=lambda x: securefabric_pb2.SendResp(),
        )
        self.Subscribe = channel.unary_stream(
            "/securefabric.FabricNode/Subscribe",
            request_serializer=lambda x: b"",
            response_deserializer=lambda x: securefabric_pb2.Envelope(),
        )
        self.Stats = channel.unary_unary(
            "/securefabric.FabricNode/Stats",
            request_serializer=lambda x: b"",
            response_deserializer=lambda x: securefabric_pb2.StatsResp(),
        )
        self.Join = channel.unary_unary(
            "/securefabric.FabricNode/Join",
            request_serializer=lambda x: b"",
            response_deserializer=lambda x: securefabric_pb2.JoinResp(),
        )
        self.Unjoin = channel.unary_unary(
            "/securefabric.FabricNode/Unjoin",
            request_serializer=lambda x: b"",
            response_deserializer=lambda x: securefabric_pb2.JoinResp(),
        )
