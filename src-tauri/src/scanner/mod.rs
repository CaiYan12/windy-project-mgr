//! Project Scanner：项目类型 / 技术栈 / 活动检测与根目录启动脚本枚举。
//!
//! 仅做根目录扁平文件特征检查（原始文档第 12 节）：无 AST、不递归深扫、
//! 单项失败不影响其它扫描；路径不存在 / 无权限时明确报错；非项目目录返回 Unknown。

use std::path::{Path, PathBuf};

/// Scanner 错误：必须可诊断，不得导致应用崩溃（原始文档第 12/13 节精神）。
#[derive(Debug)]
pub enum ScannerError {
    PathNotFound { path: PathBuf },
    PermissionDenied { path: PathBuf, detail: String },
    Io(std::io::Error),
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathNotFound { path } => write!(f, "path not found: {}", path.display()),
            Self::PermissionDenied { path, detail } => {
                write!(f, "permission denied for {}: {detail}", path.display())
            }
            Self::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for ScannerError {}

impl From<std::io::Error> for ScannerError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// 最近活动（D2：lastScannedAt 为本次运行内的扫描时刻）。
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMetadata {
    pub last_modified_at: Option<String>,
    pub last_scanned_at: String,
}

/// 根目录启动脚本候选（D5）。
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupScript {
    pub name: String,
    pub path: String,
}

/// 项目类型检测：特征文件命中返回类型名，均不命中返回 None（UI 显示 Unknown）。
pub fn detect_project_type(path: &Path) -> Result<Option<String>, ScannerError> {
    require_dir(path)?;
    if has_file(path, "package.json") {
        return Ok(Some("Node".to_string()));
    }
    if has_file(path, "pyproject.toml") || has_file(path, "requirements.txt") {
        return Ok(Some("Python".to_string()));
    }
    if has_file(path, "Cargo.toml") {
        return Ok(Some("Rust".to_string()));
    }
    if has_file(path, "pom.xml") {
        return Ok(Some("Java".to_string()));
    }
    if has_file_with_ext(path, "csproj")? {
        return Ok(Some("CSharp".to_string()));
    }
    Ok(None)
}

/// 确认目标是存在的目录；不存在 / 非目录 / 无权限 → 可诊断错误。
fn require_dir(path: &Path) -> Result<(), ScannerError> {
    match std::fs::metadata(path) {
        Ok(m) if m.is_dir() => Ok(()),
        Ok(_) => Err(ScannerError::PathNotFound {
            path: path.to_path_buf(),
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(ScannerError::PathNotFound {
                path: path.to_path_buf(),
            })
        }
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            Err(ScannerError::PermissionDenied {
                path: path.to_path_buf(),
                detail: e.to_string(),
            })
        }
        Err(e) => Err(ScannerError::Io(e)),
    }
}

fn has_file(path: &Path, name: &str) -> bool {
    path.join(name).is_file()
}

/// 根目录中是否存在指定扩展名的文件（大小写不敏感，不递归；单项失败不阻断）。
fn has_file_with_ext(path: &Path, ext: &str) -> Result<bool, ScannerError> {
    for entry in read_dir_tolerant(path)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // 单项失败不影响其它扫描
        };
        if entry.path().is_file()
            && entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case(ext))
                .unwrap_or(false)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// read_dir 包装：权限错误转为 PermissionDenied。
fn read_dir_tolerant(path: &Path) -> Result<std::fs::ReadDir, ScannerError> {
    std::fs::read_dir(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            ScannerError::PermissionDenied {
                path: path.to_path_buf(),
                detail: e.to_string(),
            }
        } else if e.kind() == std::io::ErrorKind::NotFound {
            ScannerError::PathNotFound {
                path: path.to_path_buf(),
            }
        } else {
            ScannerError::Io(e)
        }
    })
}

/// 技术栈检测：特征文件 → 标签列表（可多项），按固定顺序输出。
pub fn detect_tech_stack(path: &Path) -> Result<Vec<String>, ScannerError> {
    require_dir(path)?;
    let mut files: Vec<String> = Vec::new();
    for entry in read_dir_tolerant(path)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // 单项失败不影响其它扫描
        };
        if entry.path().is_file() {
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }
    }
    let has = |n: &str| files.iter().any(|f| f == n);
    let has_prefix = |p: &str| files.iter().any(|f| f.starts_with(p));
    let has_ext = |e: &str| {
        files.iter().any(|f| {
            f.rsplit('.')
                .next()
                .map(|x| x.eq_ignore_ascii_case(e))
                .unwrap_or(false)
        })
    };

    let mut labels: Vec<String> = Vec::new();
    let mut push = |label: &str| {
        if !labels.iter().any(|l| l == label) {
            labels.push(label.to_string());
        }
    };
    if has("package.json") {
        push("Node");
    }
    if has("pnpm-lock.yaml") {
        push("pnpm");
    }
    if has("package-lock.json") {
        push("npm");
    }
    if has("yarn.lock") {
        push("yarn");
    }
    if has("tsconfig.json") {
        push("TypeScript");
    }
    if has_prefix("vite.config.") {
        push("Vite");
    }
    if has_prefix("next.config.") {
        push("Next.js");
    }
    if has("pyproject.toml") || has("requirements.txt") {
        push("Python");
    }
    if has("Cargo.toml") {
        push("Rust");
    }
    if has("pom.xml") {
        push("Java");
    }
    if has_ext("csproj") {
        push("C#");
    }
    Ok(labels)
}

/// 活动检测：根目录（不递归）最近修改时间 + 本次扫描时刻。
pub fn detect_activity(path: &Path) -> Result<ActivityMetadata, ScannerError> {
    require_dir(path)?;
    let mut newest: Option<std::time::SystemTime> = None;
    for entry in read_dir_tolerant(path)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // 单项失败不影响其它扫描
        };
        if let Ok(meta) = entry.metadata() {
            if let Ok(m) = meta.modified() {
                newest = Some(match newest {
                    Some(n) if n >= m => n,
                    _ => m,
                });
            }
        }
    }
    let last_modified_at = newest.map(|t| {
        let secs = t
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        crate::project::store::iso8601_utc(secs)
    });
    Ok(ActivityMetadata {
        last_modified_at,
        last_scanned_at: crate::project::store::now_utc(),
    })
}

/// 根目录启动脚本枚举（不递归）：全部 `*.bat` / `*.cmd` / `*.ps1`，
/// 按 D5 排序：文件名含 `start` 优先，其次含 `run`，其余按字母序。
pub fn list_startup_scripts(path: &Path) -> Result<Vec<StartupScript>, ScannerError> {
    require_dir(path)?;
    let mut scripts: Vec<StartupScript> = Vec::new();
    for entry in read_dir_tolerant(path)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        let is_script = p
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| matches!(e.to_ascii_lowercase().as_str(), "bat" | "cmd" | "ps1"))
            .unwrap_or(false);
        if !is_script {
            continue;
        }
        scripts.push(StartupScript {
            name: entry.file_name().to_string_lossy().to_string(),
            path: p.to_string_lossy().to_string(),
        });
    }
    scripts.sort_by(|a, b| {
        let ka = (script_rank(&a.name), a.name.to_ascii_lowercase());
        let kb = (script_rank(&b.name), b.name.to_ascii_lowercase());
        ka.cmp(&kb)
    });
    Ok(scripts)
}

/// D5 排序桶：含 `start` = 0，含 `run` = 1，其余 = 2。
fn script_rank(name: &str) -> u8 {
    let lower = name.to_ascii_lowercase();
    if lower.contains("start") {
        0
    } else if lower.contains("run") {
        1
    } else {
        2
    }
}
