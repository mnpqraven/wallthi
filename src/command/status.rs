use std::collections::HashMap;

use crate::utils::error::AppError;

#[derive(Debug)]
pub struct WallthiStatus {
    // key: monitor name
    // value: current information about he wallpaper
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
