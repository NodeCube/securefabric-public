// SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later
//! SecureFabric Rust SDK
//!
//! Provides high-level client API for publishing and subscribing to SecureFabric nodes.

use anyhow::{Context, Result};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use std::path::Path;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tonic::{Request, Streaming};

pub mod pb {
    tonic::include_proto!("securefabric");
}

use pb::fabric_node_client::FabricNodeClient;
use pb::{Envelope, SendReq, SubscribeReq};

/// High-level publisher client
pub struct Publisher {
    client: FabricNodeClient<Channel>,
    signing_key: Option<SigningKey>,
    bearer: Option<String>,
}

impl Publisher {
    /// Create a new Publisher connected to the given endpoint
    pub async fn new(endpoint: impl AsRef<str>) -> Result<Self> {
        let channel = Channel::from_shared(endpoint.as_ref().to_string())?
            .connect()
            .await
            .context("connect to endpoint")?;

        Ok(Self {
            client: FabricNodeClient::new(channel),
            signing_key: None,
            bearer: None,
        })
    }

    /// Create a Publisher with mTLS
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
            client: FabricNodeClient::new(channel),
            signing_key: None,
            bearer: None,
        })
    }

    /// Set signing key for message signatures
    pub fn with_signing_key(mut self, key: SigningKey) -> Self {
        self.signing_key = Some(key);
        self
    }

    /// Set bearer token for authentication
    pub fn with_bearer(mut self, token: impl Into<String>) -> Self {
        self.bearer = Some(token.into());
        self
    }

    /// Publish a message to a topic
    pub async fn send(
        &mut self,
        topic: impl AsRef<[u8]>,
        to: impl AsRef<[u8]>,
        payload: impl AsRef<[u8]>,
    ) -> Result<()> {
        let mut req = Request::new(SendReq {
            topic: topic.as_ref().to_vec(),
            to: to.as_ref().to_vec(),
            payload: payload.as_ref().to_vec(),
        });

        if let Some(bearer) = &self.bearer {
            req.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", bearer).parse().unwrap(),
            );
        }

        self.client.send(req).await.context("send message")?;
        Ok(())
    }
}

/// High-level subscriber client
pub struct Subscriber {
    client: FabricNodeClient<Channel>,
    verifying_key: Option<VerifyingKey>,
    bearer: Option<String>,
}

impl Subscriber {
    /// Create a new Subscriber connected to the given endpoint
    pub async fn new(endpoint: impl AsRef<str>) -> Result<Self> {
        let channel = Channel::from_shared(endpoint.as_ref().to_string())?
            .connect()
            .await
            .context("connect to endpoint")?;

        Ok(Self {
            client: FabricNodeClient::new(channel),
            verifying_key: None,
            bearer: None,
        })
    }

    /// Create a Subscriber with mTLS
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
            client: FabricNodeClient::new(channel),
            verifying_key: None,
            bearer: None,
        })
    }

    /// Set verifying key for signature verification
    pub fn with_verifying_key(mut self, key: VerifyingKey) -> Self {
        self.verifying_key = Some(key);
        self
    }

    /// Set bearer token for authentication
    pub fn with_bearer(mut self, token: impl Into<String>) -> Self {
        self.bearer = Some(token.into());
        self
    }

    /// Subscribe to messages matching a topic pattern
    pub async fn subscribe(&mut self, topic: impl AsRef<[u8]>) -> Result<Streaming<Envelope>> {
        let mut req = Request::new(SubscribeReq {
            topic: topic.as_ref().to_vec(),
        });

        if let Some(bearer) = &self.bearer {
            req.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", bearer).parse().unwrap(),
            );
        }

        let stream = self
            .client
            .subscribe(req)
            .await
            .context("subscribe to topic")?
            .into_inner();

        Ok(stream)
    }

    /// Verify an envelope's signature
    pub fn verify(&self, envelope: &Envelope) -> Result<bool> {
        if envelope.sig.is_empty() {
            return Ok(false);
        }

        let Some(vk) = &self.verifying_key else {
            anyhow::bail!("No verifying key configured");
        };

        let sig = ed25519_dalek::Signature::from_slice(&envelope.sig).context("parse signature")?;

        // Message to verify: topic||0||to||0||payload
        let mut msg = Vec::new();
        msg.extend_from_slice(&envelope.topic);
        msg.push(0);
        msg.extend_from_slice(&envelope.to);
        msg.push(0);
        msg.extend_from_slice(&envelope.payload);

        Ok(vk.verify_strict(&msg, &sig).is_ok())
    }
}

/// Crypto helpers
pub mod crypto {
    use super::*;
    use ed25519_dalek::SecretKey;
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

        /// Load keypair from file
        pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
            let bytes = std::fs::read(path).context("read key file")?;
            if bytes.len() != 32 {
                anyhow::bail!("Invalid key file: expected 32 bytes, got {}", bytes.len());
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes);
            Ok(Self::from_bytes(&arr))
        }
    }

    /// Sign a message
    pub fn sign(signing_key: &SigningKey, message: &[u8]) -> [u8; 64] {
        signing_key.sign(message).to_bytes()
    }

    /// Verify a signature
    pub fn verify(verifying_key: &VerifyingKey, message: &[u8], signature: &[u8]) -> Result<()> {
        let sig = ed25519_dalek::Signature::from_slice(signature)?;
        verifying_key
            .verify_strict(message, &sig)
            .map_err(|e| anyhow::anyhow!("verification failed: {}", e))
    }
}

/// TLS helpers
pub mod tls {
    use super::*;

    /// Build an mTLS channel
    pub async fn mtls_channel(
        endpoint: impl AsRef<str>,
        cert_pem: impl AsRef<[u8]>,
        key_pem: impl AsRef<[u8]>,
        ca_pem: impl AsRef<[u8]>,
    ) -> Result<Channel> {
        let identity = Identity::from_pem(cert_pem.as_ref(), key_pem.as_ref());
        let ca_cert = Certificate::from_pem(ca_pem.as_ref());

        let tls = ClientTlsConfig::new()
            .identity(identity)
            .ca_certificate(ca_cert);

        Channel::from_shared(endpoint.as_ref().to_string())?
            .tls_config(tls)?
            .connect()
            .await
            .context("connect with TLS")
    }
}

/// Authentication helpers
pub mod auth {
    use super::*;

    /// Create a bearer token interceptor
    pub fn bearer_interceptor(
        token: impl Into<String>,
    ) -> impl Fn(Request<()>) -> Result<Request<()>, tonic::Status> + Clone {
        let token = token.into();
        move |mut req: Request<()>| {
            req.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
            Ok(req)
        }
    }
}
