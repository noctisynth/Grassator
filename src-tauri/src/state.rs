use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) preset_file: String,
    pub(crate) num_threads: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            preset_file: String::from("presets/default.json"),
            num_threads: 7,
        }
    }
}
