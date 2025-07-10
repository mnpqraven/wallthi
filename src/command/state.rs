use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

use crate::{command::Commands, dot_config::DotfileTreeConfig, utils::error::AppError};

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub is_paused: bool,
    pub should_exit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
    /// wraps the app state in an Arc<RwLock<>>, allowing multiple readers in
    /// thread-safe manner
    pub fn arced(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}

#[derive(Debug)]
pub struct WallthiDaemon {
    pub app_state: Arc<RwLock<AppState>>,
    pub dot_conf: DotfileTreeConfig,
    pub addr: &'static str,
}

impl WallthiDaemon {
    const ADDR: &'static str = "127.0.0.1:6666";
    pub fn new(dot_conf: &DotfileTreeConfig) -> Self {
        Self {
            app_state: AppState::new().arced(),
            dot_conf: dot_conf.clone(),
            addr: Self::ADDR,
        }
    }
    pub fn addr() -> &'static str {
        Self::ADDR
    }

    pub fn pause(&self) -> Result<(), AppError> {
        let mut w = self.app_state.write()?;
        w.is_paused = true;
        Ok(())
    }

    pub fn quit(&self) -> Result<(), AppError> {
        let mut w = self.app_state.write()?;
        w.should_exit = true;
        Ok(())
    }

    pub fn resume(&self) -> Result<(), AppError> {
        let mut w = self.app_state.write()?;
        w.is_paused = false;
        Ok(())
    }

    /// returns a boolean denoting if the daemon should continue or not
    pub fn handle_command(&self, stream: TcpStream, cmd: Commands) -> Result<(), AppError> {
        match cmd {
            Commands::Pause => self.pause(),
            Commands::Resume => self.resume(),
            Commands::Quit => self.quit(),
            Commands::Status => {
                let status: WallthiStatus = WallthiStatus::current(self);
                let bytes = serde_json::to_vec(&status)?;
                stream.try_write(&bytes)?;
                Ok(())
            }
            // noop
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WallthiStatus {
    // key: monitor name
    // value: current information about he wallpaper
    // if there's mismatch between this and `swww query` then swww takes priority
    // on correctness
    pub current_wallpapers: HashMap<String, MonitorStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorStatus {
    pub path: Option<String>,
    pub remaining_duration: u64,
}

impl WallthiStatus {
    fn current(state: &WallthiDaemon) -> Self {
        let current_wallpapers = state
            .dot_conf
            .monitor
            .keys()
            .map(|k| {
                (
                    // TODO: actual data
                    k.clone(),
                    MonitorStatus {
                        path: None,
                        remaining_duration: 0,
                    },
                )
            })
            .collect();
        Self { current_wallpapers }
    }
}
