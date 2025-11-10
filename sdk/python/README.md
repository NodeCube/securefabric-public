SecureFabric Python SDK

Usage

1. Generate gRPC stubs from `proto/securefabric.proto` into this package
   namespace (recommended location: `sdk/python/securefabric/securefabric_pb2.py`
   and `_pb2_grpc.py`).

python -m grpc_tools.protoc -I proto --python_out=sdk/python/securefabric --grpc_python_out=sdk/python/securefabric proto/securefabric.proto

1. Install package

pip install -e sdk/python/securefabric

1. Example

```py
import asyncio
from securefabric import SecureFabricClient

async def main():
 client = SecureFabricClient("127.0.0.1:50051")
 ok = await client.send(b"topic", b"node-id", b"hello")
 async for msg in client.subscribe(b"topic"):
 print(msg)

asyncio.run(main())
```
