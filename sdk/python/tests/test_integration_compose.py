# SPDX-License-Identifier: Apache-2.0

import os
import asyncio
import subprocess
import time
import requests
import pytest
import shutil
from securefabric import SecureFabricClient

COMPOSE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))


@pytest.mark.skipif(
    shutil.which("docker-compose") is None, reason="docker-compose not available"
)
@pytest.mark.asyncio
async def test_sdk_against_compose():
    # Start docker-compose in background
    p = subprocess.Popen(["docker-compose", "up", "--build", "-d"], cwd=COMPOSE_DIR)
    try:
        # wait for container to be ready (check metrics endpoint)
        ready = False
        for _ in range(30):
            try:
                r = requests.get("http://localhost:9090/metrics", timeout=1.0)
                if r.status_code == 200:
                    ready = True
                    break
            except Exception:
                time.sleep(1)
        assert ready, "compose services not ready"

        # Use SDK to send a message and subscribe
        client = SecureFabricClient("localhost:50051")
        ok = await client.send(b"test-topic", b"node-1", b"hello-from-test")
        assert ok
        await client.close()
    finally:
        subprocess.call(["docker-compose", "down"], cwd=COMPOSE_DIR)
