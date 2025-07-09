use crate::{
    command::{
        Commands,
        daemon::{daemon_status, start_daemon},
        state::WallthiDaemon,
        swww_loop,
    },
    dot_config::{DotfileTreeConfig, read_config},
    utils::error::AppError,
};
use clap::Parser;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tracing::{info, warn};

mod command;
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
    info!("{dot_conf:?}");

    match args.command {
        Commands::Daemon => {
            start_daemon()?;
            daemon_rt_loop(dot_conf)?;
        }
        Commands::Start => {
            daemon_rt_loop(dot_conf)?;
        }
        Commands::Status => {
            let status = daemon_status()?;
            // tokio talks to daemon
            println!("{status:?}");
        }
        cmd @ (Commands::Pause | Commands::Resume) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = tcp::send_cmd(cmd).await;
            })
        }
    };

    Ok(())
}

// https://users.rust-lang.org/t/tokio-daemonize-w-privileged-ports/81603
// https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
#[tokio::main]
async fn daemon_rt_loop(dot_conf: DotfileTreeConfig) -> Result<(), AppError> {
    let daemon = WallthiDaemon::new();

    // see https://tokio.rs/tokio/tutorial/shared-state
    let listener = TcpListener::bind(daemon.addr).await?;

    let dot_conf = dot_conf.clone();
    info!("Daemon started with config {dot_conf:?}");
    let monitors = dot_conf.monitor.clone();

    for (monitor, monitor_conf) in monitors.into_iter() {
        let state = daemon.app_state.clone();
        let dot_conf = dot_conf.clone();
        let _handle = tokio::spawn(async move {
            info!("starting task for monitor {monitor} with conf {monitor_conf:?}");
            swww_loop(&monitor, &monitor_conf, &dot_conf, state);
        });
    }

    // process given listener
    loop {
        let (socket, _) = listener.accept().await?;
        tcp::process_stream(socket, &daemon).await?;
    }
}
