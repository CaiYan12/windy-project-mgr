//! 真实目录验证（票面要求）：在本仓库实际目录上运行 Scanner。
//! `CARGO_MANIFEST_DIR` = src-tauri；其父目录即仓库根（Node 项目）。

use std::path::{Path, PathBuf};
use windy_project_mgr_lib::scanner::{
    detect_activity, detect_project_type, detect_tech_stack, list_startup_scripts,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri parent")
        .to_path_buf()
}

#[test]
fn repo_root_is_a_node_project() {
    let root = repo_root();
    assert_eq!(
        detect_project_type(&root).expect("scan"),
        Some("Node".to_string())
    );
    let stack = detect_tech_stack(&root).expect("scan");
    for label in ["Node", "pnpm", "TypeScript", "Vite"] {
        assert!(stack.contains(&label.to_string()), "missing {label} in {stack:?}");
    }
    let activity = detect_activity(&root).expect("scan");
    assert!(activity.last_modified_at.is_some(), "repo root has files");
}

#[test]
fn src_tauri_is_a_rust_project() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_eq!(
        detect_project_type(dir).expect("scan"),
        Some("Rust".to_string())
    );
    let stack = detect_tech_stack(dir).expect("scan");
    assert!(stack.contains(&"Rust".to_string()), "stack: {stack:?}");
    // src-tauri 当前无启动脚本：枚举应为空且不报错。
    assert!(list_startup_scripts(dir).expect("scan").is_empty());
}
