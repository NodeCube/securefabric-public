# securefabric-js

A WebAssembly wrapper around `securefabric-core` providing simple `encrypt`/`decrypt` helpers and a stub `send_message` function for use in browser apps.

Build with wasm-pack:

```
cd securefabric-js
wasm-pack build --target web --out-dir pkg
```

Then copy `pkg` into your web app and import `securefabric_js`.

The `example-next` directory contains a minimal Next.js page showing encryption/decryption demo.
