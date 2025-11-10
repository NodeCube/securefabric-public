// SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
// SPDX-License-Identifier: Apache-2.0

//! SecureFabric Rust SDK
//!
//! This library provides a Rust client for the SecureFabric secure messaging fabric.
//!
//! # Example
//!
//! ```no_run
//! use securefabric_sdk::{Client, ClientConfig};
//! use ed25519_dalek::SigningKey;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Generate or load your Ed25519 signing key
//!     let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
//!
//!     let config = ClientConfig {
//!         endpoint: "https://api.securefabric.io:50051".to_string(),
//!         bearer_token: std::env::var("SF_TOKEN")?,
//!         signing_key,
//!     };
//!
//!     let client = Client::connect(config).await?;
//!
//!     client.send("my-topic", b"Hello, SecureFabric!").await?;
//!
//!     Ok(())
//! }
//! ```

pub mod proto {
    tonic::include_proto!("securefabric");
}

use proto::{fabric_node_client::FabricNodeClient, Envelope, SendReq, SubscribeReq};
use thiserror::Error;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::{Request, Status};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("Connection error: {0}")]
    Connection(#[from] tonic::transport::Error),

    #[error("gRPC error: {0}")]
    Grpc(#[from] Status),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Encoding error: {0}")]
    Encoding(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, SdkError>;

/// Additional Authenticated Data structure for message envelopes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Aad {
    /// Topic for the message
    pub topic: String,
    /// Tenant identifier (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Content type (e.g., "application/json", "text/plain")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// E2E encryption key version (0 if plaintext)
    pub key_version: u32,
}

/// Configuration for SecureFabric client
pub struct ClientConfig {
    /// gRPC endpoint (e.g., "https://api.securefabric.io:50051")
    pub endpoint: String,

    /// Bearer token for authentication
    pub bearer_token: String,

    /// Ed25519 signing key for message authentication
    pub signing_key: SigningKey,
}

/// SecureFabric client
pub struct Client {
    inner: FabricNodeClient<Channel>,
    bearer_token: String,
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    sequence: Arc<AtomicU64>,
}

impl Client {
    /// Connect to a SecureFabric node
    pub async fn connect(config: ClientConfig) -> Result<Self> {
        let tls = ClientTlsConfig::new();

        let channel = Channel::from_shared(config.endpoint.clone())
            .map_err(|e| SdkError::Config(e.to_string()))?
            .tls_config(tls)?
            .connect()
            .await?;

        let inner = FabricNodeClient::new(channel);
        let verifying_key = config.signing_key.verifying_key();

        Ok(Self {
            inner,
            bearer_token: config.bearer_token,
            signing_key: config.signing_key,
            verifying_key,
            sequence: Arc::new(AtomicU64::new(1)),
        })
    }

    /// Send a message to a topic
    ///
    /// # Arguments
    ///
    /// * `topic` - The topic to send to
    /// * `payload` - The message payload
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use securefabric_sdk::{Client, ClientConfig};
    /// # use ed25519_dalek::SigningKey;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let signing_key = SigningKey::from_bytes(&[0u8; 32]);
    /// # let config = ClientConfig {
    /// #     endpoint: "https://api.securefabric.io:50051".to_string(),
    /// #     bearer_token: "token".to_string(),
    /// #     signing_key,
    /// # };
    /// # let client = Client::connect(config).await?;
    /// client.send("notifications.alerts", b"Server is down!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, topic: &str, payload: &[u8]) -> Result<String> {
        let envelope = self.build_envelope(topic, payload, None, None)?;

        let mut request = Request::new(SendReq {
            envelope: Some(envelope.clone()),
        });

        self.add_auth_header(&mut request);

        let _response = self.inner.clone().send(request).await?;

        Ok(envelope.msg_id)
    }

    /// Send a message with additional metadata
    ///
    /// # Arguments
    ///
    /// * `topic` - The topic to send to
    /// * `payload` - The message payload
    /// * `tenant_id` - Optional tenant identifier
    /// * `content_type` - Optional content type (e.g., "application/json")
    pub async fn send_with_metadata(
        &self,
        topic: &str,
        payload: &[u8],
        tenant_id: Option<String>,
        content_type: Option<String>,
    ) -> Result<String> {
        let envelope = self.build_envelope(topic, payload, tenant_id, content_type)?;

        let mut request = Request::new(SendReq {
            envelope: Some(envelope.clone()),
        });

        self.add_auth_header(&mut request);

        let _response = self.inner.clone().send(request).await?;

        Ok(envelope.msg_id)
    }

    /// Subscribe to messages on a topic
    ///
    /// Returns a stream of messages
    pub async fn subscribe(
        &self,
        topic: &str,
    ) -> Result<tonic::Streaming<Envelope>> {
        let mut request = Request::new(SubscribeReq {
            topic: topic.as_bytes().to_vec(),
        });

        self.add_auth_header(&mut request);

        let response = self.inner.clone().subscribe(request).await?;

        Ok(response.into_inner())
    }

    /// Get node statistics
    pub async fn stats(&self) -> Result<proto::StatsResp> {
        let mut request = Request::new(proto::StatsReq {});
        self.add_auth_header(&mut request);

        let response = self.inner.clone().stats(request).await?;

        Ok(response.into_inner())
    }

    fn build_envelope(
        &self,
        topic: &str,
        payload: &[u8],
        tenant_id: Option<String>,
        content_type: Option<String>,
    ) -> Result<Envelope> {
        // Get next sequence number (atomically)
        let seq = self.sequence.fetch_add(1, Ordering::SeqCst);

        // Generate unique nonce
        let nonce = self.generate_nonce();

        // Get public key bytes
        let pubkey = self.verifying_key.to_bytes();

        // Build AAD (Additional Authenticated Data)
        let key_version = 0u32; // No E2E encryption for now
        let aad = Aad {
            topic: topic.to_string(),
            tenant_id,
            content_type,
            key_version,
        };
        let aad_bytes = serde_json::to_vec(&aad)?;

        // Sign: signature = Ed25519(aad || payload)
        let mut message_to_sign = Vec::new();
        message_to_sign.extend_from_slice(&aad_bytes);
        message_to_sign.extend_from_slice(payload);
        let signature = self.signing_key.sign(&message_to_sign);

        // Compute message ID: BLAKE3(pubkey || seq || nonce)
        let msg_id = self.compute_msg_id(&pubkey, seq, &nonce);

        Ok(Envelope {
            pubkey: pubkey.to_vec(),
            sig: signature.to_bytes().to_vec(),
            nonce: nonce.to_vec(),
            aad: aad_bytes,
            payload: payload.to_vec(),
            seq,
            msg_id,
            key_version,
            topic: topic.to_string(),
        })
    }

    fn generate_nonce(&self) -> [u8; 24] {
        // Generate cryptographically secure random nonce
        let mut nonce = [0u8; 24];
        getrandom::getrandom(&mut nonce).expect("Failed to generate nonce");
        nonce
    }

    fn compute_msg_id(&self, pubkey: &[u8], seq: u64, nonce: &[u8]) -> String {
        // Compute message ID using BLAKE3: hex(blake3(pubkey || seq || nonce))
        let mut hasher = blake3::Hasher::new();
        hasher.update(pubkey);
        hasher.update(&seq.to_le_bytes());
        hasher.update(nonce);
        hex::encode(hasher.finalize().as_bytes())
    }

    fn add_auth_header<T>(&self, request: &mut Request<T>) {
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", self.bearer_token)
                .parse()
                .expect("Invalid bearer token"),
        );
    }

    /// Get the public key (verifying key) for this client
    pub fn public_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    /// Get the public key as bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
}

// Re-export commonly used types
pub use proto::{Envelope, SendResp, StatsResp};
pub use ed25519_dalek::{SigningKey, VerifyingKey};
