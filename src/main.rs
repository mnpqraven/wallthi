use crate::{
    command::Commands,
    dot_config::{DotfileTreeConfig, read_config},
    utils::error::AppError,
};
use clap::Parser;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use tracing::{debug, info};

mod command;
mod daemon;
mod dot_config;
mod tcp;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    /// toml config file
    #[arg(long)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let args = CliArgs::parse();
    // TODO: config reader in usr directories
    let dot_conf = match args.config {
        Some(conf_path) => read_config(conf_path)?,
        None => DotfileTreeConfig::default(),
    };

    debug!("{dot_conf:?}");

    match args.command {
        Commands::Start { daemon } => {
            if daemon {
                daemon::start_daemonized()?;
            }
            daemon::main_loop(dot_conf)?;
        }
        Commands::Status => {
            // TODO: tokio talks to daemon
            let status = daemon::daemon_status()?;
            info!("{status:?}");
        }
        cmd => {
            let rt = Runtime::new()?;
            rt.block_on(async {
                tcp::send_cmd(cmd).await?;
                Ok::<(), AppError>(())
            })?;
        }
    };

    Ok(())
}
