// SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
// SPDX-License-Identifier: Apache-2.0

import { SecureFabricClient } from '@securefabric/sdk';

async function main() {
  const endpoint = process.env.SF_ENDPOINT || 'https://localhost:50051';
  const token = process.env.SF_TOKEN;

  if (!token) {
    console.error('Error: SF_TOKEN environment variable not set');
    process.exit(1);
  }

  console.log('SecureFabric JavaScript SDK Demo');
  console.log('==================================');
  console.log(`Endpoint: ${endpoint}`);
  console.log();

  const client = new SecureFabricClient({
    endpoint,
    bearerToken: token,
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
