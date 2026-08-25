//! Git Scanner 集成测试：真实临时 git 仓库（D12），覆盖 D8 全部边界。

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::git::{scan_git, scan_git_with, GitError, GitStatus};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p7-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

/// 在目录中执行 git 命令（测试夹具自身允许 panic，产品代码不允许）。
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

/// 构造一个分支名固定为 `main`、带本地提交身份的临时仓库。
fn init_repo(dir: &Path) {
    git(dir, &["init"]);
    git(dir, &["branch", "-M", "main"]);
    git(dir, &["config", "user.email", "test@windy.local"]);
    git(dir, &["config", "user.name", "Windy Test"]);
    git(dir, &["config", "commit.gpgsign", "false"]);
}

fn commit(dir: &Path, file: &str, message: &str) {
    std::fs::write(dir.join(file), message).expect("write fixture file");
    git(dir, &["add", "."]);
    git(dir, &["commit", "-m", message]);
}

// ---------- 切片 1：接缝与错误路径 ----------

#[test]
fn non_git_dir_returns_none() {
    let dir = temp_dir("non-git");
    assert!(scan_git(&dir).expect("scan").is_none());
    cleanup(&dir);
}

#[test]
fn missing_path_is_diagnosable_error() {
    let dir = std::env::temp_dir().join("windy-p7-does-not-exist");
    match scan_git(&dir) {
        Err(GitError::PathNotFound { .. }) => {}
        other => panic!("expected PathNotFound, got {other:?}"),
    }
}

#[test]
fn git_unavailable_is_diagnosable_error() {
    let dir = temp_dir("no-git");
    match scan_git_with(&dir, "windy-no-such-git-binary") {
        Err(GitError::GitNotFound { .. }) => {}
        other => panic!("expected GitNotFound, got {other:?}"),
    }
    cleanup(&dir);
}

#[test]
fn empty_repo_degrades_gracefully() {
    let dir = temp_dir("empty-repo");
    init_repo(&dir);
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    assert_eq!(meta.branch, "main");
    assert_eq!(meta.status, GitStatus::Clean);
    assert_eq!(meta.changed_files, 0);
    assert_eq!(meta.ahead, 0);
    assert_eq!(meta.behind, 0);
    assert!(meta.last_commit.is_none());
    assert!(meta.recent_commits.is_empty());
    cleanup(&dir);
}

// ---------- 切片 3~7：常规仓库与 D8 边界 ----------

#[test]
fn single_commit_repo_reports_metadata() {
    let dir = temp_dir("single-commit");
    init_repo(&dir);
    commit(&dir, "a.txt", "first commit");
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    assert_eq!(meta.branch, "main");
    assert_eq!(meta.status, GitStatus::Clean);
    assert_eq!(meta.changed_files, 0);
    assert_eq!(meta.ahead, 0);
    assert_eq!(meta.behind, 0);
    assert_eq!(meta.recent_commits.len(), 1);
    let last = meta.last_commit.expect("last commit");
    assert_eq!(last.message, "first commit");
    assert_eq!(last.author, "Windy Test");
    assert!(last.hash.len() >= 7, "hash: {}", last.hash);
    assert!(!last.date.is_empty(), "date should be present");
    cleanup(&dir);
}

#[test]
fn modified_tracked_file_marks_modified() {
    let dir = temp_dir("modified");
    init_repo(&dir);
    commit(&dir, "a.txt", "first commit");
    std::fs::write(dir.join("a.txt"), "changed").expect("modify tracked file");
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    assert_eq!(meta.status, GitStatus::Modified);
    assert_eq!(meta.changed_files, 1);
    cleanup(&dir);
}

#[test]
fn untracked_file_counts_as_modified() {
    let dir = temp_dir("untracked");
    init_repo(&dir);
    commit(&dir, "a.txt", "first commit");
    std::fs::write(dir.join("new.txt"), "x").expect("create untracked file");
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    assert_eq!(meta.status, GitStatus::Modified);
    assert_eq!(meta.changed_files, 1);
    cleanup(&dir);
}

#[test]
fn recent_commits_capped_at_ten_newest_first() {
    let dir = temp_dir("many-commits");
    init_repo(&dir);
    for i in 1..=12 {
        commit(&dir, &format!("f{i}.txt"), &format!("commit {i}"));
    }
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    assert_eq!(meta.recent_commits.len(), 10);
    assert_eq!(meta.recent_commits[0].message, "commit 12");
    assert_eq!(meta.recent_commits[9].message, "commit 3");
    assert_eq!(meta.last_commit.expect("last").message, "commit 12");
    cleanup(&dir);
}

#[test]
fn branch_without_upstream_has_zero_ahead_behind() {
    let dir = temp_dir("no-upstream");
    init_repo(&dir);
    commit(&dir, "a.txt", "first commit");
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    assert_eq!((meta.ahead, meta.behind), (0, 0));
    cleanup(&dir);
}

#[test]
fn ahead_behind_computed_from_local_upstream() {
    let dir = temp_dir("upstream");
    init_repo(&dir);
    commit(&dir, "a.txt", "base commit");
    git(&dir, &["branch", "up"]);
    git(&dir, &["branch", "--set-upstream-to=up"]);
    // main 领先 up 一条。
    commit(&dir, "b.txt", "ahead commit");
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    assert_eq!((meta.ahead, meta.behind), (1, 0));
    // up 再进一条 → main 落后一条。
    git(&dir, &["checkout", "up"]);
    commit(&dir, "c.txt", "upstream commit");
    git(&dir, &["checkout", "main"]);
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    assert_eq!((meta.ahead, meta.behind), (1, 1));
    cleanup(&dir);
}

#[test]
fn detached_head_reports_detached_at_short_hash() {
    let dir = temp_dir("detached");
    init_repo(&dir);
    commit(&dir, "a.txt", "first commit");
    git(&dir, &["checkout", "--detach", "HEAD"]);
    let meta = scan_git(&dir).expect("scan").expect("is a repo");
    let short = std::process::Command::new("git")
        .args(["-C"])
        .arg(&dir)
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("spawn git");
    let short = String::from_utf8_lossy(&short.stdout).trim().to_string();
    assert_eq!(meta.branch, format!("detached@{short}"));
    cleanup(&dir);
}
