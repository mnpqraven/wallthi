use clap::Subcommand;

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
}

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
