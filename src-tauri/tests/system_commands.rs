//! System discovery and app info tests for Task 2.

use serde_json::json;
use std::collections::BTreeMap;
use std::io::ErrorKind;
use std::path::Path;
use windy_project_mgr_lib::commands::system::{
    classify_where_output, command_error_message, deduplicate_detected_editors,
    detect_path_editors_from_where_output, detect_registry_editors_from_blocks,
    finalize_editor_discovery, get_app_info_in, parse_registry_dword, parse_registry_query_blocks,
    resolve_windows_accent_candidates, windows_bgr_dword_to_css_hex, AppInfo, DetectedEditor,
    DetectionSource, RegistryQueryBlock,
};

fn make_block(key_path: &str, values: &[(&str, &str)]) -> RegistryQueryBlock {
    let mut map = BTreeMap::new();
    for (name, value) in values {
        map.insert((*name).to_string(), (*value).to_string());
    }
    RegistryQueryBlock {
        key_path: key_path.to_string(),
        values: map,
    }
}

#[test]
fn app_info_serializes_camel_case_fields() {
    let info = AppInfo {
        version: "0.1.0".to_string(),
        data_dir: r"D:\Apps\windy\data".to_string(),
    };

    assert_eq!(
        serde_json::to_value(info).expect("serialize"),
        json!({
            "version": "0.1.0",
            "dataDir": r"D:\Apps\windy\data"
        })
    );
}

#[test]
fn get_app_info_in_uses_current_package_version_and_given_data_dir() {
    let info = get_app_info_in(Path::new(r"D:\Portable\windy\data")).expect("app info");

    assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(info.data_dir, r"D:\Portable\windy\data");
}

#[test]
fn detected_editor_serializes_camel_case_fields() {
    let editor = DetectedEditor {
        id: "vscode".to_string(),
        name: "Visual Studio Code".to_string(),
        executable: r"C:\Users\Test\AppData\Local\Programs\Microsoft VS Code\Code.exe".to_string(),
        source: DetectionSource::Registry,
    };

    assert_eq!(
        serde_json::to_value(editor).expect("serialize"),
        json!({
            "id": "vscode",
            "name": "Visual Studio Code",
            "executable": r"C:\Users\Test\AppData\Local\Programs\Microsoft VS Code\Code.exe",
            "source": "registry"
        })
    );
}

#[test]
fn parse_registry_query_blocks_groups_key_value_blocks() {
    let raw = concat!(
        "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\VSCode\r\n",
        "    DisplayName    REG_SZ    Visual Studio Code\r\n",
        "    DisplayIcon    REG_SZ    C:\\Users\\Test\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe,0\r\n",
        "\r\n",
        "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Cursor\r\n",
        "    DisplayName    REG_SZ    Cursor\r\n",
        "    InstallLocation    REG_SZ    C:\\Users\\Test\\AppData\\Local\\Programs\\Cursor\r\n",
    );

    let blocks = parse_registry_query_blocks(raw);

    assert_eq!(blocks.len(), 2);
    assert_eq!(
        blocks[0].values.get("DisplayName").map(String::as_str),
        Some("Visual Studio Code")
    );
    assert_eq!(
        blocks[1].values.get("InstallLocation").map(String::as_str),
        Some(r"C:\Users\Test\AppData\Local\Programs\Cursor")
    );
}

#[test]
fn detect_registry_editors_from_blocks_recognizes_products_and_skips_missing_paths() {
    let blocks = vec![
        make_block(
            r"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Uninstall\VSCode",
            &[
                ("DisplayName", "Visual Studio Code"),
                (
                    "DisplayIcon",
                    r"C:\Users\Test\AppData\Local\Programs\Microsoft VS Code\Code.exe,0",
                ),
            ],
        ),
        make_block(
            r"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Uninstall\Missing",
            &[
                ("DisplayName", "Cursor"),
                (
                    "DisplayIcon",
                    r"C:\Users\Test\AppData\Local\Programs\Cursor\Cursor.exe,0",
                ),
            ],
        ),
    ];

    let editors = detect_registry_editors_from_blocks(&blocks, |path| {
        path == Path::new(r"C:\Users\Test\AppData\Local\Programs\Microsoft VS Code\Code.exe")
    });

    assert_eq!(
        editors,
        vec![DetectedEditor {
            id: "vscode".to_string(),
            name: "Visual Studio Code".to_string(),
            executable: r"C:\Users\Test\AppData\Local\Programs\Microsoft VS Code\Code.exe"
                .to_string(),
            source: DetectionSource::Registry,
        }]
    );
}

#[test]
fn detect_path_editors_from_where_output_returns_path_only_rows_for_existing_commands() {
    let editors =
        detect_path_editors_from_where_output("code", "C:\\Tools\\code.cmd\r\n", |path| {
            path == Path::new(r"C:\Tools\code.cmd")
        });

    assert_eq!(
        editors,
        vec![DetectedEditor {
            id: "path:code".to_string(),
            name: "PATH command: code".to_string(),
            executable: r"C:\Tools\code.cmd".to_string(),
            source: DetectionSource::Path,
        }]
    );
}

