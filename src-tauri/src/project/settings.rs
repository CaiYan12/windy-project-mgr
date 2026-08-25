//! `settings.json` 读写（D6）：与 `projects.json` 同机制
//!（`version` 字段、临时文件替换、损坏可诊断）。

use super::types::{StoreError, Versioned};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CURRENT_VERSION: u32 = 1;

/// MVP 设置：编辑器命令（D6）与主题选择（D1）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub editor_command: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_theme() -> String {
    "system".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editor_command: String::new(),
            theme: default_theme(),
        }
    }
}

impl Settings {
    /// 从文件加载。文件不存在 → 默认设置；文件为空或损坏 → 可诊断错误。
    pub fn load(path: &Path) -> Result<Settings, StoreError> {
        let raw = match std::fs::read_to_string(path) {
            Ok(raw) => raw,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Settings::default());
            }
            Err(e) => return Err(StoreError::Io(e)),
        };
        if raw.trim().is_empty() {
            return Err(StoreError::Corrupted {
                path: path.to_path_buf(),
                detail: "file is empty".to_string(),
            });
        }
        let versioned: Versioned<Settings> =
            serde_json::from_str(&raw).map_err(|e| StoreError::Corrupted {
                path: path.to_path_buf(),
                detail: e.to_string(),
            })?;
        if versioned.version != CURRENT_VERSION {
            return Err(StoreError::VersionMismatch {
                found: versioned.version,
                expected: CURRENT_VERSION,
            });
        }
        Ok(versioned.payload)
    }

    /// 原子保存：写临时文件、同步、重命名替换；失败不破坏原文件。
    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        let versioned = Versioned {
            version: CURRENT_VERSION,
            payload: self.clone(),
        };
        let json = serde_json::to_string_pretty(&versioned).map_err(|e| {
            StoreError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })?;
        super::store::write_atomic(path, &json)
    }
}
