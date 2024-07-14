use serde::{Deserialize, Serialize};
use tauri::async_runtime::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InnerConfig {
    pub(crate) preset_file: Option<String>,
    pub(crate) num_threads: Option<u32>,
}

impl Default for InnerConfig {
    fn default() -> Self {
        Self {
            preset_file: None,
            num_threads: Some(12),
        }
    }
}

pub(crate) struct Config {
    pub(crate) inner: RwLock<InnerConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inner: RwLock::new(InnerConfig::default()),
        }
    }
}
