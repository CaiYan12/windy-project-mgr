//! 路径查重（D3）：规范化后大小写不敏感比较，命中即拒绝、不合并。
//!
//! MVP 仅做 Windows 语义的字符串规范化（分隔符统一、去尾分隔符），
//! 不访问文件系统；绝对路径由 UI 文件选择器保证。

use super::store::Store;
use super::types::Project;

/// 规范化路径：`/` 统一为 `\`，词法解析 `.` / `..`，相对路径以当前目录绝对化，
/// 去除末尾分隔符；保留原始大小写。不访问文件系统。
pub fn normalize_path(raw: &str) -> String {
    let unified: String = raw
        .chars()
        .map(|c| if c == '/' { '\\' } else { c })
        .collect();
    let absolute = make_absolute(&unified);
    let resolved = resolve_segments(&absolute);
    let trimmed = resolved.trim_end_matches('\\');
    if trimmed.is_empty() {
        resolved
    } else {
        trimmed.to_string()
    }
}

/// 相对路径 → 绝对路径（以当前工作目录为基）；已绝对则原样返回。
fn make_absolute(path: &str) -> String {
    let is_unc = path.starts_with("\\\\");
    let has_root = path.starts_with('\\');
    let has_drive_root = path.len() >= 3
        && path.as_bytes()[0].is_ascii_alphabetic()
        && path.as_bytes()[1] == b':'
        && path.as_bytes()[2] == b'\\';
    if is_unc || has_root || has_drive_root {
        return path.to_string();
    }
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().replace('/', "\\"))
        .unwrap_or_else(|_| "C:\\".to_string());
    format!("{cwd}\\{path}")
}

/// 词法解析路径段：丢弃 `.`，`..` 弹出上一级（不越过盘符根 / UNC 主机段）。
fn resolve_segments(path: &str) -> String {
    let (prefix, rest) = split_prefix(path);
    let mut stack: Vec<&str> = Vec::new();
    for seg in rest.split('\\') {
        match seg {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            s => stack.push(s),
        }
    }
    if stack.is_empty() {
        prefix
    } else {
        format!("{prefix}{}", stack.join("\\"))
    }
}

/// 分离不可弹出的前缀：UNC `\\server\share\` 或盘符根 `X:\`；无前缀返回空。
fn split_prefix(path: &str) -> (String, &str) {
    if let Some(stripped) = path.strip_prefix("\\\\") {
        let mut parts = stripped.splitn(3, '\\');
        let server = parts.next().unwrap_or("");
        let share = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("");
        return (format!("\\\\{server}\\{share}\\"), rest);
    }
    if path.len() >= 3
        && path.as_bytes()[0].is_ascii_alphabetic()
        && path.as_bytes()[1] == b':'
        && path.as_bytes()[2] == b'\\'
    {
        return (path[..3].to_string(), &path[3..]);
    }
    (String::new(), path)
}

/// Windows 语义的路径相等：规范化后大小写不敏感。
pub fn is_same_path(a: &str, b: &str) -> bool {
    normalize_path(a).eq_ignore_ascii_case(&normalize_path(b))
}

impl Store {
    /// 按路径查找重复记录（排除指定 ID，供 update 自排除使用）。
    pub fn find_duplicate_by_path(&self, path: &str, exclude_id: Option<&str>) -> Option<&Project> {
        self.projects
            .iter()
            .find(|p| exclude_id != Some(p.id.as_str()) && is_same_path(&p.path, path))
    }
}
