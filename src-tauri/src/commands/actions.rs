//! 操作与设置 command 层（Phase 10，D4 / D6）。
//!
//! 设置核心以 `data_dir` 参数化（`*_in` 函数），可在临时目录中直接测试；
//! `#[tauri::command]` 封装仅负责解析应用数据目录并转换错误为字符串。
//! 启动动作委托 `launch` 模块（D4 detached 语义，只报启动成败）。

use crate::launch;
use crate::project::Settings;
use std::path::{Path, PathBuf};

pub fn settings_file(data_dir: &Path) -> PathBuf {
    data_dir.join("settings.json")
}

pub fn get_settings_in(data_dir: &Path) -> Result<Settings, crate::project::StoreError> {
    Settings::load(&settings_file(data_dir))
}

pub fn update_settings_in(
    data_dir: &Path,
    settings: Settings,
) -> Result<Settings, crate::project::StoreError> {
    settings.save(&settings_file(data_dir))?;
    Ok(settings)
}

pub fn open_in_editor_in(data_dir: &Path, path: &Path) -> Result<(), String> {
    let settings = get_settings_in(data_dir).map_err(|e| e.to_string())?;
    launch::open_in_editor(&settings.editor, path).map_err(|e| e.to_string())
}

// ---------- Tauri command 薄封装 ----------

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    get_settings_in(&super::project::app_data_dir().map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_settings(settings: Settings) -> Result<Settings, String> {
    update_settings_in(
        &super::project::app_data_dir().map_err(|e| e.to_string())?,
        settings,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_project(path: String) -> Result<(), String> {
    launch::open_dir(Path::new(&path)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn run_project(path: String, command: String) -> Result<(), String> {
    launch::run_in_terminal(Path::new(&path), &command).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn build_project(path: String, command: String) -> Result<(), String> {
    launch::run_in_terminal(Path::new(&path), &command).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_in_editor(path: String) -> Result<(), String> {
    let data_dir = super::project::app_data_dir().map_err(|e| e.to_string())?;
    open_in_editor_in(&data_dir, Path::new(&path))
}
