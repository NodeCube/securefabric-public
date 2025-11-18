# SPDX-License-Identifier: Apache-2.0

"""
SecureFabric Python Client SDK

Official Python client library for SecureFabric - a secure, low-latency
messaging fabric designed for verified senders and end-to-end confidentiality.
"""

from .client import SecureFabricClient

__version__ = "0.1.0"
__all__ = ["SecureFabricClient", "__version__"]
