//! `projects.json` Store：Load / Save / Create / Update / Delete / Get。
//!
//! 写盘走「临时文件 → 重命名替换」，保存失败绝不破坏原文件。

use super::types::{Project, StoreError, Versioned};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CURRENT_VERSION: u32 = 1;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Store {
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
struct Payload {
    projects: Vec<Project>,
}

impl Store {
    /// 从文件加载。文件不存在 → 空 Store；文件为空或损坏 → 可诊断错误（不静默覆盖）。
    pub fn load(path: &Path) -> Result<Store, StoreError> {
        let raw = match std::fs::read_to_string(path) {
            Ok(raw) => raw,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Store::default());
            }
            Err(e) => return Err(StoreError::Io(e)),
        };
        if raw.trim().is_empty() {
            return Err(StoreError::Corrupted {
                path: path.to_path_buf(),
                detail: "file is empty".to_string(),
            });
        }
        let versioned: Versioned<Payload> =
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
        Ok(Store {
            projects: versioned.payload.projects,
        })
    }

    /// 原子保存：写临时文件、同步、重命名替换；失败不破坏原文件。
    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        let versioned = Versioned {
            version: CURRENT_VERSION,
            payload: Payload {
                projects: self.projects.clone(),
            },
        };
        let json = serde_json::to_string_pretty(&versioned)
            .map_err(|e| StoreError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        write_atomic(path, &json)
    }

    /// 新增项目；重复 ID 拒绝（不做合并）。
    pub fn create(&mut self, project: Project) -> Result<(), StoreError> {
        if self.projects.iter().any(|p| p.id == project.id) {
            return Err(StoreError::DuplicateId { id: project.id });
        }
        self.projects.push(project);
        Ok(())
    }

    /// 按 ID 整体替换；不存在则报 NotFound。
    pub fn update(&mut self, project: Project) -> Result<(), StoreError> {
        match self.projects.iter_mut().find(|p| p.id == project.id) {
            Some(slot) => {
                *slot = project;
                Ok(())
            }
            None => Err(StoreError::NotFound { id: project.id }),
        }
    }

    /// 仅删除记录，永远不删除项目目录；不存在则报 NotFound。
    pub fn delete(&mut self, id: &str) -> Result<Project, StoreError> {
        let idx = self
            .projects
            .iter()
            .position(|p| p.id == id)
            .ok_or_else(|| StoreError::NotFound { id: id.to_string() })?;
        Ok(self.projects.remove(idx))
    }

    pub fn get(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }
}

/// 写临时文件（同目录）后重命名替换目标；任一步失败则清理临时文件并报错。
pub(crate) fn write_atomic(path: &Path, json: &str) -> Result<(), StoreError> {
    use std::fs;
    use std::io::Write;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension(format!(
        "{}.tmp.{}",
        path.extension().and_then(|e| e.to_str()).unwrap_or(""),
        std::process::id()
    ));
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(json.as_bytes())?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        StoreError::Io(e)
    })
}

/// 生成项目 ID：毫秒时间戳 + 进程内计数器（仅依赖 std，无需 uuid 依赖）。
pub fn new_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{millis}-{n}")
}

/// 当前时间的 ISO-8601 UTC 字符串（仅依赖 std）。
pub fn now_utc() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    iso8601_utc(secs)
}

/// Unix 秒 → `YYYY-MM-DDTHH:MM:SSZ`（公历民用算法）。
pub fn iso8601_utc(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let (h, m, s) = (rem / 3600, (rem / 60) % 60, rem % 60);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if mo <= 2 { y + 1 } else { y };
    format!("{year:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

#[cfg(test)]
mod id_time_tests {
    use super::*;

    #[test]
    fn new_id_is_unique_across_calls() {
        let a = new_id();
        let b = new_id();
        assert_ne!(a, b);
        assert!(!a.is_empty());
    }

    #[test]
    fn now_utc_matches_known_epoch_and_date() {
        assert_eq!(iso8601_utc(0), "1970-01-01T00:00:00Z");
        // 2026-08-25T10:30:45Z = 1787653845 秒
        assert_eq!(iso8601_utc(1_787_653_845), "2026-08-25T10:30:45Z");
        // 闰年边界：2024-02-29
        assert_eq!(iso8601_utc(1_709_164_800), "2024-02-29T00:00:00Z");
        assert!(now_utc().ends_with('Z'));
    }
}
