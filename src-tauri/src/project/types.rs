//! 项目数据层：`Project` 模型、存储错误与版本化文件结构。

use serde::{Deserialize, Serialize};
use std::fmt;

/// 持久化的项目记录。扫描类运行时数据（类型 / 技术栈 / Git / 活动）
/// 按 D2 决策不落盘，因此不在此模型中出现。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,
    pub created_at: String,
}

/// 数据层错误。保存失败绝不破坏原文件（写临时文件再替换）。
#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
    /// 文件存在但为空或不是合法 JSON。
    Corrupted { path: std::path::PathBuf, detail: String },
    /// Schema 版本不匹配。
    VersionMismatch { found: u32, expected: u32 },
    NotFound { id: String },
    DuplicateId { id: String },
    /// 路径查重命中（D3）：拒绝并引导编辑已有记录，不合并。
    DuplicatePath { path: String },
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Corrupted { path, detail } => {
                write!(f, "corrupted data file {}: {detail}", path.display())
            }
            Self::VersionMismatch { found, expected } => {
                write!(f, "unsupported data version {found} (expected {expected})")
            }
            Self::NotFound { id } => write!(f, "project not found: {id}"),
            Self::DuplicateId { id } => write!(f, "duplicate project id: {id}"),
            Self::DuplicatePath { path } => write!(
                f,
                "duplicate project path: {path} (edit the existing record instead)"
            ),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// 版本化文件外层结构：`{ "version": 1, "<key>": ... }`。
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Versioned<T> {
    pub version: u32,
    #[serde(flatten)]
    pub payload: T,
}
