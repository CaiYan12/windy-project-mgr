//! `settings.json` 集成测试：真实临时目录（Settings v2 / D12）。

use serde_json::json;
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

fn read_json(path: &PathBuf) -> serde_json::Value {
    let raw = std::fs::read_to_string(path).expect("read");
    serde_json::from_str(&raw).expect("valid json")
}

#[test]
fn load_missing_file_returns_v2_defaults() {
    let dir = temp_dir("missing");
    let file = dir.join("settings.json");
    let settings = Settings::load(&file).expect("missing file loads defaults");
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
fn load_invalid_v2_accent_color_is_diagnosable() {
    let dir = temp_dir("invalid-accent");
    let file = dir.join("settings.json");
    std::fs::write(
        &file,
        r##"{
  "version": 2,
  "colorMode": "system",
  "accentColor": { "kind": "custom", "value": "#12abz9" },
  "editor": { "executable": "", "arguments": ["{path}"] }
}"##,
    )
    .expect("write");

    let err = Settings::load(&file).expect_err("invalid accent must be rejected");
    assert!(
        err.to_string().contains("accentColor"),
        "expected accentColor detail, got: {err}"
    );
    cleanup(&dir);
}

#[test]
fn load_windows_accent_with_persisted_value_is_diagnosable() {
    let dir = temp_dir("invalid-windows-accent");
    let file = dir.join("settings.json");
    std::fs::write(
        &file,
        r##"{
  "version": 2,
  "colorMode": "system",
  "accentColor": { "kind": "windows", "value": "#123456" },
  "editor": { "executable": "", "arguments": ["{path}"] }
}"##,
    )
    .expect("write");

    let err = Settings::load(&file).expect_err("windows accent must not persist a value");
    assert!(
        err.to_string().contains("unknown field") || err.to_string().contains("accentColor"),
        "expected exact tagged-union failure, got: {err}"
    );
    cleanup(&dir);
}

#[test]
fn load_invalid_v2_editor_arguments_are_diagnosable() {
    let dir = temp_dir("invalid-editor");
    let file = dir.join("settings.json");
    std::fs::write(
        &file,
        r#"{
  "version": 2,
  "colorMode": "dark",
  "accentColor": { "kind": "preset", "value": "violet" },
  "editor": {
    "executable": "C:/Tools/Code.exe",
    "arguments": ["--reuse-window", "--folder-uri"]
  }
}"#,
    )
    .expect("write");

    let err = Settings::load(&file).expect_err("missing {path} must be rejected");
    assert!(
        err.to_string().contains("{path}"),
        "expected {{path}} detail, got: {err}"
    );
    cleanup(&dir);
}

#[test]
fn save_rejects_double_quote_arguments_for_batch_editors() {
    let dir = temp_dir("invalid-batch-quote-save");
    let settings: Settings = serde_json::from_value(json!({
        "colorMode": "dark",
        "accentColor": {
            "kind": "preset",
            "value": "violet"
        },
        "editor": {
            "executable": "C:/Tools/Open Editor.BaT",
            "arguments": [r#"--title="hello world""#, "{path}"]
        }
    }))
    .expect("construct settings");

    let err = settings
        .save(&dir.join("settings.json"))
        .expect_err("batch arguments with double quotes must be rejected");
    match err {
        StoreError::Validation { detail } => assert_eq!(
            detail,
            "cmd.exe batch arguments cannot contain the double quote character"
        ),
        other => panic!("expected Validation, got {other:?}"),
    }
    cleanup(&dir);
}

#[test]
fn load_rejects_double_quote_arguments_for_batch_editors() {
    let dir = temp_dir("invalid-batch-quote-load");
    let file = dir.join("settings.json");
    std::fs::write(
        &file,
        r##"{
  "version": 2,
  "colorMode": "dark",
  "accentColor": { "kind": "preset", "value": "violet" },
  "editor": {
    "executable": "open-editor.cMd",
    "arguments": ["--title=\"hello world\"", "{path}"]
  }
}"##,
    )
    .expect("write");

    let err = Settings::load(&file).expect_err("batch arguments with double quotes must be rejected");
    assert!(
        err.to_string()
            .contains("cmd.exe batch arguments cannot contain the double quote character"),
        "unexpected error: {err}"
    );
    cleanup(&dir);
}

