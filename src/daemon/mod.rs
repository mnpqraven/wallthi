use crate::{
    command::{state::WallthiDaemon, swww_loop},
    dot_config::DotfileTreeConfig,
    tcp,
    utils::error::AppError,
};
use daemonize::Daemonize;
use std::fs::File;
use tokio::net::TcpListener;
use tracing::info;

// https://users.rust-lang.org/t/tokio-daemonize-w-privileged-ports/81603
// https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
#[tokio::main]
pub async fn main_loop(dot_conf: DotfileTreeConfig) -> Result<(), AppError> {
    let daemon = WallthiDaemon::new(&dot_conf);

    // see https://tokio.rs/tokio/tutorial/shared-state
    let listener = TcpListener::bind(daemon.addr).await?;

    let dot_conf = dot_conf.clone();
    info!("Daemon started with config {dot_conf:?}");
    let monitors = dot_conf.monitor.clone();
    let mut handles = vec![];

    for (monitor, monitor_conf) in monitors.into_iter() {
        let state = daemon.app_state.clone();
        let dot_conf = dot_conf.clone();
        let handle = tokio::spawn(async move {
            info!("starting task for monitor {monitor} with conf {monitor_conf:?}");
            swww_loop(&monitor, &monitor_conf, &dot_conf, state)?;
            Ok::<(), AppError>(())
        });
        handles.push(handle);
    }

    // process given listener
    loop {
        let (socket, _) = listener.accept().await?;
        tcp::process_cmd(socket, &daemon).await?;

        if let Ok(app) = daemon.app_state.try_read()
            && app.should_exit
        {
            break;
        };
    }
    Ok(())
}

/// NOTE: this MUST be called before we enter `[tokio::main]`
pub fn start_daemonized() -> Result<(), daemonize::Error> {
    let stdout = File::create("/tmp/wallthi.out").unwrap();
    let stderr = File::create("/tmp/wallthi.err").unwrap();

    // TODO: probably needs testing
    let daemonize = Daemonize::new()
        .pid_file("/tmp/wallthi.pid")
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);
    daemonize.start()
}
