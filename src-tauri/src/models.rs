use serde::{Deserialize, Serialize};

use crate::state::InnerConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileSize {
    pub(crate) size: u64,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DownloadResult {
    pub(crate) file_path: String,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SetConfigResult {
    pub(crate) config_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ConfigResult {
    pub(crate) inner: InnerConfig,
    pub(crate) file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ErrorPayload {
    pub(crate) message: String,
}
