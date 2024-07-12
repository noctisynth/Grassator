use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) preset_file: String,
    pub(crate) threads_count: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            preset_file: String::from("presets/default.json"),
            threads_count: 1,
        }
    }
}
