use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::Value;

/// Nextgen CLI - control Gitdigital ecosystem from your terminal
#[derive(Parser)]
#[command(name = "nextgen")]
#[command(version = "0.1.0")]
#[command(about = "Your all-in-one Gitdigital CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ping the system
    Ping,
    /// Fetch system status
    Status,
    /// Call API service
    Api { endpoint: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ping => {
            println!("✅ Pong! Nextgen CLI is live.");
        }
        Commands::Status => {
            println!("📊 System status: all green (for now).");
        }
        Commands::Api { endpoint } => {
            let client = Client::new();
            let url = format!("http://localhost:8080/{}", endpoint);
            match client.get(&url).send().await {
                Ok(resp) => {
                    let json: Value = resp.json().await.unwrap_or_else(|_| Value::Null);
                    println!("API Response: {}", json);
                }
                Err(e) => println!("❌ API Error: {e}"),
            }
        }
    }
              }
