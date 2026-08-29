//! `settings.json` 集成测试：真实临时目录（Settings v3 主题工坊 / D12）。

use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::project::settings::{AppearanceSettings, ColorMode};
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

fn default_appearance_json() -> serde_json::Value {
    json!({
        "stylePreset": "windy",
        "radius": 10,
        "fontSize": 14,
        "density": "comfortable",
        "fontFamily": "",
        "neutrals": {
            "light": {
                "bg": "#f5f6f8",
                "surface": "#ffffff",
                "sunken": "#eceef2",
                "line": "#dfe3e8",
                "ink": "#17191f",
                "muted": "#6a7280"
            },
            "dark": {
                "bg": "#121417",
                "surface": "#1a1d22",
                "sunken": "#23272e",
                "line": "#2e343c",
                "ink": "#e8eaee",
                "muted": "#949ca8"
            }
        }
    })
}

#[test]
fn load_missing_file_returns_v3_defaults() {
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
            "appearance": default_appearance_json(),
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
        matches!(err, StoreError::VersionMismatch { found: 7, expected: 3 }),
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
            "appearance": default_appearance_json(),
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
            "version": 3,
            "colorMode": "dark",
            "accentColor": {
                "kind": "preset",
                "value": "windy-teal"
            },
            "appearance": default_appearance_json(),
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
fn save_and_load_roundtrip_uses_flat_v3_payload() {
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
            "version": 3,
            "colorMode": "light",
            "accentColor": {
                "kind": "custom",
                "value": "#1234AB"
            },
            "appearance": default_appearance_json(),
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
            "version": 3,
            "colorMode": "dark",
            "accentColor": {
                "kind": "windows"
            },
            "appearance": default_appearance_json(),
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

#[test]
fn load_valid_v2_file_migrates_to_v3_with_default_appearance() {
    let dir = temp_dir("migrate-v2");
    let file = dir.join("settings.json");
    std::fs::write(
        &file,
        r#"{
  "version": 2,
  "colorMode": "light",
  "accentColor": { "kind": "preset", "value": "amber" },
  "editor": { "executable": "code", "arguments": ["{path}"] }
}"#,
    )
    .expect("write v2 file");

    let settings = Settings::load(&file).expect("valid v2 must migrate");
    assert_eq!(settings.color_mode, ColorMode::Light);
    assert_eq!(
        settings.appearance,
        AppearanceSettings::default(),
        "v2 migration must materialize the Windy default appearance"
    );

    let rewritten = read_json(&file);
    assert_eq!(rewritten["version"], json!(3));
    assert_eq!(rewritten["appearance"], default_appearance_json());
    assert_eq!(rewritten["accentColor"]["kind"], json!("preset"));
    assert_eq!(rewritten["accentColor"]["value"], json!("amber"));
    cleanup(&dir);
}

#[test]
fn v3_roundtrip_preserves_custom_appearance() {
    let dir = temp_dir("v3-appearance-roundtrip");
    let file = dir.join("settings.json");
    let settings: Settings = serde_json::from_value(json!({
        "colorMode": "dark",
        "accentColor": { "kind": "preset", "value": "violet" },
        "appearance": {
            "stylePreset": null,
            "radius": 0,
            "fontSize": 16,
            "density": "compact",
            "fontFamily": "\"Cascadia Code\", Consolas, monospace",
            "neutrals": {
                "light": {
                    "bg": "#010203", "surface": "#040506", "sunken": "#070809",
                    "line": "#0a0b0c", "ink": "#0d0e0f", "muted": "#101112"
                },
                "dark": {
                    "bg": "#111213", "surface": "#141516", "sunken": "#171819",
                    "line": "#1a1b1c", "ink": "#1d1e1f", "muted": "#202122"
                }
            }
        },
        "editor": { "executable": "", "arguments": ["{path}"] }
    }))
    .expect("construct custom-appearance settings");

    settings.save(&file).expect("save");
    let json = read_json(&file);
    assert_eq!(json["version"], json!(3));
    assert_eq!(json["appearance"]["stylePreset"], serde_json::Value::Null);
    assert_eq!(json["appearance"]["density"], json!("compact"));

    let loaded = Settings::load(&file).expect("reload");
    assert_eq!(loaded, settings);
    cleanup(&dir);
}

#[test]
fn load_rejects_out_of_range_appearance_values() {
    for (label, appearance) in [
        ("radius", json!({ "radius": 21 })),
        ("negative-radius", json!({ "radius": -1 })),
        ("font-size-low", json!({ "fontSize": 12 })),
        ("font-size-high", json!({ "fontSize": 17 })),
        ("style-preset", json!({ "stylePreset": "paper" })),
        ("font-family-control", json!({ "fontFamily": "bad\nfont" })),
    ] {
        let dir = temp_dir(&format!("invalid-appearance-{label}"));
        let file = dir.join("settings.json");
        let payload = json!({
            "version": 3,
            "colorMode": "system",
            "accentColor": { "kind": "preset", "value": "windy-teal" },
            "appearance": appearance,
            "editor": { "executable": "", "arguments": ["{path}"] }
        });
        std::fs::write(&file, serde_json::to_string(&payload).expect("serialize"))
            .expect("write v3 file");

        let err = Settings::load(&file).expect_err("invalid appearance must be rejected");
        assert!(
            matches!(err, StoreError::Corrupted { .. }),
            "[{label}] expected Corrupted, got: {err:?}"
        );
        cleanup(&dir);
    }
}

#[test]
fn load_rejects_non_hex_neutral_colors() {
    let dir = temp_dir("invalid-neutrals");
    let file = dir.join("settings.json");
    let payload = json!({
        "version": 3,
        "colorMode": "system",
        "accentColor": { "kind": "preset", "value": "windy-teal" },
        "appearance": {
            "stylePreset": null,
            "radius": 10,
            "fontSize": 14,
            "density": "comfortable",
            "fontFamily": "",
            "neutrals": {
                "light": {
                    "bg": "#f5f6f8", "surface": "#ffffff", "sunken": "#eceef2",
                    "line": "#dfe3e8", "ink": "#17191f", "muted": "gray"
                },
                "dark": {
                    "bg": "#121417", "surface": "#1a1d22", "sunken": "#23272e",
                    "line": "#2e343c", "ink": "#e8eaee", "muted": "#949ca8"
                }
            }
        },
        "editor": { "executable": "", "arguments": ["{path}"] }
    });
    std::fs::write(&file, serde_json::to_string(&payload).expect("serialize")).expect("write");

    let err = Settings::load(&file).expect_err("non-hex neutral must be rejected");
    assert!(
        err.to_string().contains("neutrals"),
        "expected neutrals detail, got: {err}"
    );
    cleanup(&dir);
}

#[test]
fn save_rejects_invalid_appearance_values() {
    let dir = temp_dir("save-invalid-appearance");
    let settings: Settings = serde_json::from_value(json!({
        "colorMode": "system",
        "accentColor": { "kind": "preset", "value": "windy-teal" },
        "appearance": {
            "stylePreset": "windy",
            "radius": 21,
            "fontSize": 14,
            "density": "comfortable",
            "fontFamily": "",
            "neutrals": default_appearance_json()["neutrals"]
        },
        "editor": { "executable": "", "arguments": ["{path}"] }
    }))
    .expect("construct settings");

    let err = settings
        .save(&dir.join("settings.json"))
        .expect_err("radius 21 must be rejected on save");
    match err {
        StoreError::Validation { detail } => {
            assert!(detail.contains("radius"), "got: {detail}");
        }
        other => panic!("expected Validation, got {other:?}"),
    }
    cleanup(&dir);
}
