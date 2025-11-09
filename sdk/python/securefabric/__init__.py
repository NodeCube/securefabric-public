# SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
# SPDX-License-Identifier: Apache-2.0

"""
SecureFabric Python SDK

Example usage:
    from securefabric import Client

    client = Client(
        endpoint="https://api.securefabric.io:50051",
        bearer_token="your-token-here"
    )

    client.send("my-topic", b"Hello, SecureFabric!")
"""

__version__ = "0.1.0"

from .client import Client

__all__ = ["Client", "__version__"]
