# SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
# SPDX-License-Identifier: Apache-2.0

"""
Tests for SecureFabric Python SDK

TODO: Add comprehensive tests for:
- Client initialization
- Message signing and verification
- Sequence number tracking
- AAD serialization
- Message ID computation (BLAKE3)
- Error handling
"""

import pytest


def test_import():
    """Test that the SDK can be imported"""
    from securefabric import Client, ClientConfig

    assert Client is not None
    assert ClientConfig is not None


def test_aad():
    """Test AAD serialization"""
    from securefabric.client import Aad

    aad = Aad(topic="test.topic", key_version=0)
    aad_dict = aad.to_dict()

    assert aad_dict["topic"] == "test.topic"
    assert aad_dict["key_version"] == 0
    assert "tenant_id" not in aad_dict

    # Test with optional fields
    aad2 = Aad(
        topic="test.topic",
        key_version=1,
        tenant_id="tenant123",
        content_type="application/json"
    )
    aad2_dict = aad2.to_dict()

    assert aad2_dict["tenant_id"] == "tenant123"
    assert aad2_dict["content_type"] == "application/json"
