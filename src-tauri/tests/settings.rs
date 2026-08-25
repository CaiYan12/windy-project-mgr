//! `settings.json` 集成测试：真实临时目录（D6 / D12）。

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::project::{Settings, StoreError};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p4-set-{}-{}-{n}",
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
fn load_missing_file_returns_defaults() {
    let dir = temp_dir("missing");
    let file = dir.join("settings.json");
    let settings = Settings::load(&file).expect("missing file loads defaults");
    assert_eq!(settings, Settings::default());
    assert_eq!(settings.editor_command, "");
    assert_eq!(settings.theme, "system");
    cleanup(&dir);
}

#[test]
fn load_empty_file_returns_corrupted_error() {
    let dir = temp_dir("empty");
    let file = dir.join("settings.json");
    std::fs::write(&file, "").expect("write");
    let err = Settings::load(&file).expect_err("empty file must be diagnosable");
    assert!(matches!(err, StoreError::Corrupted { .. }), "got: {err:?}");
    cleanup(&dir);
}

#[test]
fn load_invalid_json_returns_corrupted_error() {
    let dir = temp_dir("badjson");
    let file = dir.join("settings.json");
    std::fs::write(&file, "###").expect("write");
    let err = Settings::load(&file).expect_err("invalid JSON must be diagnosable");
    assert!(matches!(err, StoreError::Corrupted { .. }), "got: {err:?}");
    cleanup(&dir);
}

#[test]
fn load_wrong_version_returns_version_mismatch() {
    let dir = temp_dir("version");
    let file = dir.join("settings.json");
    std::fs::write(
        &file,
        r#"{ "version": 7, "editorCommand": "code", "theme": "dark" }"#,
    )
    .expect("write");
    let err = Settings::load(&file).expect_err("unknown version must be rejected");
    assert!(
        matches!(err, StoreError::VersionMismatch { found: 7, expected: 1 }),
        "got: {err:?}"
    );
    cleanup(&dir);
}

#[test]
fn save_and_load_roundtrip() {
    let dir = temp_dir("roundtrip");
    let file = dir.join("settings.json");
    let settings = Settings {
        editor_command: "cursor".to_string(),
        theme: "dark".to_string(),
    };
    settings.save(&file).expect("save");

    let raw = std::fs::read_to_string(&file).expect("read");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    assert_eq!(json["version"], 1);

    let loaded = Settings::load(&file).expect("reload");
    assert_eq!(loaded, settings);
    cleanup(&dir);
}

#[test]
fn save_failure_does_not_corrupt_existing_file() {
    let dir = temp_dir("failure");
    let file = dir.join("settings.json");
    let settings = Settings {
        editor_command: "code".to_string(),
        theme: "light".to_string(),
    };
    settings.save(&file).expect("initial save");
    let before = std::fs::read_to_string(&file).expect("read");

    std::fs::remove_file(&file).expect("remove");
    std::fs::create_dir(&file).expect("make target a dir");

    settings
        .save(&file)
        .expect_err("save must fail when replace is impossible");

    std::fs::remove_dir_all(&file).expect("cleanup dir");
    std::fs::write(&file, &before).expect("restore");
    let loaded = Settings::load(&file).expect("reload after failed save");
    assert_eq!(loaded, settings);
    cleanup(&dir);
}
