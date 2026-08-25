//! scan_project 组装测试（D2）：真实临时目录 + 真实临时 git 仓库（D12）。

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::scanner::{scan_project_path, ScannerError};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p8-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

fn git(dir: &Path, args: &[&str]) {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("spawn git");
    assert!(
        out.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
}

fn touch(dir: &Path, name: &str) {
    std::fs::write(dir.join(name), "").expect("create fixture file");
}

#[test]
fn full_scan_on_node_git_repo() {
    let dir = temp_dir("node-git");
    touch(&dir, "package.json");
    touch(&dir, "tsconfig.json");
    git(&dir, &["init"]);
    git(&dir, &["branch", "-M", "main"]);
    git(&dir, &["config", "user.email", "test@windy.local"]);
    git(&dir, &["config", "user.name", "Windy Test"]);
    git(&dir, &["config", "commit.gpgsign", "false"]);
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-m", "init"]);

    let meta = scan_project_path(&dir).expect("scan");
    assert_eq!(meta.project_type.as_deref(), Some("Node"));
    assert!(meta.tech_stack.iter().any(|t| t == "Node"));
    assert!(meta.tech_stack.iter().any(|t| t == "TypeScript"));
    let git_meta = meta.git.expect("is a repo");
    assert_eq!(git_meta.branch, "main");
    assert_eq!(git_meta.recent_commits.len(), 1);
    assert!(meta.activity.last_modified_at.is_some());
    cleanup(&dir);
}

#[test]
fn non_git_dir_has_null_git() {
    let dir = temp_dir("node-no-git");
    touch(&dir, "package.json");
    let meta = scan_project_path(&dir).expect("scan");
    assert_eq!(meta.project_type.as_deref(), Some("Node"));
    assert!(meta.git.is_none());
    cleanup(&dir);
}

#[test]
fn empty_dir_degrades_every_field() {
    let dir = temp_dir("empty");
    let meta = scan_project_path(&dir).expect("scan");
    assert!(meta.project_type.is_none());
    assert!(meta.tech_stack.is_empty());
    assert!(meta.git.is_none());
    assert!(meta.activity.last_modified_at.is_none());
    assert!(!meta.activity.last_scanned_at.is_empty());
    cleanup(&dir);
}

#[test]
fn missing_path_is_diagnosable_error() {
    let dir = std::env::temp_dir().join("windy-p8-does-not-exist");
    match scan_project_path(&dir) {
        Err(ScannerError::PathNotFound { .. }) => {}
        other => panic!("expected PathNotFound, got {other:?}"),
    }
}
