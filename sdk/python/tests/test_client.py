# SPDX-License-Identifier: Apache-2.0

"""
Basic tests for the SecureFabric Python SDK.

These tests verify that the SDK can be imported and basic functionality works.
They do not require a running SecureFabric node.
"""

import pytest


def test_import():
    """Test that the SDK can be imported"""
    from securefabric import SecureFabricClient

    assert SecureFabricClient is not None


@pytest.mark.asyncio
async def test_client_construction():
    """Test that a client can be constructed"""
    from securefabric import SecureFabricClient

    client = SecureFabricClient("localhost:50051")
    assert client is not None
    await client.close()


@pytest.mark.asyncio
async def test_client_with_tls():
    """Test that a client can be constructed with TLS config"""
    from securefabric import SecureFabricClient

    client = SecureFabricClient(
        "localhost:50051",
        ca_cert=b"fake-ca",
        client_cert=b"fake-cert",
        client_key=b"fake-key",
        bearer_token="test-token",
    )
    assert client is not None
    await client.close()
