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

pub mod plugins {
    pub mod plugins;
}

pub mod downloader;
pub mod eula; // an EULA file generator
pub mod version; // a version parser

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
    /// Initialize a new Minecraft server (download server.jar)
    Init {
        #[clap(subcommand)]
        server: ServerCommand,
    },
    /// Plugin management
    Plugin {
        #[clap(subcommand)]
        plugin: PluginCommand,
    },
}

#[derive(Subcommand, Debug)]
enum ServerCommand {
    Vanilla {
        /// Minecraft version to use
        #[arg(default_value = "")]
        version: String,

        /// Use the latest snapshot version
        #[arg(long)]
        snapshot: bool,

        /// Accept the Mojang EULA
        #[arg(long)]
        eula: bool,
    },
    Paper {
        /// Minecraft version to use
        #[arg(default_value = "")]
        version: String,

        /// Build number
        #[arg(long, default_value = None)]
        build: Option<u32>,

        /// Accept the Mojang EULA
        #[arg(long)]
        eula: bool,
    },
    Fabric {
        /// Minecraft version to use
        #[arg(default_value = "")]
        version: String,

        /// Loader version
        #[arg(long, default_value = "")]
        loader: String,

        /// Installer version
        #[arg(long, default_value = "")]
        installer: String,

        /// Use unstable loader version
        #[arg(long)]
        unstable_loader: bool,

        /// Use unstable installer version
        #[arg(long)]
        unstable_installer: bool,

        /// Accept the Mojang EULA
        #[arg(long)]
        eula: bool,
    },
}

#[derive(Subcommand, Debug)]
enum PluginCommand{
    Install {
        /// Plugin name
        #[arg(default_value = "")]
        name: String,
    },
}

// convert ServerType to string impl
impl ServerCommand {
    fn to_string(&self) -> String {
        match self {
            ServerCommand::Vanilla { .. } => "Vanilla".to_string(),
            ServerCommand::Paper { .. } => "Paper".to_string(),
            ServerCommand::Fabric { .. } => "Fabric".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let pb = ProgressBar::new_spinner();
    
    match args.command {
        Command::Init { server } => {
            println!("\x1b[33mHint: use --help to see available options!\x1b[0m");
            
            pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_message("Working...");
            
            let eula_accepted = match server {
                ServerCommand::Vanilla { eula, .. } => eula,
                ServerCommand::Paper { eula, .. } => eula,
                ServerCommand::Fabric { eula, .. } => eula,
            };

            if eula_accepted {
                if let Err(e) = eula::generate_eula() {
                    eprintln!("Error generating EULA: {}", e);
                }
            }

            let version_info;
            let download_link = match server {
                ServerCommand::Vanilla { ref version, snapshot, .. } => {
                    let result = server::vanilla::vanilla::get_download_link(Some(version.clone()), snapshot).await;
                    match result {
                        Ok((link, ver_info)) => {
                            version_info = ver_info;
                            link
                        },
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            return Err(e.into());
                        },
                    }
                },
                ServerCommand::Paper { ref version, build, .. } => {
                    let result = server::paper::paper::get_download_link(Some(version.clone()), build).await;
                    match result {
                        Ok((link, ver_info)) => {
                            version_info = ver_info;
                            link
                        },
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            return Err(e.into());
                        },
                    }
                },
                ServerCommand::Fabric { ref version, ref loader, ref installer, unstable_loader, unstable_installer, .. } => {
                    let result = server::fabric::fabric::get_download_link(Some(version.clone()), Some(loader.clone()), Some(installer.clone()), unstable_loader, unstable_installer).await;
                    match result {
                        Ok((link, ver_info)) => {
                            version_info = ver_info;
                            link
                        },
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            return Err(e.into());
                        },
                    }
                },
            };

            pb.finish_and_clear();

            let (progress_tx, mut progress_rx) = mpsc::channel(100);
            let (length_tx, mut length_rx) = mpsc::channel(1);

            tokio::spawn(async move {
                if let Err(e) = downloader::download_file(&download_link, Path::new("server.jar"), progress_tx, length_tx).await {
                    eprintln!("\x1b[31mDownload error: {}\x1b[0m", e);
                }
            });

            let total_bytes = match length_rx.recv().await {
                Some(Some(bytes)) => bytes,
                Some(None) | None => 0,
            };

            let pb = if total_bytes > 0 {
                // Progress bar for known content length
                ProgressBar::new(total_bytes).with_style(
                    ProgressStyle::with_template(
                        "{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
                    )?
                    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
                    })
                    .progress_chars("#>-")
                )
            } else {
                // Spinner for unknown content length
                ProgressBar::new_spinner().with_style(
                    ProgressStyle::with_template("{spinner:.green} {msg}").unwrap()
                )
            };

            pb.set_message("Downloading...");
            pb.enable_steady_tick(Duration::from_millis(100));

            while let Some(downloaded) = progress_rx.recv().await {
                pb.set_position(downloaded);
            }

            pb.finish_and_clear();

            println!("\x1b[32mSuccessfully initialized {} {} server!\x1b[0m", server.to_string(), version_info);
        },
        Command::Plugin { plugin } => {
            match plugin {
                PluginCommand::Install { name } => {
                    let plugin = plugins::plugins::search_plugin(name.to_string()).await?;
                    println!("Plugin: {}", plugin.title);
                    println!("Description: {}", plugin.description);
                    println!("Downloads: {}", plugin.downloads);
                },
            }
        },
    }

    Ok(())
}
