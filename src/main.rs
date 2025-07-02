use std::{collections::HashMap, path::Path};

use clap::Parser;

/// later to be dotfile
struct DotfileTreeConfig {
    /// general configuration
    general: GeneralConfig,
    /// monitor-dependant configuration
    monitor: HashMap<String, MonitorConfig>,
}

struct GeneralConfig {
    /// duration between each transition
    duration: u32,
    /// dirs of horizontal wallpapers
    // NOTE: this path needs to be able to follow symlinks
    path: Vec<String>,
    /// dirs of vertical wallpapers
    // NOTE: this path needs to be able to follow symlinks
    path_vertical: Vec<String>,
}

struct MonitorConfig {
    resolution: String,
    /// transform rotation degree
    transform: Option<i32>,
}

#[derive(Parser)]
struct CliArgs {
    /// toml config file
    config: String,
}

fn main() {
    println!("Hello, world!");
}

fn read_config<P: AsRef<Path>>(path: P) -> DotfileTreeConfig {
    todo!()
}

struct SwwwConf {
    resize_type: ResizeType,
    transition_fps: i32,
    transition_step: i32,
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

enum ResizeType {
    Crop,
}

fn command_builder() {}

struct WallthiStatus {
    // key: monitor name
    // value: current information about he wallpaper
    current_wallpaper: HashMap<String, MonitorStatus>,
}
struct MonitorStatus {
    path: Option<String>,
    remaining_duration: u64,
}
fn status() {}

fn randomizer() -> i32 {
    todo!()
}
