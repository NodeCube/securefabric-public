// SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
// SPDX-License-Identifier: Apache-2.0

/**
 * SecureFabric JavaScript/TypeScript SDK
 *
 * @example
 * ```typescript
 * import { SecureFabricClient } from '@securefabric/sdk';
 *
 * const client = new SecureFabricClient({
 *   endpoint: 'https://api.securefabric.io:50051',
 *   bearerToken: process.env.SF_TOKEN!
 * });
 *
 * await client.send('my-topic', Buffer.from('Hello, SecureFabric!'));
 * ```
 */

import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import * as path from 'path';

export interface ClientConfig {
  endpoint: string;
  bearerToken: string;
}

export interface Envelope {
  pubkey: Buffer;
  sig: Buffer;
  nonce: Buffer;
  aad: Buffer;
  payload: Buffer;
  seq: number;
  msgId: string;
  keyVersion: number;
  topic: string;
}

export class SecureFabricClient {
  private client: any;
  private bearerToken: string;

  constructor(config: ClientConfig) {
    this.bearerToken = config.bearerToken;

    // Load proto file
    const PROTO_PATH = path.join(__dirname, '../../../specs/securefabric.proto');
    const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
      keepCase: true,
      longs: String,
      enums: String,
      defaults: true,
      oneofs: true,
    });

    const protoDescriptor = grpc.loadPackageDefinition(packageDefinition) as any;
    const FabricNode = protoDescriptor.securefabric.FabricNode;

    // Create gRPC client
    const credentials = grpc.credentials.createSsl();
    this.client = new FabricNode(config.endpoint, credentials);
  }

  /**
   * Send a message to a topic
   */
  async send(topic: string, payload: Buffer): Promise<string> {
    const envelope = this.buildEnvelope(topic, payload);

    const metadata = new grpc.Metadata();
    metadata.add('authorization', `Bearer ${this.bearerToken}`);

    return new Promise((resolve, reject) => {
      this.client.Send(
        { envelope },
        metadata,
        (error: grpc.ServiceError | null, response: any) => {
          if (error) {
            reject(error);
          } else {
            resolve(envelope.msgId);
          }
        }
      );
    });
  }

  /**
   * Subscribe to messages on a topic
   */
  subscribe(topic: string, callback: (envelope: Envelope) => void): void {
    const metadata = new grpc.Metadata();
    metadata.add('authorization', `Bearer ${this.bearerToken}`);

    const call = this.client.Subscribe({ topic: Buffer.from(topic) }, metadata);

    call.on('data', (envelope: any) => {
      callback({
        pubkey: envelope.pubkey,
        sig: envelope.sig,
        nonce: envelope.nonce,
        aad: envelope.aad,
        payload: envelope.payload,
        seq: envelope.seq,
        msgId: envelope.msg_id,
        keyVersion: envelope.key_version,
        topic: envelope.topic,
      });
    });

    call.on('error', (error: Error) => {
      console.error('Subscription error:', error);
    });
  }

  /**
   * Get node statistics
   */
  async stats(): Promise<any> {
    const metadata = new grpc.Metadata();
    metadata.add('authorization', `Bearer ${this.bearerToken}`);

    return new Promise((resolve, reject) => {
      this.client.Stats({}, metadata, (error: grpc.ServiceError | null, response: any) => {
        if (error) {
          reject(error);
        } else {
          resolve(response);
        }
      });
    });
  }

  private buildEnvelope(topic: string, payload: Buffer): any {
    // Simplified envelope for demo
    // TODO: Implement proper signing and encryption
    const nonce = Buffer.from(crypto.getRandomValues(new Uint8Array(24)));
    const seq = 1;
    const msgId = this.computeMsgId(Buffer.alloc(32), seq, nonce);

    return {
      pubkey: Buffer.alloc(32),
      sig: Buffer.alloc(64),
      nonce,
      aad: Buffer.alloc(0),
      payload,
      seq,
      msg_id: msgId,
      key_version: 0,
      topic,
    };
  }

  private computeMsgId(pubkey: Buffer, seq: number, nonce: Buffer): string {
    // TODO: Implement proper BLAKE3 hashing
    return Buffer.concat([pubkey, Buffer.from(seq.toString()), nonce])
      .toString('hex')
      .substring(0, 32);
  }
}

export default SecureFabricClient;
