use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::path::Path;
use std::time::Duration;

pub mod server {
    pub mod vanilla {
        pub mod vanilla;
    }
    pub mod paper {
        pub mod paper;
    }
    pub mod fabric {
        pub mod fabric;
    }
}

pub mod downloader;

/// Simple program to initialize a Minecraft server
#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)] // disable the -V, --version flag
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[clap(subcommand)]
        server: ServerCommand,
    },
}

#[derive(Subcommand, Debug)]
enum ServerCommand {
    Vanilla {
        /// Minecraft version to use
        #[arg(default_value = "")]
        version: String,

        #[arg(long)]
        snapshot: bool,
    },
    Paper {
        /// Minecraft version to use
        #[arg(default_value = "")]
        version: String,

        /// Build number
        #[arg(long, default_value = None)]
        build: Option<u32>,
    },
    Fabric {
        /// Minecraft version to use
        #[arg(default_value = "")]
        version: String,

        /// Loader version
        #[arg(long, default_value = "")]
        loader_version: String,

        /// Installer version
        #[arg(long, default_value = "")]
        installer_version: String,

        /// Use unstable loader version
        #[arg(long)]
        unstable_loader: bool,

        /// Use unstable installer version
        #[arg(long)]
        unstable_installer: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    pb.set_message("Working...");
    pb.enable_steady_tick(Duration::from_nanos(100));


    match args.command {
        Command::Init { server } => {
            let download_link = match server {
                ServerCommand::Vanilla { version, snapshot } => {
                    let link = server::vanilla::vanilla::get_download_link(Some(version), snapshot).await;
                    match link {
                        Ok(link) => link,
                        Err(e) => {
                            String::from(format!("Error: {}", e.to_string()))
                        },
                    }
                },
                ServerCommand::Paper { version, build } => {
                    let link = server::paper::paper::get_download_link(Some(version), build).await;
                    match link {
                        Ok(link) => link,
                        Err(e) => {
                            String::from(format!("Error: {}", e.to_string()))
                        },
                    }
                },
                ServerCommand::Fabric { version, loader_version, installer_version, unstable_loader, unstable_installer } => {
                    let link = server::fabric::fabric::get_download_link(Some(version), Some(loader_version), Some(installer_version), unstable_loader, unstable_installer).await;
                    match link {
                        Ok(link) => link,
                        Err(e) => {
                            String::from(format!("Error: {}", e.to_string()))
                        },
                    }
                },
            };

            pb.finish_and_clear();
            downloader::download_file(&download_link, Path::new("server.jar")).await?;
        },
    }

    Ok(())
}
