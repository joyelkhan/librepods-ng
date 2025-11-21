use clap::{Parser, Subcommand};
use librepods_core::*;
use std::io;

#[derive(Parser)]
#[command(name = "librepods")]
#[command(about = "Apple AirPods Control Framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan for nearby AirPods devices
    Scan,
    /// Connect to a device
    Connect { id: String },
    /// Disconnect from device
    Disconnect { id: String },
    /// Get device status
    Status { id: String },
    /// Set ANC mode
    Anc { id: String, mode: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan => {
            println!("Scanning for AirPods devices...");
        }
        Commands::Connect { id } => {
            println!("Connecting to device: {}", id);
        }
        Commands::Disconnect { id } => {
            println!("Disconnecting from device: {}", id);
        }
        Commands::Status { id } => {
            println!("Getting status for device: {}", id);
        }
        Commands::Anc { id, mode } => {
            println!("Setting ANC mode to {} for device: {}", mode, id);
        }
    }

    Ok(())
}
