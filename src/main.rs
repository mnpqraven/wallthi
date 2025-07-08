use crate::{
    command::{
        Commands,
        daemon::{daemon_status, start_daemon},
        state::AppState,
        swww_loop,
    },
    dot_config::{DotfileTreeConfig, read_config},
    utils::error::AppError,
};
use clap::Parser;
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};
use tracing_appender::non_blocking::DEFAULT_BUFFERED_LINES_LIMIT;

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
            start_daemon()?;
            daemon_rt_loop(dot_conf)?;
        }
        Commands::Status => {
            let status = daemon_status()?;
            // tokio talks to daemon
            println!("{status:?}");
        }
        Commands::Pause => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                send_stream().await;
            })
        }
        Commands::Start => {
            daemon_rt_loop(dot_conf)?;
        }
    };

    Ok(())
}

// https://users.rust-lang.org/t/tokio-daemonize-w-privileged-ports/81603
// https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
#[tokio::main]
async fn daemon_rt_loop(dot_conf: DotfileTreeConfig) -> Result<(), AppError> {
    // how to access this from the cli ?
    let state = AppState::new().arced();

    // TODO: listener in main thread
    // see https://tokio.rs/tokio/tutorial/shared-state
    let listener = TcpListener::bind("127.0.0.1:6666").await.unwrap();

    let dot_conf = dot_conf.clone();
    info!("Daemon started with config {dot_conf:?}");
    let monitors = dot_conf.monitor.clone();

    for (monitor, monitor_conf) in monitors.into_iter() {
        let state = state.clone();
        let dot_conf = dot_conf.clone();
        let _handle = tokio::spawn(async move {
            info!("starting task for monitor {monitor} with conf {monitor_conf:?}");
            swww_loop(&monitor, &monitor_conf, &dot_conf, state);
        });
    }

    let (socket, _) = listener.accept().await.unwrap();
    let mut buffer = vec![0; 1024];
    // process given listener
    process_stream(socket, &mut buffer).await;

    Ok(())
}

async fn process_stream(stream: TcpStream, buffer: &mut Vec<u8>) {
    loop {
        stream.readable().await.unwrap();

        match stream.try_read(buffer) {
            Ok(n) => {
                let message = String::from_utf8_lossy(buffer);
                // TODO: serde message passing
                warn!("PAUSING PAUSING {message}");
                buffer.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                panic!("{e}");
            }
        }
    }
}

async fn send_stream() {
    info!("RUNNING PAUSE COMMAND");
    let addr = "127.0.0.1:6666";
    let mut conn = TcpStream::connect(addr)
        .await
        .expect("tcp socket not found. Is wallthi running?");

    // TODO: serde message passing
    conn.write_all(b"hello from pause command!").await.unwrap();
}
