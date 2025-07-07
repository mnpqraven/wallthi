use crate::{
    command::{
        Commands,
        daemon::{daemon_status, start_daemon},
        start_blocking_loop,
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
        Commands::Daemon => {
            // https://users.rust-lang.org/t/tokio-daemonize-w-privileged-ports/81603
            // https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
            start_daemon()?;
            daemon_rt_loop(dot_conf);
        }
        Commands::Status => {
            let status = daemon_status()?;
            // tokio talks to daemon
            println!("{status:?}");
        }
        Commands::Pause => pause_all(),
        Commands::Start => start_or_resume(),
    };

    Ok(())
}

fn daemon_rt_loop(dot_conf: DotfileTreeConfig) {
    info!("Daemon started with config {dot_conf:?}");
    for (monitor, monitor_conf) in dot_conf.monitor.iter() {
        start_blocking_loop(monitor, monitor_conf, &dot_conf);
    }
}
