//! iOSS Emulator CLI
//!
//! Command-line interface for managing and controlling iOS devices

use clap::{Parser, Subcommand};
use ioss_emulator::{Emulator, iOSVersion, Result};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ioss")]
#[command(about = "iOS Simulator - Open-source iOS emulator for NixOS", long_about = None)]
#[command(version = ioss_emulator::VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Home directory for iOSS
    #[arg(long, global = true)]
    home: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage virtual devices
    Device {
        #[command(subcommand)]
        command: DeviceCommand,
    },

    /// Manage apps
    App {
        #[command(subcommand)]
        command: AppCommand,
    },

    /// Manage simulator settings
    Simulator {
        #[command(subcommand)]
        command: SimulatorCommand,
    },
}

#[derive(Subcommand)]
enum DeviceCommand {
    /// Create a new virtual device
    Create {
        /// Device name
        #[arg(short, long)]
        name: String,

        /// iOS version (14 or 15)
        #[arg(short, long, default_value = "15")]
        ios: u32,
    },

    /// List all devices
    List,

    /// Boot a device
    Boot {
        /// Device ID or name
        device: String,
    },

    /// Shutdown a device
    Shutdown {
        /// Device ID or name
        device: String,
    },

    /// Delete a device
    Delete {
        /// Device ID or name
        device: String,
    },

    /// Show device details
    Info {
        /// Device ID or name
        device: String,
    },
}

#[derive(Subcommand)]
enum AppCommand {
    /// Install an app
    Install {
        /// Device ID or name
        #[arg(short, long)]
        device: String,

        /// App bundle path or IPA file
        #[arg(short, long)]
        path: PathBuf,

        /// App ID
        #[arg(short, long)]
        app_id: String,
    },

    /// Launch an app
    Launch {
        /// Device ID or name
        #[arg(short, long)]
        device: String,

        /// App ID
        app_id: String,
    },

    /// List installed apps
    List {
        /// Device ID or name
        device: String,
    },
}

#[derive(Subcommand)]
enum SimulatorCommand {
    /// Show simulator information
    Info,

    /// Check simulator status
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    if cli.verbose {
        eprintln!("Verbose logging enabled");
    }

    let mut emulator = Emulator::new(cli.home)?;

    match cli.command {
        Commands::Device { command } => handle_device_command(&mut emulator, command).await?,
        Commands::App { command } => handle_app_command(&emulator, command).await?,
        Commands::Simulator { command } => handle_simulator_command(command)?,
    }

    Ok(())
}

async fn handle_device_command(
    emulator: &mut Emulator,
    command: DeviceCommand,
) -> Result<()> {
    match command {
        DeviceCommand::Create { name, ios } => {
            let version = match ios {
                14 => iOSVersion::iOS14,
                15 => iOSVersion::iOS15,
                _ => iOSVersion::iOS15,
            };

            let device_id = emulator.create_device(name.clone(), version)?;
            println!("Created device '{}' with ID: {}", name, device_id);
        }

        DeviceCommand::List => {
            let devices = emulator.list_devices();
            if devices.is_empty() {
                println!("No devices found");
            } else {
                println!("Available devices:");
                for device in devices {
                    println!(
                        "  {} - {} ({})",
                        device.id, device.name, device.state
                    );
                }
            }
        }

        DeviceCommand::Boot { device } => {
            let device_mut = emulator.get_device_mut(&device)?;
            device_mut.boot().await?;
            println!("Device '{}' booted successfully", device);
        }

        DeviceCommand::Shutdown { device } => {
            let device_mut = emulator.get_device_mut(&device)?;
            device_mut.shutdown().await?;
            println!("Device '{}' shut down successfully", device);
        }

        DeviceCommand::Delete { device } => {
            emulator.delete_device(&device)?;
            println!("Device '{}' deleted successfully", device);
        }

        DeviceCommand::Info { device } => {
            let device = emulator.get_device(&device)?;
            println!("Device: {}", device.name);
            println!("  ID: {}", device.id);
            println!("  iOS Version: {}", device.ios_version);
            println!("  State: {}", device.state);
            println!("  Screen: {}x{}", device.properties.screen_width, device.properties.screen_height);
            println!("  RAM: {}MB", device.properties.ram_mb);
            println!("  Storage: {}MB", device.properties.storage_mb);
        }
    }

    Ok(())
}

async fn handle_app_command(
    _emulator: &Emulator,
    command: AppCommand,
) -> Result<()> {
    match command {
        AppCommand::Install { device: _, path: _, app_id: _ } => {
            println!("App installation support coming soon");
        }

        AppCommand::Launch { device: _, app_id: _ } => {
            println!("App launching support coming soon");
        }

        AppCommand::List { device: _ } => {
            println!("App listing support coming soon");
        }
    }

    Ok(())
}

fn handle_simulator_command(command: SimulatorCommand) -> Result<()> {
    match command {
        SimulatorCommand::Info => {
            println!("iOSS Emulator - Open-source iOS Simulator");
            println!("Version: {}", ioss_emulator::VERSION);
            println!("Supported iOS Versions: 14.0, 15.0");
        }

        SimulatorCommand::Status => {
            println!("Simulator Status: Ready");
        }
    }

    Ok(())
}
