//! 操作与设置 command 核心函数集成测试：真实临时数据目录（D6 / D12）。

use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::commands::actions::{
    get_settings_in, open_in_editor_in, update_settings_in,
};
use windy_project_mgr_lib::commands::project::portable_data_dir;
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
    assert_eq!(
        serde_json::to_value(&settings).expect("serialize"),
        json!({
            "colorMode": "system",
            "accentColor": {
                "kind": "preset",
                "value": "windy-teal"
            },
            "editor": {
                "executable": "",
                "arguments": ["{path}"]
            }
        })
    );
    cleanup(&dir);
}

#[test]
fn update_settings_persists_and_returns_saved_value() {
    let dir = temp_data_dir("update");
    let input: Settings = serde_json::from_value(json!({
        "colorMode": "dark",
        "accentColor": {
            "kind": "preset",
            "value": "coral"
        },
        "editor": {
            "executable": "C:/Tools/Cursor/Cursor.exe",
            "arguments": ["--reuse-window", "{path}"]
        }
    }))
    .expect("construct settings");
    let updated = update_settings_in(
        &dir,
        input.clone(),
    )
    .expect("update");
    assert_eq!(updated, input);

    let reloaded = get_settings_in(&dir).expect("reload");
    assert_eq!(reloaded, updated);

    let raw = std::fs::read_to_string(dir.join("settings.json")).expect("read");
    let persisted: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    assert_eq!(
        persisted,
        json!({
            "version": 2,
            "colorMode": "dark",
            "accentColor": {
                "kind": "preset",
                "value": "coral"
            },
            "editor": {
                "executable": "C:/Tools/Cursor/Cursor.exe",
                "arguments": ["--reuse-window", "{path}"]
            }
        })
    );
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
fn portable_settings_are_written_beside_executable() {
    let base = temp_data_dir("portable");
    let executable = base.join("windy-project-mgr.exe");
    let data_dir = portable_data_dir(&executable).expect("portable data dir");

    update_settings_in(&data_dir, Settings::default()).expect("write portable settings");

    assert_eq!(data_dir, base.join("data"));
    assert!(base.join("data").join("settings.json").is_file());
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

#[test]
fn open_in_editor_uses_full_editor_profile_from_settings() {
    let data_dir = temp_data_dir("editor-profile");
    let project_dir = data_dir.join("project folder");
    std::fs::create_dir_all(&project_dir).expect("create project dir");

    let settings: Settings = serde_json::from_value(json!({
        "colorMode": "dark",
        "accentColor": {
            "kind": "preset",
            "value": "coral"
        },
        "editor": {
            "executable": "open-editor.cmd",
            "arguments": ["{path}"]
        }
    }))
    .expect("construct settings");
    update_settings_in(&data_dir, settings).expect("persist settings");

    open_in_editor_in(&data_dir, &project_dir)
        .expect("action layer must pass the full editor profile");

    cleanup(&data_dir);
}
