//! System discovery and app info commands for settings v2.
//!
//! B1：本文件只做编排与 `#[tauri::command]` 壳；纯逻辑拆到子模块：
//! `registry`（注册表解析与查询）、`editors`（编辑器发现）、`accent`（强调色）、
//! `exec`（子进程捕获）。

mod accent;
mod editors;
mod exec;
mod registry;

use serde::Serialize;
use std::path::Path;

use editors::{
    detect_standard_editors_in_roots, query_path_editors, standard_search_roots,
    strip_verbatim_prefix,
};
use registry::{query_registry_value, query_uninstall_registry_blocks};

// 对外（含集成测试）保持原有 `commands::system::*` 名称面。
pub use accent::{resolve_windows_accent_candidates, windows_bgr_dword_to_css_hex};
pub use editors::{
    deduplicate_detected_editors, detect_path_editors_from_where_output,
    detect_registry_editors_from_blocks, finalize_editor_discovery, DetectedEditor, DetectionSource,
};
pub use exec::{classify_where_output, command_error_message};
pub use registry::{parse_registry_dword, parse_registry_query_blocks, RegistryQueryBlock};

const WINDOWS_ONLY_ERROR: &str = "system discovery is only supported on Windows";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub data_dir: String,
}

pub fn get_app_info_in(data_dir: &Path) -> Result<AppInfo, String> {
    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        data_dir: strip_verbatim_prefix(&data_dir.display().to_string()),
    })
}

#[tauri::command]
pub fn get_app_info() -> Result<AppInfo, String> {
    let data_dir = super::project::app_data_dir().map_err(|e| e.to_string())?;
    get_app_info_in(&data_dir)
}

#[tauri::command]
pub fn detect_editors() -> Result<Vec<DetectedEditor>, String> {
    if !cfg!(windows) {
        return Err(WINDOWS_ONLY_ERROR.to_string());
    }

    let mut editors = Vec::new();
    let mut source_queried = vec![false, false, false];
    let mut errors = Vec::new();

    let (registry_blocks, registry_queried, registry_errors) = query_uninstall_registry_blocks();
    source_queried[0] = registry_queried;
    errors.extend(registry_errors);
    editors.extend(detect_registry_editors_from_blocks(&registry_blocks, |path| {
        path.is_file()
    }));

    let standard_roots = standard_search_roots();
    source_queried[1] = !standard_roots.is_empty();
    editors.extend(detect_standard_editors_in_roots(&standard_roots, |path| {
        path.is_file()
    }));

    let (path_editors, path_queried, path_errors) = query_path_editors();
    source_queried[2] = path_queried;
    errors.extend(path_errors);
    editors.extend(path_editors);

    finalize_editor_discovery(editors, &source_queried, &errors)
}

#[tauri::command]
pub fn get_windows_accent_color() -> Result<String, String> {
    if !cfg!(windows) {
        return Err(WINDOWS_ONLY_ERROR.to_string());
    }

    let registry_targets = [
        (
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Accent",
            "AccentColorMenu",
        ),
        (r"HKCU\Software\Microsoft\Windows\DWM", "AccentColor"),
    ];

    let candidates = registry_targets
        .iter()
        .map(|(key_path, value_name)| query_registry_value(key_path, value_name))
        .collect::<Vec<_>>();

    resolve_windows_accent_candidates(&candidates)
}