#[test]
fn save_allows_double_quote_arguments_for_direct_executables() {
    let dir = temp_dir("direct-exe-quote");
    let settings: Settings = serde_json::from_value(json!({
        "colorMode": "dark",
        "accentColor": {
            "kind": "preset",
            "value": "violet"
        },
        "editor": {
            "executable": "C:/Tools/code.exe",
            "arguments": [r#"--title="hello world""#, "{path}"]
        }
    }))
    .expect("construct settings");

    settings
        .save(&dir.join("settings.json"))
        .expect("direct executable arguments may contain double quotes");
    cleanup(&dir);
}

#[test]
fn load_wrong_version_returns_version_mismatch() {
    let dir = temp_dir("version");
    let file = dir.join("settings.json");
    std::fs::write(
        &file,
        r#"{ "version": 7, "colorMode": "system", "accentColor": { "kind": "preset", "value": "windy-teal" }, "editor": { "executable": "", "arguments": ["{path}"] } }"#,
    )
    .expect("write");
    let err = Settings::load(&file).expect_err("unknown version must be rejected");
    assert!(
        matches!(err, StoreError::VersionMismatch { found: 7, expected: 2 }),
        "got: {err:?}"
    );
    cleanup(&dir);
}

#[test]
fn load_valid_v1_file_migrates_and_rewrites_v2_payload() {
    let dir = temp_dir("migrate-v1");
    let file = dir.join("settings.json");
    std::fs::write(
        &file,
        r#"{ "version": 1, "editorCommand": "cursor", "theme": "dark" }"#,
    )
    .expect("write");

    let settings = Settings::load(&file).expect("valid v1 must migrate");
    assert_eq!(
        serde_json::to_value(&settings).expect("serialize"),
        json!({
            "colorMode": "dark",
            "accentColor": {
                "kind": "preset",
                "value": "windy-teal"
            },
            "editor": {
                "executable": "cursor",
                "arguments": ["{path}"]
            }
        })
    );

    let rewritten = read_json(&file);
    assert_eq!(
        rewritten,
        json!({
            "version": 2,
            "colorMode": "dark",
            "accentColor": {
                "kind": "preset",
                "value": "windy-teal"
            },
            "editor": {
                "executable": "cursor",
                "arguments": ["{path}"]
            }
        })
    );
    cleanup(&dir);
}

#[test]
fn load_valid_v1_file_preserves_original_bytes_when_migration_temp_path_is_blocked() {
    let dir = temp_dir("migrate-v1-temp-blocked");
    let file = dir.join("settings.json");
    let before = r#"{ "version": 1, "editorCommand": "code", "theme": "light" }"#;
    std::fs::write(&file, before).expect("write v1");

    let temp_target = file.with_extension(format!(
        "{}.tmp.{}",
        file.extension().and_then(|ext| ext.to_str()).unwrap_or(""),
        std::process::id()
    ));
    std::fs::create_dir(&temp_target).expect("block atomic temp path");

    let err = Settings::load(&file).expect_err("migration write must fail when temp path is blocked");
    assert!(matches!(err, StoreError::Io(_)), "got: {err:?}");

    let after = std::fs::read(&file).expect("read original bytes");
    assert_eq!(after, before.as_bytes());

    std::fs::remove_dir_all(&temp_target).expect("cleanup temp blocker");
    cleanup(&dir);
}

#[test]
fn save_and_load_roundtrip_uses_flat_v2_payload() {
    let dir = temp_dir("roundtrip");
    let file = dir.join("settings.json");
    let settings: Settings = serde_json::from_value(json!({
        "colorMode": "light",
        "accentColor": {
            "kind": "custom",
            "value": "#1234AB"
        },
        "editor": {
            "executable": "C:/Program Files/Zed/zed.exe",
            "arguments": ["--add", "{path}"]
        }
    }))
    .expect("construct settings");
    settings.save(&file).expect("save");

    let json = read_json(&file);
    assert_eq!(
        json,
        json!({
            "version": 2,
            "colorMode": "light",
            "accentColor": {
                "kind": "custom",
                "value": "#1234AB"
            },
            "editor": {
                "executable": "C:/Program Files/Zed/zed.exe",
                "arguments": ["--add", "{path}"]
            }
        })
    );

    let loaded = Settings::load(&file).expect("reload");
    assert_eq!(loaded, settings);
    cleanup(&dir);
}

#[test]
fn save_failure_does_not_corrupt_existing_file() {
    let dir = temp_dir("failure");
    let file = dir.join("settings.json");
    let settings: Settings = serde_json::from_value(json!({
        "colorMode": "light",
        "accentColor": {
            "kind": "preset",
            "value": "amber"
        },
        "editor": {
            "executable": "code",
            "arguments": ["{path}"]
        }
    }))
    .expect("construct settings");
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

#[test]
fn save_and_load_roundtrip_supports_windows_accent_without_value() {
    let dir = temp_dir("windows-roundtrip");
    let file = dir.join("settings.json");
    let settings: Settings = serde_json::from_value(json!({
        "colorMode": "dark",
        "accentColor": {
            "kind": "windows"
        },
        "editor": {
            "executable": "",
            "arguments": ["{path}"]
        }
    }))
    .expect("construct settings");
    settings.save(&file).expect("save");

    let json = read_json(&file);
    assert_eq!(
        json,
        json!({
            "version": 2,
            "colorMode": "dark",
            "accentColor": {
                "kind": "windows"
            },
            "editor": {
                "executable": "",
                "arguments": ["{path}"]
            }
        })
    );

    let loaded = Settings::load(&file).expect("reload");
    assert_eq!(loaded, settings);
    cleanup(&dir);
}
