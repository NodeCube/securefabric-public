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
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ClientConfig {
//!         endpoint: "https://api.securefabric.io:50051".to_string(),
//!         bearer_token: std::env::var("SF_TOKEN")?,
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
}

pub type Result<T> = std::result::Result<T, SdkError>;

/// Configuration for SecureFabric client
#[derive(Clone, Debug)]
pub struct ClientConfig {
    /// gRPC endpoint (e.g., "https://api.securefabric.io:50051")
    pub endpoint: String,

    /// Bearer token for authentication
    pub bearer_token: String,
}

/// SecureFabric client
pub struct Client {
    inner: FabricNodeClient<Channel>,
    bearer_token: String,
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

        Ok(Self {
            inner,
            bearer_token: config.bearer_token,
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig {
    /// #     endpoint: "https://api.securefabric.io:50051".to_string(),
    /// #     bearer_token: "token".to_string(),
    /// # };
    /// # let client = Client::connect(config).await?;
    /// client.send("notifications.alerts", b"Server is down!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, topic: &str, payload: &[u8]) -> Result<String> {
        let envelope = self.build_envelope(topic, payload)?;

        let mut request = Request::new(SendReq {
            envelope: Some(envelope.clone()),
        });

        self.add_auth_header(&mut request);

        let response = self.inner.clone().send(request).await?;

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

    fn build_envelope(&self, topic: &str, payload: &[u8]) -> Result<Envelope> {
        // For simplicity, this example creates a minimal envelope
        // In production, you should:
        // 1. Generate proper Ed25519 keypair
        // 2. Sign the message
        // 3. Use unique nonces
        // 4. Implement sequence numbers

        let nonce = self.generate_nonce();
        let seq = 1; // TODO: implement proper sequence tracking
        let msg_id = self.compute_msg_id(&[], seq, &nonce);

        Ok(Envelope {
            pubkey: vec![0u8; 32], // TODO: real public key
            sig: vec![0u8; 64],    // TODO: real signature
            nonce: nonce.to_vec(),
            aad: vec![],           // TODO: serialize AAD
            payload: payload.to_vec(),
            seq,
            msg_id,
            key_version: 0,
            topic: topic.to_string(),
        })
    }

    fn generate_nonce(&self) -> [u8; 24] {
        // TODO: Use cryptographically secure RNG
        let mut nonce = [0u8; 24];
        getrandom::getrandom(&mut nonce).expect("Failed to generate nonce");
        nonce
    }

    fn compute_msg_id(&self, pubkey: &[u8], seq: u64, nonce: &[u8]) -> String {
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
}

// Re-export commonly used types
pub use proto::{Envelope, SendResp, StatsResp};
