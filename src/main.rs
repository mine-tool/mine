use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use tokio::sync::mpsc;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use std::fmt::Write;

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
    pb.enable_steady_tick(Duration::from_millis(100));


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

            let (progress_tx, mut progress_rx) = mpsc::channel(100);
            let (length_tx, mut length_rx) = mpsc::channel(1);

            tokio::spawn(async move {
                if let Err(e) = downloader::download_file(&download_link, Path::new("server.jar"), progress_tx, length_tx).await {
                    eprintln!("Download error: {}", e);
                }
            });

            let total_bytes = length_rx.recv().await.unwrap();

            let pb = ProgressBar::new(total_bytes.unwrap_or(0));
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
                )?
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                    write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
                })
                .progress_chars("#>-"),
            );
            pb.set_message("Downloading...");
            pb.enable_steady_tick(Duration::from_millis(100));

            while let Some(downloaded) = progress_rx.recv().await {
                pb.set_position(downloaded);
            }
        },
    }

    Ok(())
}
