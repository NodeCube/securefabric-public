// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use securefabric_sdk::{crypto::Keypair, Client};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "securefabric-demo")]
#[command(about = "SecureFabric Rust SDK Demo", long_about = None)]
struct Args {
    /// SecureFabric node endpoint
    #[arg(long, default_value = "localhost:50051")]
    endpoint: String,

    /// Bearer token for authentication
    #[arg(long)]
    token: String,

    /// Topic to send/receive messages
    #[arg(long, default_value = "demo.messages")]
    topic: String,

    /// Mode: send or subscribe
    #[arg(long, default_value = "send")]
    mode: String,

    /// Message to send (only for send mode)
    #[arg(long, default_value = "Hello from Rust!")]
    message: String,

    /// Path to Ed25519 private key file (32 bytes hex). If not provided, generates a new key.
    #[arg(long)]
    key_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("SecureFabric Rust SDK Demo");
    println!("==========================");
    println!("Endpoint: {}", args.endpoint);
    println!("Topic: {}", args.topic);

    // Load or generate signing key
    let keypair = if let Some(key_path) = &args.key_path {
        // Load from file (hex format)
        let hex = fs::read_to_string(key_path)?.trim().to_string();
        Keypair::from_hex(&hex)?
    } else {
        // Generate new key
        println!("No key provided, generating new Ed25519 keypair...");
        Keypair::generate()
    };

    println!("Public key: {}", keypair.verifying_key_hex());
    println!();

    let mut client = Client::new(&args.endpoint)
        .await?
        .with_signing_key(keypair.signing_key)
        .with_bearer(&args.token);

    match args.mode.as_str() {
        "send" => {
            println!("Sending message: {}", args.message);
            let msg_id = client.send(&args.topic, args.message.as_bytes()).await?;
            println!("✓ Message sent successfully!");
            println!("  Message ID: {}", msg_id);
        }
        "subscribe" => {
            println!("Subscribing to topic: {}", args.topic);
            let mut stream = client.subscribe(args.topic.as_bytes()).await?;

            println!("Waiting for messages (Ctrl+C to exit)...");
            println!();

            use tokio_stream::StreamExt;
            while let Some(envelope) = stream.next().await {
                match envelope {
                    Ok(env) => {
                        let payload = String::from_utf8_lossy(&env.payload);
                        println!("📨 Received message:");
                        println!("  Topic: {}", env.topic);
                        println!("  Message ID: {}", env.msg_id);
                        println!("  Sequence: {}", env.seq);
                        println!("  Payload: {}", payload);

                        // Verify signature
                        if let Ok(valid) = client.verify(&env) {
                            println!("  Signature valid: {}", valid);
                        }

                        // Verify message ID
                        println!("  Message ID valid: {}", client.verify_msg_id(&env));
                        println!();
                    }
                    Err(e) => {
                        eprintln!("Error receiving message: {}", e);
                    }
                }
            }
        }
        _ => {
            eprintln!("Invalid mode: {}. Use 'send' or 'subscribe'", args.mode);
            std::process::exit(1);
        }
    }

    Ok(())
}
