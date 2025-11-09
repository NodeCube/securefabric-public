// SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use securefabric_sdk::{Client, ClientConfig};

#[derive(Parser)]
#[command(name = "securefabric-demo")]
#[command(about = "SecureFabric Rust SDK Demo", long_about = None)]
struct Args {
    /// SecureFabric node endpoint
    #[arg(long, env = "SF_ENDPOINT", default_value = "https://localhost:50051")]
    endpoint: String,

    /// Bearer token for authentication
    #[arg(long, env = "SF_TOKEN")]
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("SecureFabric Rust SDK Demo");
    println!("==========================");
    println!("Endpoint: {}", args.endpoint);
    println!("Topic: {}", args.topic);
    println!();

    let config = ClientConfig {
        endpoint: args.endpoint,
        bearer_token: args.token,
    };

    let client = Client::connect(config).await?;

    match args.mode.as_str() {
        "send" => {
            println!("Sending message: {}", args.message);
            let msg_id = client.send(&args.topic, args.message.as_bytes()).await?;
            println!("✓ Message sent successfully!");
            println!("  Message ID: {}", msg_id);
        }
        "subscribe" => {
            println!("Subscribing to topic: {}", args.topic);
            let mut stream = client.subscribe(&args.topic).await?;

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
                        println!("  Payload: {}", payload);
                        println!();
                    }
                    Err(e) => {
                        eprintln!("Error receiving message: {}", e);
                    }
                }
            }
        }
        "stats" => {
            println!("Fetching node statistics...");
            let stats = client.stats().await?;
            println!("Node Statistics:");
            println!("  Peers: {}", stats.peers);
            println!("  P95 Latency: {:.2}ms", stats.p95_latency_ms);
            println!("  Version: {}", stats.version);
            println!("  Git SHA: {}", stats.git_sha);
            println!("  Built: {}", stats.built);
            println!("  Rust: {}", stats.rustc);
        }
        _ => {
            eprintln!("Invalid mode: {}. Use 'send', 'subscribe', or 'stats'", args.mode);
            std::process::exit(1);
        }
    }

    Ok(())
}
