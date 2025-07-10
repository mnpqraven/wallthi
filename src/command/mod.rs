use crate::command::img_paths::random_img;
use crate::command::state::AppState;
use crate::dot_config::SwwwConf;
use crate::{
    dot_config::{DotfileTreeConfig, MonitorConfig},
    utils::error::AppError,
};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::{path::Path, process::Command, thread::sleep, time::Duration};
use tracing::{error, info, warn};

mod img_paths;
pub mod state;

#[derive(Subcommand, Debug, Serialize, Deserialize, PartialEq)]
pub enum Commands {
    /// prints the current status of the wallpaper queue
    Status,
    /// pause all swww instances in queue
    Pause,
    /// resumes all swww instances in queue
    Resume,
    /// starts all swww instances in queue
    Start {
        /// start the application as a daemon
        #[clap(short, long)]
        daemon: bool,
    },
    /// closes running daemon
    Quit,
}

pub fn swww_loop(
    monitor: &str,
    conf: &MonitorConfig,
    global_conf: &DotfileTreeConfig,
    daemon_state: Arc<RwLock<AppState>>,
) -> Result<(), AppError> {
    info!("logging isolation code snippets {conf:?} {global_conf:?}");
    // TODO: resolver for ~
    let wall_dirs = if conf.vertical {
        global_conf.general.path_vertical.clone()
    } else {
        global_conf.general.path.clone()
    };

    // TODO: killswitch
    // we need some way to talk to this loop for status + play/pause functions
    loop {
        let img = random_img(wall_dirs.clone());
        match daemon_state.try_read() {
            Ok(t) if t.should_exit => return Ok(()),
            Ok(t) if !t.is_paused => {
                execute_swww(img, global_conf.swww.clone().unwrap_or_default(), monitor)?;
            }
            _ => {}
        }

        sleep(Duration::from_secs(global_conf.general.duration.into()));
    }
}

fn execute_swww<P: AsRef<Path>>(img: P, swww_conf: SwwwConf, output: &str) -> Result<(), AppError> {
    if let Some(img_path) = img.as_ref().to_str() {
        info!("[SWWW] executing cmd for img {img_path}");

        let resize_type: &str = swww_conf.resize_type.into();
        let transition_step = swww_conf.transition_step.to_string();
        let transition_fps = swww_conf.transition_fps.to_string();

        let cmd = Command::new("swww")
            .args([
                "img",
                "--resize",
                resize_type,
                "--outputs",
                output,
                "--transition-step",
                &transition_step,
                "--transition-fps",
                &transition_fps,
                img_path,
            ])
            .output()?;

        if !cmd.status.success() {
            let sig = cmd.status.code();
            warn!("swww command failed with status code {sig:?}");
            let stdout = String::from_utf8_lossy(&cmd.stdout);
            warn!("{stdout:?}");
        }

        Ok(())
    } else {
        error!("invalid img path {}", img.as_ref().to_string_lossy());
        Err(AppError::General)
    }
}
