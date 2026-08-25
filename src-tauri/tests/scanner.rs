//! Project Scanner 集成测试：真实临时目录（D12）。

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::scanner::{
    detect_activity, detect_project_type, detect_tech_stack, list_startup_scripts, ScannerError,
};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p6-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

fn touch(dir: &Path, name: &str) {
    std::fs::write(dir.join(name), "").expect("create fixture file");
}

// ---------- 项目类型 ----------

#[test]
fn detects_node_project() {
    let dir = temp_dir("node");
    touch(&dir, "package.json");
    assert_eq!(detect_project_type(&dir).expect("scan"), Some("Node".to_string()));
    cleanup(&dir);
}

#[test]
fn detects_python_project() {
    let dir = temp_dir("python");
    touch(&dir, "requirements.txt");
    assert_eq!(detect_project_type(&dir).expect("scan"), Some("Python".to_string()));
    cleanup(&dir);
}

#[test]
fn detects_python_project_via_pyproject() {
    let dir = temp_dir("pyproject");
    touch(&dir, "pyproject.toml");
    assert_eq!(detect_project_type(&dir).expect("scan"), Some("Python".to_string()));
    cleanup(&dir);
}

#[test]
fn detects_rust_project() {
    let dir = temp_dir("rust");
    touch(&dir, "Cargo.toml");
    assert_eq!(detect_project_type(&dir).expect("scan"), Some("Rust".to_string()));
    cleanup(&dir);
}

#[test]
fn detects_java_and_csharp_projects() {
    let java = temp_dir("java");
    touch(&java, "pom.xml");
    assert_eq!(detect_project_type(&java).expect("scan"), Some("Java".to_string()));
    cleanup(&java);

    let cs = temp_dir("csharp");
    touch(&cs, "App.csproj");
    assert_eq!(detect_project_type(&cs).expect("scan"), Some("C#".to_string()));
    cleanup(&cs);
}

#[test]
fn empty_dir_is_unknown() {
    let dir = temp_dir("unknown");
    assert_eq!(detect_project_type(&dir).expect("scan"), None);
    cleanup(&dir);
}

#[test]
fn missing_path_returns_path_not_found() {
    let dir = temp_dir("missing-base");
    let missing = dir.join("does-not-exist");
    let err = detect_project_type(&missing).expect_err("missing path must error");
    assert!(matches!(err, ScannerError::PathNotFound { .. }), "got: {err:?}");
    cleanup(&dir);
}

// ---------- 技术栈 ----------

#[test]
fn detects_node_tech_stack_labels() {
    let dir = temp_dir("stack-node");
    touch(&dir, "package.json");
    touch(&dir, "pnpm-lock.yaml");
    touch(&dir, "tsconfig.json");
    touch(&dir, "vite.config.ts");
    let stack = detect_tech_stack(&dir).expect("scan");
    for label in ["Node", "pnpm", "TypeScript", "Vite"] {
        assert!(stack.contains(&label.to_string()), "missing {label} in {stack:?}");
    }
    cleanup(&dir);
}

#[test]
fn detects_nextjs_and_yarn_labels() {
    let dir = temp_dir("stack-next");
    touch(&dir, "package.json");
    touch(&dir, "yarn.lock");
    touch(&dir, "next.config.mjs");
    let stack = detect_tech_stack(&dir).expect("scan");
    for label in ["Node", "yarn", "Next.js"] {
        assert!(stack.contains(&label.to_string()), "missing {label} in {stack:?}");
    }
    cleanup(&dir);
}

#[test]
fn detects_rust_stack_and_empty_for_unknown_dir() {
    let rust = temp_dir("stack-rust");
    touch(&rust, "Cargo.toml");
    assert!(detect_tech_stack(&rust).expect("scan").contains(&"Rust".to_string()));
    cleanup(&rust);

    let empty = temp_dir("stack-empty");
    assert!(detect_tech_stack(&empty).expect("scan").is_empty());
    cleanup(&empty);
}

// ---------- 活动 ----------

#[test]
fn activity_reports_newest_root_entry_mtime() {
    let dir = temp_dir("activity");
    touch(&dir, "old.txt");
    let old_time = std::time::SystemTime::now() - std::time::Duration::from_secs(3600);
    std::fs::File::options()
        .write(true)
        .open(dir.join("old.txt"))
        .expect("reopen")
        .set_modified(old_time)
        .expect("set mtime");
    touch(&dir, "new.txt");

    let activity = detect_activity(&dir).expect("scan");
    assert!(activity.last_modified_at.is_some());
    assert!(activity.last_scanned_at.ends_with('Z'));
    let modified = activity.last_modified_at.expect("mtime");
    let expected = windy_project_mgr_lib::project::store::now_utc();
    // 新文件的 mtime 接近当前时刻：日期部分应与今天一致。
    assert_eq!(&modified[..10], &expected[..10], "mtime should be today-ish");
    cleanup(&dir);
}

#[test]
fn activity_of_empty_dir_has_no_last_modified() {
    let dir = temp_dir("activity-empty");
    let activity = detect_activity(&dir).expect("scan");
    assert_eq!(activity.last_modified_at, None);
    cleanup(&dir);
}

// ---------- 启动脚本枚举（D5） ----------

#[test]
fn lists_all_scripts_sorted_start_then_run_then_alpha() {
    let dir = temp_dir("scripts");
    touch(&dir, "zeta.cmd");
    touch(&dir, "build.bat");
    touch(&dir, "run.ps1");
    touch(&dir, "start.bat");
    touch(&dir, "START-server.ps1");
    touch(&dir, "notes.txt"); // 非脚本，不得出现
    std::fs::create_dir(dir.join("sub")).expect("subdir");
    touch(&dir.join("sub"), "nested.bat"); // 不递归，不得出现

    let scripts = list_startup_scripts(&dir).expect("scan");
    let names: Vec<String> = scripts.iter().map(|s| s.name.clone()).collect();
    assert_eq!(
        names,
        vec![
            "START-server.ps1",
            "start.bat",
            "run.ps1",
            "build.bat",
            "zeta.cmd"
        ]
    );
    // 路径可用：指向真实文件。
    for s in &scripts {
        assert!(std::path::Path::new(&s.path).is_file(), "{} must exist", s.path);
    }
    cleanup(&dir);
}

#[test]
fn scripts_of_dir_without_any_is_empty() {
    let dir = temp_dir("scripts-none");
    touch(&dir, "package.json");
    assert!(list_startup_scripts(&dir).expect("scan").is_empty());
    cleanup(&dir);
}

#[test]
fn scripts_of_missing_path_errors() {
    let dir = temp_dir("scripts-missing");
    let err = list_startup_scripts(&dir.join("nope")).expect_err("missing must error");
    assert!(matches!(err, ScannerError::PathNotFound { .. }), "got: {err:?}");
    cleanup(&dir);
}
