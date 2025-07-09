use std::sync::{Arc, RwLock};

use crate::{command::Commands, utils::error::AppError};

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
    pub addr: &'static str,
}

impl WallthiDaemon {
    const ADDR: &'static str = "127.0.0.1:6666";
    pub fn new() -> Self {
        Self {
            app_state: AppState::new().arced(),
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
    pub fn handle_command(&self, cmd: Commands) -> Result<(), AppError> {
        match cmd {
            Commands::Pause => self.pause(),
            Commands::Resume => self.resume(),
            Commands::Quit => self.quit(),
            // noop
            _ => Ok(()),
        }
    }
}
