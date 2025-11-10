// SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later
//! SecureFabric Rust SDK
//!
//! Provides high-level client API for publishing and subscribing to SecureFabric nodes.

use anyhow::{Context, Result};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tonic::{Request, Streaming};

pub mod pb {
    tonic::include_proto!("securefabric");
}

use pb::fabric_node_client::FabricNodeClient;
use pb::{Envelope, SendReq, SubscribeReq};

/// High-level client for SecureFabric
pub struct Client {
    inner: FabricNodeClient<Channel>,
    signing_key: Option<SigningKey>,
    verifying_key: Option<VerifyingKey>,
    bearer: Option<String>,
    sequence: Arc<AtomicU64>,
}

impl Client {
    /// Create a new Client connected to the given endpoint
    pub async fn new(endpoint: impl AsRef<str>) -> Result<Self> {
        let channel = Channel::from_shared(endpoint.as_ref().to_string())?
            .connect()
            .await
            .context("connect to endpoint")?;

        Ok(Self {
            inner: FabricNodeClient::new(channel),
            signing_key: None,
            verifying_key: None,
            bearer: None,
            sequence: Arc::new(AtomicU64::new(1)),
        })
    }

    /// Create a Client with mTLS
    pub async fn with_mtls(
        endpoint: impl AsRef<str>,
        cert_pem: impl AsRef<[u8]>,
        key_pem: impl AsRef<[u8]>,
        ca_pem: impl AsRef<[u8]>,
    ) -> Result<Self> {
        let identity = Identity::from_pem(cert_pem.as_ref(), key_pem.as_ref());
        let ca_cert = Certificate::from_pem(ca_pem.as_ref());

        let tls = ClientTlsConfig::new()
            .identity(identity)
            .ca_certificate(ca_cert);

        let channel = Channel::from_shared(endpoint.as_ref().to_string())?
            .tls_config(tls)?
            .connect()
            .await
            .context("connect with TLS")?;

        Ok(Self {
            inner: FabricNodeClient::new(channel),
            signing_key: None,
            verifying_key: None,
            bearer: None,
            sequence: Arc::new(AtomicU64::new(1)),
        })
    }

    /// Set signing key for message signatures
    pub fn with_signing_key(mut self, key: SigningKey) -> Self {
        self.verifying_key = Some(key.verifying_key());
        self.signing_key = Some(key);
        self
    }

    /// Set bearer token for authentication
    pub fn with_bearer(mut self, token: impl Into<String>) -> Self {
        self.bearer = Some(token.into());
        self
    }

    /// Build an envelope with signature
    fn build_envelope(&self, topic: &str, payload: &[u8]) -> Result<Envelope> {
        let signing_key = self
            .signing_key
            .as_ref()
            .context("No signing key configured")?;
        let verifying_key = self
            .verifying_key
            .as_ref()
            .context("No verifying key configured")?;

        let seq = self.sequence.fetch_add(1, Ordering::SeqCst);
        let nonce = self.generate_nonce();
        let pubkey = verifying_key.to_bytes().to_vec();

        // Build AAD: {"topic":"...","key_version":0}
        let aad = serde_json::json!({
            "topic": topic,
            "key_version": 0u32,
        });
        let aad_bytes = serde_json::to_vec(&aad)?;

        // Sign: signature = Ed25519(aad || payload)
        let mut message_to_sign = Vec::new();
        message_to_sign.extend_from_slice(&aad_bytes);
        message_to_sign.extend_from_slice(payload);
        let signature = signing_key.sign(&message_to_sign);

        // Compute BLAKE3 message ID: hex(blake3(pubkey || seq || nonce))
        let msg_id = self.compute_msg_id(&pubkey, seq, &nonce);

        Ok(Envelope {
            pubkey,
            sig: signature.to_bytes().to_vec(),
            nonce: nonce.to_vec(),
            aad: aad_bytes,
            payload: payload.to_vec(),
            seq,
            msg_id,
            key_version: 0,
            topic: topic.to_string(),
        })
    }

    /// Generate a random 24-byte nonce
    fn generate_nonce(&self) -> Vec<u8> {
        use rand::RngCore;
        let mut nonce = vec![0u8; 24];
        rand::rngs::OsRng.fill_bytes(&mut nonce);
        nonce
    }

