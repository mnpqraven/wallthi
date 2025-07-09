use crate::utils::error::AppError;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::Path};
use strum::IntoStaticStr;

/// later to be dotfile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DotfileTreeConfig {
    /// general configuration
    pub general: GeneralConfig,
    /// monitor-dependant configuration
    pub monitor: HashMap<String, MonitorConfig>,
    /// optional configuration of the swww daemon
    pub swww: Option<SwwwConf>,
}

impl Default for DotfileTreeConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            // TODO: see if this is valid or do we need to detect current monitors
            monitor: HashMap::new(),
            swww: Some(SwwwConf::default()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwwwConf {
    pub resize_type: ResizeType,
    pub transition_fps: i32,
    pub transition_step: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum ResizeType {
    Crop,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// duration between each transition
    pub duration: u32,
    /// dirs of horizontal wallpapers
    // NOTE: this path needs to be able to follow symlinks
    pub path: Vec<String>,
    /// dirs of vertical wallpapers
    // NOTE: this path needs to be able to follow symlinks
    pub path_vertical: Vec<String>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            duration: 60,
            path: Vec::new(),
            path_vertical: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub resolution: String,
    /// transform rotation degree
    pub transform: Option<i32>,
    pub vertical: Option<bool>,
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<DotfileTreeConfig, AppError> {
    let file_str = read_to_string(path)?;
    let conf: DotfileTreeConfig = toml::from_str(&file_str)?;
    Ok(conf)
}
