use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub is_paused: bool,
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