    /// Compute BLAKE3 message ID
    fn compute_msg_id(&self, pubkey: &[u8], seq: u64, nonce: &[u8]) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(pubkey);
        hasher.update(&seq.to_le_bytes());
        hasher.update(nonce);
        hasher.finalize().to_hex().to_string()
    }

    /// Send a message
    pub async fn send(&mut self, topic: &str, payload: &[u8]) -> Result<String> {
        let envelope = self.build_envelope(topic, payload)?;
        let msg_id = envelope.msg_id.clone();

        let mut req = Request::new(SendReq {
            envelope: Some(envelope),
        });

        if let Some(bearer) = &self.bearer {
            req.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", bearer).parse().unwrap(),
            );
        }

        self.inner.send(req).await.context("send message")?;
        Ok(msg_id)
    }

    /// Subscribe to messages matching a topic pattern
    pub async fn subscribe(&mut self, topic: &[u8]) -> Result<Streaming<Envelope>> {
        let mut req = Request::new(SubscribeReq {
            topic: topic.to_vec(),
        });

        if let Some(bearer) = &self.bearer {
            req.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", bearer).parse().unwrap(),
            );
        }

        let stream = self
            .inner
            .subscribe(req)
            .await
            .context("subscribe to topic")?
            .into_inner();

        Ok(stream)
    }

    /// Verify an envelope's signature
    pub fn verify(&self, envelope: &Envelope) -> Result<bool> {
        if envelope.sig.is_empty() || envelope.sig.len() != 64 {
            return Ok(false);
        }

        if envelope.pubkey.is_empty() || envelope.pubkey.len() != 32 {
            return Ok(false);
        }

        let vk = VerifyingKey::from_bytes(
            envelope
                .pubkey
                .as_slice()
                .try_into()
                .context("Invalid pubkey length")?,
        )
        .context("Invalid public key")?;

        let sig = ed25519_dalek::Signature::from_slice(&envelope.sig).context("parse signature")?;

        // Verify: signature = Ed25519(aad || payload)
        let mut message = Vec::new();
        message.extend_from_slice(&envelope.aad);
        message.extend_from_slice(&envelope.payload);

        Ok(vk.verify_strict(&message, &sig).is_ok())
    }

    /// Verify message ID
    pub fn verify_msg_id(&self, envelope: &Envelope) -> bool {
        let computed = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(&envelope.pubkey);
            hasher.update(&envelope.seq.to_le_bytes());
            hasher.update(&envelope.nonce);
            hasher.finalize().to_hex().to_string()
        };
        computed == envelope.msg_id
    }
}

/// Crypto helpers
pub mod crypto {
    use super::*;
    use rand::rngs::OsRng;

    /// Ed25519 keypair
    pub struct Keypair {
        pub signing_key: SigningKey,
        pub verifying_key: VerifyingKey,
    }

    impl Keypair {
        /// Generate a new random keypair
        pub fn generate() -> Self {
            use rand::RngCore;
            let mut seed = [0u8; 32];
            OsRng.fill_bytes(&mut seed);
            let signing_key = SigningKey::from_bytes(&seed);
            let verifying_key = signing_key.verifying_key();
            Self {
                signing_key,
                verifying_key,
            }
        }

        /// Load keypair from 32-byte seed
        pub fn from_bytes(bytes: &[u8; 32]) -> Self {
            let signing_key = SigningKey::from_bytes(bytes);
            let verifying_key = signing_key.verifying_key();
            Self {
                signing_key,
                verifying_key,
            }
        }

        /// Load keypair from hex string
        pub fn from_hex(hex: &str) -> Result<Self> {
            let bytes = hex::decode(hex).context("decode hex")?;
            if bytes.len() != 32 {
                anyhow::bail!("Expected 32 bytes, got {}", bytes.len());
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes);
            Ok(Self::from_bytes(&arr))
        }

        /// Export signing key as hex
        pub fn to_hex(&self) -> String {
            hex::encode(self.signing_key.to_bytes())
        }

        /// Export verifying key as hex
        pub fn verifying_key_hex(&self) -> String {
            hex::encode(self.verifying_key.to_bytes())
        }
    }
}
