//! Git 扫描器：调用系统 Git CLI 读取仓库信息，全程离线（绝不 `git fetch`）。
//!
//! D8 边界：无上游分支时 ahead/behind = 0；`recent_commits` 最多 10 条；
//! 空仓库 `last_commit = None`；detached HEAD → `detached@<短hash>`；
//! status 三态 clean / modified / unknown。非仓库目录返回 `Ok(None)`，
//! git 不可用 / 路径不存在返回可诊断错误，不导致应用崩溃。

use std::path::{Path, PathBuf};
use std::process::Command;

/// Git 扫描错误：必须可诊断，不得导致应用崩溃。
#[derive(Debug)]
pub enum GitError {
    PathNotFound { path: PathBuf },
    GitNotFound { detail: String },
    GitFailed { detail: String },
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathNotFound { path } => write!(f, "path not found: {}", path.display()),
            Self::GitNotFound { detail } => write!(f, "git executable not found: {detail}"),
            Self::GitFailed { detail } => write!(f, "git command failed: {detail}"),
        }
    }
}

impl std::error::Error for GitError {}

/// Working Tree 三态（D8）：读取失败时为 Unknown。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GitStatus {
    Clean,
    Modified,
    Unknown,
}

/// 单条提交记录。
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitCommit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

/// Git 扫描结果（D2：仅存前端内存，不落盘）。
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitMetadata {
    pub branch: String,
    pub status: GitStatus,
    pub changed_files: usize,
    pub ahead: u32,
    pub behind: u32,
    pub last_commit: Option<GitCommit>,
    pub recent_commits: Vec<GitCommit>,
}

/// 用系统默认 `git` 扫描；非仓库返回 `Ok(None)`。
pub fn scan_git(path: &Path) -> Result<Option<GitMetadata>, GitError> {
    scan_git_with(path, "git")
}

/// 可指定 git 可执行文件名的核心实现（便于测试"git 不可用"分支）。
pub fn scan_git_with(path: &Path, git_bin: &str) -> Result<Option<GitMetadata>, GitError> {
    require_dir(path)?;
    // 是否位于 work tree 内；非仓库时该命令以非零码退出。
    let probe = run_git(git_bin, path, &["rev-parse", "--is-inside-work-tree"])?;
    if !probe.success || probe.stdout.trim() != "true" {
        return Ok(None);
    }

    let branch = detect_branch(git_bin, path);
    let (status, changed_files) = detect_status(git_bin, path);
    let recent_commits = detect_recent_commits(git_bin, path);
    let (ahead, behind) = detect_ahead_behind(git_bin, path);

    Ok(Some(GitMetadata {
        branch,
        status,
        changed_files,
        ahead,
        behind,
        last_commit: recent_commits.first().cloned(),
        recent_commits,
    }))
}

/// 确认目标是存在的目录（与 scanner 同款语义）。
fn require_dir(path: &Path) -> Result<(), GitError> {
    match std::fs::metadata(path) {
        Ok(m) if m.is_dir() => Ok(()),
        _ => Err(GitError::PathNotFound {
            path: path.to_path_buf(),
        }),
    }
}

/// 一次 git 调用的结果；spawn 失败（二进制缺失）映射为 GitNotFound。
struct GitOutput {
    success: bool,
    stdout: String,
}

fn run_git(git_bin: &str, cwd: &Path, args: &[&str]) -> Result<GitOutput, GitError> {
    let out = Command::new(git_bin)
        .arg("-C")
        .arg(cwd)
        .args(args)
        .output()
        .map_err(|e| GitError::GitNotFound {
            detail: format!("{git_bin}: {e}"),
        })?;
    Ok(GitOutput {
        success: out.status.success(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
    })
}

/// 分支名：`symbolic-ref --short HEAD`；detached HEAD 降级为 `detached@<短hash>`。
fn detect_branch(git_bin: &str, path: &Path) -> String {
    let sym = run_git(git_bin, path, &["symbolic-ref", "--short", "HEAD"])
        .map(|o| o.success.then(|| o.stdout.trim().to_string()))
        .ok()
        .flatten();
    if let Some(name) = sym {
        return name;
    }
    // 空仓库不会走到这里（symbolic-ref 对空仓库仍成功）；此处仅处理 detached。
    match run_git(git_bin, path, &["rev-parse", "--short", "HEAD"]) {
        Ok(o) if o.success => format!("detached@{}", o.stdout.trim()),
        _ => "HEAD".to_string(),
    }
}

/// status 三态 + 修改文件数；`git status --porcelain` 读取失败 → Unknown。
/// 口径约定：changed_files 计入 porcelain 全部条目（含 untracked `??` 行）。
fn detect_status(git_bin: &str, path: &Path) -> (GitStatus, usize) {
    let out = match run_git(git_bin, path, &["status", "--porcelain"]) {
        Ok(o) if o.success => o,
        _ => return (GitStatus::Unknown, 0),
    };
    let changed = out.stdout.lines().filter(|l| !l.trim().is_empty()).count();
    if changed == 0 {
        (GitStatus::Clean, 0)
    } else {
        (GitStatus::Modified, changed)
    }
}

/// 最近 10 条提交（D8）；空仓库 `git log` 失败 → 空列表。
/// 字段以 \x1f 分隔，避免提交信息中的常规分隔符造成解析歧义。
fn detect_recent_commits(git_bin: &str, path: &Path) -> Vec<GitCommit> {
    let out = match run_git(
        git_bin,
        path,
        &[
            "log",
            "--max-count=10",
            "--pretty=format:%H%x1f%an%x1f%aI%x1f%s",
        ],
    ) {
        Ok(o) if o.success => o,
        _ => return Vec::new(),
    };
    out.stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let mut parts = line.split('\x1f');
            let hash = parts.next()?;
            let author = parts.next()?;
            let date = parts.next()?;
            let message = parts.next()?;
            Some(GitCommit {
                hash: hash.to_string(),
                message: message.to_string(),
                author: author.to_string(),
                date: date.to_string(),
            })
        })
        .collect()
}

/// ahead/behind 基于本地 `@{u}` 计算（绝不 `git fetch`）；无上游 = 0/0。
fn detect_ahead_behind(git_bin: &str, path: &Path) -> (u32, u32) {
    let has_upstream = run_git(git_bin, path, &["rev-parse", "--abbrev-ref", "@{u}"])
        .map(|o| o.success)
        .unwrap_or(false);
    if !has_upstream {
        return (0, 0);
    }
    let out = match run_git(
        git_bin,
        path,
        &["rev-list", "--left-right", "--count", "@{u}...HEAD"],
    ) {
        Ok(o) if o.success => o,
        _ => return (0, 0),
    };
    let mut parts = out.stdout.split_whitespace();
    let behind = parts.next().and_then(|n| n.parse().ok()).unwrap_or(0);
    let ahead = parts.next().and_then(|n| n.parse().ok()).unwrap_or(0);
    (ahead, behind)
}