#[test]
fn detect_path_editors_from_where_output_collects_all_existing_paths_for_alias() {
    let editors = detect_path_editors_from_where_output(
        "code",
        "C:\\Tools\\code.cmd\r\nC:\\Tools\\code.exe\r\nC:\\Tools\\missing.cmd\r\n",
        |path| path == Path::new(r"C:\Tools\code.cmd") || path == Path::new(r"C:\Tools\code.exe"),
    );

    assert_eq!(
        editors,
        vec![
            DetectedEditor {
                id: "path:code".to_string(),
                name: "PATH command: code".to_string(),
                executable: r"C:\Tools\code.cmd".to_string(),
                source: DetectionSource::Path,
            },
            DetectedEditor {
                id: "path:code".to_string(),
                name: "PATH command: code".to_string(),
                executable: r"C:\Tools\code.exe".to_string(),
                source: DetectionSource::Path,
            },
        ]
    );
}

#[test]
fn detect_registry_editors_from_blocks_collects_multiple_existing_paths_for_same_product() {
    let blocks = vec![
        make_block(
            r"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Uninstall\VSCode-A",
            &[
                ("DisplayName", "Visual Studio Code"),
                ("DisplayIcon", r"C:\Apps\VSCode-A\Code.exe,0"),
            ],
        ),
        make_block(
            r"HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Uninstall\VSCode-B",
            &[
                ("DisplayName", "Visual Studio Code"),
                ("DisplayIcon", r"C:\Apps\VSCode-B\Code.exe,0"),
            ],
        ),
    ];

    let editors = detect_registry_editors_from_blocks(&blocks, |path| {
        path == Path::new(r"C:\Apps\VSCode-A\Code.exe")
            || path == Path::new(r"C:\Apps\VSCode-B\Code.exe")
    });

    assert_eq!(
        editors,
        vec![
            DetectedEditor {
                id: "vscode".to_string(),
                name: "Visual Studio Code".to_string(),
                executable: r"C:\Apps\VSCode-A\Code.exe".to_string(),
                source: DetectionSource::Registry,
            },
            DetectedEditor {
                id: "vscode".to_string(),
                name: "Visual Studio Code".to_string(),
                executable: r"C:\Apps\VSCode-B\Code.exe".to_string(),
                source: DetectionSource::Registry,
            },
        ]
    );
}

#[test]
fn deduplicate_detected_editors_prefers_product_evidence_and_ignores_case() {
    let deduped = deduplicate_detected_editors(vec![
        DetectedEditor {
            id: "path:code".to_string(),
            name: "PATH command: code".to_string(),
            executable: r"c:\Users\Test\AppData\Local\Programs\Microsoft VS Code\Code.exe"
                .to_string(),
            source: DetectionSource::Path,
        },
        DetectedEditor {
            id: "vscode".to_string(),
            name: "Visual Studio Code".to_string(),
            executable: r"C:\Users\Test\AppData\Local\Programs\Microsoft VS Code\Code.exe"
                .to_string(),
            source: DetectionSource::Standard,
        },
    ]);

    assert_eq!(deduped.len(), 1);
    assert_eq!(deduped[0].id, "vscode");
    assert_eq!(deduped[0].source, DetectionSource::Standard);
}

#[test]
fn parse_registry_dword_accepts_hex_and_decimal_forms() {
    assert_eq!(
        parse_registry_dword("0xFF534C2A (4283649066)"),
        Some(0xFF53_4C2A)
    );
    assert_eq!(parse_registry_dword("4283649066"), Some(0xFF53_4C2A));
}

#[test]
fn windows_bgr_dword_to_css_hex_uses_low_bgr_bytes() {
    assert_eq!(windows_bgr_dword_to_css_hex(0xFF53_4C2A), "#2A4C53");
}

#[test]
fn classify_where_output_treats_exit_code_one_as_absent_but_other_failures_as_errors() {
    assert_eq!(classify_where_output(Some(1), "", ""), Ok(String::new()));
    assert!(classify_where_output(Some(2), "", "where failed").is_err());
}

#[test]
fn resolve_windows_accent_candidates_uses_dwm_after_missing_first_value() {
    let resolved = resolve_windows_accent_candidates(&[
        Ok(None),
        Ok(Some("0xFF534C2A (4283649066)".to_string())),
    ])
    .expect("DWM fallback should work");

    assert_eq!(resolved, "#2A4C53");
}

#[test]
fn resolve_windows_accent_candidates_returns_retained_error_when_both_queries_fail() {
    let error = resolve_windows_accent_candidates(&[
        Err("reg.exe failed: spawn 1".to_string()),
        Err("reg.exe failed: spawn 2".to_string()),
    ])
    .expect_err("both query executions failed");

    assert!(error.contains("reg.exe failed: spawn 1"), "got: {error}");
}

#[test]
fn finalize_editor_discovery_returns_ok_empty_when_any_source_was_queryable() {
    let result = finalize_editor_discovery(
        Vec::new(),
        &[false, true, false],
        &[String::from("reg error")],
    )
    .expect("queryable source should suppress combined failure");

    assert!(result.is_empty());
}

#[test]
fn finalize_editor_discovery_returns_combined_error_only_when_no_source_was_queryable() {
    let error = finalize_editor_discovery(
        Vec::new(),
        &[false, false, false],
        &[String::from("reg error"), String::from("where error")],
    )
    .expect_err("no source was queryable");

    assert!(error.contains("reg error"), "got: {error}");
    assert!(error.contains("where error"), "got: {error}");
}

#[test]
fn command_error_message_mentions_missing_command() {
    let error = std::io::Error::new(ErrorKind::NotFound, "missing");
    let message = command_error_message("where.exe", &error);

    assert!(message.contains("where.exe"), "got: {message}");
    assert!(message.contains("missing"), "got: {message}");
}
