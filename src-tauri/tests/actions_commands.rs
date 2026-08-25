//! 操作与设置 command 核心函数集成测试：真实临时数据目录（D6 / D12）。

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::commands::actions::{get_settings_in, update_settings_in};
use windy_project_mgr_lib::project::{Settings, StoreError};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_data_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p10-set-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn get_settings_returns_defaults_when_file_missing() {
    let dir = temp_data_dir("get-defaults");
    let settings = get_settings_in(&dir).expect("missing file loads defaults");
    assert_eq!(settings, Settings::default());
    cleanup(&dir);
}

#[test]
fn update_settings_persists_and_returns_saved_value() {
    let dir = temp_data_dir("update");
    let updated = update_settings_in(
        &dir,
        Settings {
            editor_command: "code".to_string(),
            theme: "dark".to_string(),
        },
    )
    .expect("update");
    assert_eq!(updated.editor_command, "code");
    assert_eq!(updated.theme, "dark");

    let reloaded = get_settings_in(&dir).expect("reload");
    assert_eq!(reloaded, updated);
    cleanup(&dir);
}

#[test]
fn update_settings_creates_data_dir_when_missing() {
    let base = temp_data_dir("create-dir");
    let dir = base.join("nested");
    update_settings_in(&dir, Settings::default()).expect("update creates dir");
    assert!(dir.join("settings.json").is_file());
    cleanup(&base);
}

#[test]
fn get_settings_corrupted_file_is_diagnosable() {
    let dir = temp_data_dir("corrupted");
    std::fs::write(dir.join("settings.json"), "###").expect("write");
    let err = get_settings_in(&dir).expect_err("corrupted must be diagnosable");
    assert!(matches!(err, StoreError::Corrupted { .. }), "got: {err:?}");
    cleanup(&dir);
}
