// SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
// SPDX-License-Identifier: Apache-2.0

/**
 * SecureFabric JavaScript/TypeScript SDK
 *
 * @example
 * ```typescript
 * import { SecureFabricClient } from '@securefabric/sdk';
 * import * as ed25519 from '@noble/ed25519';
 *
 * // Generate or load your signing key
 * const privateKey = ed25519.utils.randomPrivateKey();
 *
 * const client = new SecureFabricClient({
 *   endpoint: 'https://api.securefabric.io:50051',
 *   bearerToken: process.env.SF_TOKEN!,
 *   signingKey: privateKey
 * });
 *
 * await client.send('my-topic', Buffer.from('Hello, SecureFabric!'));
 * ```
 */

import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import * as path from 'path';
import * as ed25519 from '@noble/ed25519';
import { hash } from 'blake3';

export interface Aad {
  topic: string;
  tenant_id?: string;
  content_type?: string;
  key_version: number;
}

export interface ClientConfig {
  endpoint: string;
  bearerToken: string;
  signingKey: Uint8Array; // 32-byte Ed25519 private key
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
  private signingKey: Uint8Array;
  private publicKey: Uint8Array;
  private sequence: number;

  constructor(config: ClientConfig) {
    if (config.signingKey.length !== 32) {
      throw new Error('Signing key must be exactly 32 bytes');
    }

    this.bearerToken = config.bearerToken;
    this.signingKey = config.signingKey;
    this.publicKey = ed25519.getPublicKey(config.signingKey);
    this.sequence = 1;

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
  async send(
    topic: string,
    payload: Buffer,
    options?: { tenantId?: string; contentType?: string }
  ): Promise<string> {
    const envelope = await this.buildEnvelope(
      topic,
      payload,
      options?.tenantId,
      options?.contentType
    );

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
            resolve(envelope.msg_id);
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

  private async buildEnvelope(
    topic: string,
    payload: Buffer,
    tenantId?: string,
    contentType?: string
  ): Promise<any> {
    // Get next sequence number
    const seq = this.sequence++;

    // Generate unique nonce (24 bytes for XChaCha20)
    const nonce = Buffer.from(crypto.getRandomValues(new Uint8Array(24)));

    // Build AAD (Additional Authenticated Data)
    const keyVersion = 0; // No E2E encryption for now
    const aad: Aad = {
      topic,
      key_version: keyVersion,
    };
    if (tenantId) aad.tenant_id = tenantId;
    if (contentType) aad.content_type = contentType;

    const aadBytes = Buffer.from(JSON.stringify(aad));

    // Sign: signature = Ed25519(aad || payload)
    const messageToSign = Buffer.concat([aadBytes, payload]);
    const signature = await ed25519.sign(messageToSign, this.signingKey);

    // Compute message ID: BLAKE3(pubkey || seq || nonce)
    const msgId = this.computeMsgId(Buffer.from(this.publicKey), seq, nonce);

    return {
      pubkey: Buffer.from(this.publicKey),
      sig: Buffer.from(signature),
      nonce,
      aad: aadBytes,
      payload,
      seq,
      msg_id: msgId,
      key_version: keyVersion,
      topic,
    };
  }

  private computeMsgId(pubkey: Buffer, seq: number, nonce: Buffer): string {
    // Compute message ID using BLAKE3: hex(blake3(pubkey || seq || nonce))
    const seqBytes = Buffer.alloc(8);
    seqBytes.writeBigUInt64LE(BigInt(seq));

    const data = Buffer.concat([pubkey, seqBytes, nonce]);
    const hashBytes = hash(data);
    return Buffer.from(hashBytes).toString('hex');
  }

  /**
   * Get the public key for this client
   */
  getPublicKey(): Uint8Array {
    return this.publicKey;
  }

  /**
   * Get the public key as a hex string
   */
  getPublicKeyHex(): string {
    return Buffer.from(this.publicKey).toString('hex');
  }
}

export default SecureFabricClient;
