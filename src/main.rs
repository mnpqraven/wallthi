use crate::{
    command::{
        Commands, run_daemon, start_blocking,
        status::daemon_status,
        swww_control::{pause_all, start_or_resume},
    },
    dot_config::{DotfileTreeConfig, read_config},
    utils::error::AppError,
};
use clap::Parser;
use std::path::PathBuf;
use tracing::info;

mod command;
mod dot_config;
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
    info!("{dot_conf:?}");

    match args.command {
        Commands::Daemon => run_daemon(),
        Commands::Status => {
            let status = daemon_status()?;
            println!("{status:?}");
        }
        Commands::Pause => pause_all(),
        Commands::Start => start_or_resume(),
        Commands::StartBlocking => {
            for (monitor, monitor_conf) in dot_conf.monitor.iter() {
                start_blocking(monitor, monitor_conf, &dot_conf);
            }
        }
    };

    Ok(())
}
