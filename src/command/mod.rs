use crate::command::img_paths::random_img;
use crate::dot_config::SwwwConf;
use crate::{
    dot_config::{DotfileTreeConfig, MonitorConfig},
    utils::error::AppError,
};
use clap::Subcommand;
use daemonize::Daemonize;
use std::fs::File;
use std::{path::Path, process::Command, thread::sleep, time::Duration};
use tracing::{error, info, instrument};

mod img_paths;
pub mod status;
pub mod swww_control;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// start the application as a daemon
    Daemon,
    /// prints the current status of the wallpaper queue
    Status,
    /// pause all swww instances in queue
    Pause,
    /// starts/resumes all swww instances in queue
    Start,
    /// run isolation command (development code snippets)
    StartBlocking,
}

pub fn run_daemon() -> Result<String, daemonize::Error> {
    let stdout = File::create("/tmp/wallthi.out").unwrap();
    let stderr = File::create("/tmp/wallthi.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/wallthi.pid") // Every method except `new` and `start`
        .working_directory("/tmp") // for default behaviour.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");
    let t = daemonize.start().map(|s| s.to_string());
    dbg!(&t);
    t
}

pub fn start_blocking(monitor: &str, conf: &MonitorConfig, global_conf: &DotfileTreeConfig) {
    info!("logging isolation code snippets {conf:?} {global_conf:?}");
    // TODO: resolver for ~
    let wall_dirs = match conf.vertical {
        Some(true) => global_conf.general.path_vertical.clone(),
        _ => global_conf.general.path.clone(),
    };

    // TODO: killswitch
    // we need some way to talk to this loop for status + play/pause functions
    loop {
        let img = random_img(wall_dirs.clone());

        // TODO: unwrap
        execute_swww(img, global_conf.swww.clone().unwrap_or_default(), monitor).unwrap();

        sleep(Duration::from_secs(global_conf.general.duration.into()));
    }
}

#[instrument(skip(img))]
fn execute_swww<P: AsRef<Path>>(img: P, swww_conf: SwwwConf, output: &str) -> Result<(), AppError> {
    match img.as_ref().to_str() {
        Some(img_path) => {
            let _cmd = Command::new("swww")
                .args(["img", "--resize", "crop", img_path, "-o", &output])
                .output()
                .expect("huh?");

            return Ok(());
        }
        None => {
            error!("invalid img path {}", img.as_ref().to_string_lossy());
            return Err(AppError::General);
        }
    }
}
