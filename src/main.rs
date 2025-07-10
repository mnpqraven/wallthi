use crate::{
    command::Commands,
    dot_config::{DotfileTreeConfig, read_config},
    utils::error::AppError,
};
use clap::Parser;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use tracing::info;

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

    let dot_conf = match (args.config, DotfileTreeConfig::first_valid()) {
        (Some(conf_path), _) | (None, Some(conf_path)) => {
            info!("using config file at {conf_path:?}");
            read_config(conf_path)?
        }
        _ => DotfileTreeConfig::default(),
    };
    info!("{dot_conf:?}");

    match args.command {
        Commands::Start { daemon } => {
            if daemon {
                daemon::start_daemonized()?;
            }
            daemon::main_loop(dot_conf)?;
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
