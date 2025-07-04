use clap::Subcommand;
use rand::seq::IndexedRandom;
use std::fs::read_dir;
use std::path::PathBuf;
use std::{path::Path, process::Command, thread::sleep, time::Duration};
use tracing::{error, info, instrument};

use crate::{
    dot_config::{DotfileTreeConfig, MonitorConfig},
    utils::error::AppError,
};
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
    Isolate,
}

#[derive(Debug)]
pub struct SwwwConf {
    pub resize_type: ResizeType,
    pub transition_fps: i32,
    pub transition_step: i32,
}

impl Default for SwwwConf {
    fn default() -> Self {
        Self {
            resize_type: ResizeType::Crop,
            transition_fps: 60,
            transition_step: 2,
        }
    }
}

#[derive(Debug)]
pub enum ResizeType {
    Crop,
}

pub fn run_daemon() {}

// TODO: better naming
pub fn command_builder(conf: &DotfileTreeConfig) -> Result<(), AppError> {
    for (monitor, monitor_conf) in conf.monitor.iter() {
        create_monitor_subprocess(monitor, monitor_conf);
    }
    Ok(())
}

fn create_monitor_subprocess(monitor: &String, conf: &MonitorConfig) {}

pub fn hardcode_subprocess() {
    info!("logging isolation code snippets");
    let monitor = "HDMI-A-1";
    let rate = 5;
    // TODO: resolver for ~
    let wall_dir = "/home/othi/wallpaper/horizontal";

    loop {
        let img = random_img(wall_dir);
        let conf = SwwwConf::default();

        // TODO: unwrap
        execute_swww(img, conf, monitor);

        sleep(Duration::from_secs(rate));
    }
}

#[instrument(skip(img))]
fn execute_swww<P: AsRef<Path>>(
    img: P,
    // TODO: use
    swww_conf: SwwwConf,
    output: &str,
) -> Result<(), AppError> {
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

#[instrument(skip(path), ret)]
fn random_img<P: AsRef<Path>>(path: P) -> PathBuf {
    // if path is a readable file, ret
    if path.as_ref().is_file() {
        return path.as_ref().into();
    }
    // if path is valid dir, do rand
    if path.as_ref().is_dir() {
        let entries = read_dir(path.as_ref()).unwrap(); // TODO: unwrap
        // NOTE: any abitrary file works here, need to implement media filter
        let mut rng = rand::rng();

        let imgs: Vec<PathBuf> = entries.filter_map(|s| s.ok()).map(|e| e.path()).collect();

        let rand_index: Vec<usize> = (0..imgs.len()).collect();
        let rand_index = rand_index.choose(&mut rng).unwrap();
        let img = imgs.get(*rand_index);

        return img.unwrap().to_path_buf();
    }

    path.as_ref().into()
}
