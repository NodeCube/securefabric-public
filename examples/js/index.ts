// SPDX-License-Identifier: Apache-2.0

import { SecureFabricClient } from '@securefabric/sdk';
import * as ed25519 from '@noble/ed25519';
import * as fs from 'fs';

async function main() {
  const endpoint = process.env.SF_ENDPOINT || 'https://localhost:50051';
  const token = process.env.SF_TOKEN;
  const keyPath = process.env.SF_KEY_PATH;

  if (!token) {
    console.error('Error: SF_TOKEN environment variable not set');
    process.exit(1);
  }

  console.log('SecureFabric JavaScript SDK Demo');
  console.log('==================================');
  console.log(`Endpoint: ${endpoint}`);

  // Load or generate signing key
  let signingKey: Uint8Array;
  if (keyPath) {
    // Load from file
    const keyBytes = fs.readFileSync(keyPath);
    if (keyBytes.length !== 32) {
      console.error('Error: Key file must contain exactly 32 bytes');
      process.exit(1);
    }
    signingKey = new Uint8Array(keyBytes);
    console.log('Loaded signing key from file');
  } else {
    // Generate new key
    console.log('No key provided, generating new Ed25519 keypair...');
    signingKey = ed25519.utils.randomPrivateKey();
  }

  const publicKey = await ed25519.getPublicKey(signingKey);
  const pubkeyHex = Buffer.from(publicKey).toString('hex');
  console.log(`Public key: ${pubkeyHex}`);
  console.log();

  const client = new SecureFabricClient({
    endpoint,
    bearerToken: token,
    signingKey,
  });

  const mode = process.argv[2] || 'send';

  if (mode === 'send') {
    const topic = 'demo.messages';
    const message = 'Hello from JavaScript!';

    console.log(`Sending message to topic: ${topic}`);
    console.log(`Message: ${message}`);

    const msgId = await client.send(topic, Buffer.from(message));
    console.log(`✓ Message sent successfully!`);
    console.log(`  Message ID: ${msgId}`);
  } else if (mode === 'subscribe') {
    const topic = 'demo.messages';
    console.log(`Subscribing to topic: ${topic}`);
    console.log('Waiting for messages (Ctrl+C to exit)...');
    console.log();

    client.subscribe(topic, (envelope) => {
      const payload = envelope.payload.toString('utf-8');
      console.log('📨 Received message:');
      console.log(`  Topic: ${envelope.topic}`);
      console.log(`  Message ID: ${envelope.msgId}`);
      console.log(`  Payload: ${payload}`);
      console.log();
    });

    // Keep process alive
    await new Promise(() => {});
  } else if (mode === 'stats') {
    console.log('Fetching node statistics...');
    const stats = await client.stats();
    console.log('Node Statistics:');
    console.log(`  Peers: ${stats.peers}`);
    console.log(`  P95 Latency: ${stats.p95_latency_ms}ms`);
    console.log(`  Version: ${stats.version}`);
    console.log(`  Git SHA: ${stats.git_sha}`);
  } else {
    console.error(`Invalid mode: ${mode}. Use 'send', 'subscribe', or 'stats'`);
    process.exit(1);
  }
}

main().catch((error) => {
  console.error('Error:', error);
  process.exit(1);
});
