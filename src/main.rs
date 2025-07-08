use crate::{
    command::{
        Commands,
        daemon::{daemon_status, start_daemon},
        start_blocking_loop,
        state::AppState,
    },
    dot_config::{DotfileTreeConfig, read_config},
    utils::error::AppError,
};
use clap::Parser;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
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

    // how to access this from the cli ?
    let app_state = AppState::new().arced();

    match args.command {
        Commands::Daemon => {
            start_daemon()?;
            daemon_rt_loop(dot_conf, app_state)?;
        }
        Commands::Status => {
            let status = daemon_status()?;
            // tokio talks to daemon
            println!("{status:?}");
        }
        Commands::Pause => {}
        Commands::Start => {}
    };

    Ok(())
}

// https://users.rust-lang.org/t/tokio-daemonize-w-privileged-ports/81603
// https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
#[tokio::main]
async fn daemon_rt_loop(
    dot_conf: DotfileTreeConfig,
    state: Arc<RwLock<AppState>>,
) -> Result<(), AppError> {
    let dot_conf = dot_conf.clone();
    info!("Daemon started with config {dot_conf:?}");
    let monitors = dot_conf.monitor.clone();

    for (monitor, monitor_conf) in monitors.into_iter() {
        let state = state.clone();
        let dot_conf = dot_conf.clone();
        tokio::spawn(async move {
            start_blocking_loop(&monitor, &monitor_conf, &dot_conf, state);
        });
    }

    Ok(())
}
