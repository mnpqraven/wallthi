use crate::utils::error::AppError;
use daemonize::Daemonize;
use std::{collections::HashMap, fs::File};

pub fn start_daemon() -> Result<(), daemonize::Error> {
    let stdout = File::create("/tmp/wallthi.out").unwrap();
    let stderr = File::create("/tmp/wallthi.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/wallthi.pid")
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);
    daemonize.start()
}

#[derive(Debug)]
pub struct WallthiStatus {
    // key: monitor name
    // value: current information about he wallpaper
    // if there's mismatch between this and `swww query` then swww takes priority
    // on correctness
    pub current_wallpapers: HashMap<String, MonitorStatus>,
}
#[derive(Debug)]
pub struct MonitorStatus {
    pub path: Option<String>,
    pub remaining_duration: u64,
}

pub fn daemon_status() -> Result<WallthiStatus, AppError> {
    todo!()
}
