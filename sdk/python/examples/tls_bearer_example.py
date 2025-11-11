# SPDX-License-Identifier: Apache-2.0

import asyncio
from securefabric import SecureFabricClient


async def main():
    # load certs
    with open("certs/ca.crt", "rb") as f:
        ca = f.read()
    with open("certs/client.crt", "rb") as f:
        cert = f.read()
    with open("certs/client.key", "rb") as f:
        key = f.read()

    tls = {"ca_cert": ca, "client_cert": cert, "client_key": key}
    bearer = "s3cr3t-token"
    client = SecureFabricClient("localhost:50051", tls=tls, bearer=bearer)

    ok = await client.send(b"topic1", b"node-B", b"hello world")
    print("send ok", ok)

    async for env in client.subscribe(b"topic1"):
        print("received", env)


if __name__ == "__main__":
    asyncio.run(main())
