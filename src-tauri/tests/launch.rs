//! 分离式启动集成测试（D4 / ADR 0002，D12：真实临时目录 + 真实进程）。
//!
//! 计划构造函数为纯函数直接断言；`spawn_plan` / `run_in_terminal_with` /
//! `open_in_editor` 用真实可执行文件（`hostname`：忽略参数、立即退出，
//! 不残留进程）与不存在的二进制名验证成功拉起、拉起失败与 wt→powershell
//! 回退路径。

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::launch::{
    editor_plan, open_dir_plan, open_in_editor, ps_plan, run_in_terminal_with, spawn_plan,
    wt_plan, LaunchError, LaunchPlan,
};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p10-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

// ---------- 计划构造（纯函数） ----------

#[test]
fn wt_plan_shape() {
    let cwd = Path::new(r"C:\dev\demo");
    let plan = wt_plan(cwd, "pnpm dev", "wt", "powershell");
    assert_eq!(plan.program, "wt");
    assert_eq!(
        plan.args,
        vec!["-d", r"C:\dev\demo", "powershell", "-NoExit", "-Command", "pnpm dev"]
    );
    assert_eq!(plan.cwd, cwd);
}

#[test]
fn ps_plan_shape() {
    let cwd = Path::new(r"C:\dev\demo");
    let plan = ps_plan(cwd, "cargo build", "powershell");
    assert_eq!(plan.program, "powershell");
    assert_eq!(plan.args, vec!["-NoExit", "-Command", "cargo build"]);
    assert_eq!(plan.cwd, cwd);
}

#[test]
fn editor_plan_shape() {
    let path = Path::new(r"C:\dev\demo");
    let plan = editor_plan("code", path).expect("configured editor");
    assert_eq!(plan.program, "code");
    assert_eq!(plan.args, vec![r"C:\dev\demo"]);
    assert_eq!(plan.cwd, path);
}

#[test]
fn editor_plan_empty_command_is_not_configured() {
    match editor_plan("", Path::new(r"C:\dev\demo")) {
        Err(LaunchError::EditorNotConfigured) => {}
        other => panic!("expected EditorNotConfigured, got {other:?}"),
    }
    match editor_plan("   ", Path::new(r"C:\dev\demo")) {
        Err(LaunchError::EditorNotConfigured) => {}
        other => panic!("expected EditorNotConfigured for blanks, got {other:?}"),
    }
}

#[test]
fn open_dir_plan_shape() {
    let path = Path::new(r"C:\dev\demo");
    let plan = open_dir_plan(path);
    assert_eq!(plan.program, "explorer");
    assert_eq!(plan.args, vec![r"C:\dev\demo"]);
    assert_eq!(plan.cwd, path);
}

// ---------- 真实进程拉起 ----------

#[test]
fn spawn_plan_launches_real_process() {
    let dir = temp_dir("spawn");
    let plan = LaunchPlan {
        program: "hostname".to_string(),
        args: vec!["ignored".to_string()],
        cwd: dir.clone(),
    };
    spawn_plan(&plan).expect("hostname must spawn");
    cleanup(&dir);
}

#[test]
fn spawn_plan_missing_binary_is_launch_failed() {
    let dir = temp_dir("spawn-missing");
    let plan = LaunchPlan {
        program: "windy-no-such-binary-xyz".to_string(),
        args: vec![],
        cwd: dir.clone(),
    };
    match spawn_plan(&plan) {
        Err(LaunchError::LaunchFailed { detail }) => {
            assert!(detail.contains("windy-no-such-binary-xyz"), "got: {detail}");
        }
        other => panic!("expected LaunchFailed, got {other:?}"),
    }
    cleanup(&dir);
}

#[test]
fn run_in_terminal_prefers_wt_and_falls_back() {
    let dir = temp_dir("run-fallback");
    // wt 二进制不存在 → 回退到真实存在的 hostname（充当终端替身，立即退出）。
    run_in_terminal_with(&dir, "echo hello", "windy-no-wt-xyz", "hostname")
        .expect("fallback must succeed");
    cleanup(&dir);
}

#[test]
fn run_in_terminal_with_wt_present_succeeds() {
    let dir = temp_dir("run-wt");
    // 注入 hostname 充当 wt：优先路径直接成功。
    run_in_terminal_with(&dir, "echo hello", "hostname", "powershell")
        .expect("wt path must succeed");
    cleanup(&dir);
}

#[test]
fn run_in_terminal_missing_path_is_diagnosable() {
    let missing = std::env::temp_dir().join("windy-p10-does-not-exist");
    match run_in_terminal_with(&missing, "echo hello", "hostname", "hostname") {
        Err(LaunchError::PathNotFound { .. }) => {}
        other => panic!("expected PathNotFound, got {other:?}"),
    }
}

#[test]
fn run_in_terminal_empty_command_is_diagnosable() {
    let dir = temp_dir("run-empty");
    for cmd in ["", "   "] {
        match run_in_terminal_with(&dir, cmd, "hostname", "hostname") {
            Err(LaunchError::EmptyCommand) => {}
            other => panic!("expected EmptyCommand for {cmd:?}, got {other:?}"),
        }
    }
    cleanup(&dir);
}

#[test]
fn run_in_terminal_both_terminals_missing_is_launch_failed() {
    let dir = temp_dir("run-both-missing");
    match run_in_terminal_with(&dir, "echo hello", "windy-no-wt-xyz", "windy-no-ps-xyz") {
        Err(LaunchError::LaunchFailed { .. }) => {}
        other => panic!("expected LaunchFailed, got {other:?}"),
    }
    cleanup(&dir);
}

#[test]
fn open_in_editor_launches_configured_editor() {
    let dir = temp_dir("editor");
    // 注入 hostname 充当编辑器：真实拉起成功且立即退出。
    open_in_editor("hostname", &dir).expect("editor launch must succeed");
    cleanup(&dir);
}

#[test]
fn open_in_editor_not_configured_before_path_check() {
    // 未配置优先于路径检查报错（D6 引导语义）。
    let missing = std::env::temp_dir().join("windy-p10-does-not-exist");
    match open_in_editor("", &missing) {
        Err(LaunchError::EditorNotConfigured) => {}
        other => panic!("expected EditorNotConfigured, got {other:?}"),
    }
}

#[test]
fn open_in_editor_missing_path_is_diagnosable() {
    let missing = std::env::temp_dir().join("windy-p10-does-not-exist");
    match open_in_editor("hostname", &missing) {
        Err(LaunchError::PathNotFound { .. }) => {}
        other => panic!("expected PathNotFound, got {other:?}"),
    }
}
