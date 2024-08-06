use clap::{Parser, Subcommand};
use spinners::{Spinner, Spinners};
use std::error::Error;

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

    match args.command {
        Command::Init { server } => {
            let download_link = match server {
                ServerCommand::Vanilla { version, snapshot } => {
                    let mut spinner = Spinner::new(Spinners::Dots3, "Loading...".into());
                    let link = server::vanilla::vanilla::get_download_link(Some(version), snapshot).await;
                    spinner.stop();
                    match link {
                        Ok(link) => link,
                        Err(e) => {
                            String::from(format!("\rError: {}", e.to_string()))
                        },
                    }
                },
                ServerCommand::Paper { version, build } => {
                    let mut spinner = Spinner::new(Spinners::Dots3, "Loading...".into());
                    let link = server::paper::paper::get_download_link(Some(version), build).await;
                    spinner.stop();
                    match link {
                        Ok(link) => link,
                        Err(e) => {
                            String::from(format!("\rError: {}", e.to_string()))
                        },
                    }
                },
                ServerCommand::Fabric { version, loader_version, installer_version, unstable_loader, unstable_installer } => {
                    let mut spinner = Spinner::new(Spinners::Dots3, "Loading...".into());
                    let link = server::fabric::fabric::get_download_link(Some(version), Some(loader_version), Some(installer_version), unstable_loader, unstable_installer).await;
                    spinner.stop();
                    match link {
                        Ok(link) => link,
                        Err(e) => {
                            String::from(format!("\rError: {}", e.to_string()))
                        },
                    }
                },
            };

            // Prints the download link, overwriting the spinner
            println!("\r{}", download_link);
        },
    }

    Ok(())
}
