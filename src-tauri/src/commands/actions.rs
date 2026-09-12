//! 操作与设置 command 层（Phase 10，D4 / D6）。
//!
//! 设置核心以 `data_dir` 参数化（`*_in` 函数），可在临时目录中直接测试；
//! `#[tauri::command]` 封装从 `State<JsonHandle<Settings>>` 取单一写者句柄（ADR 0006）。
//! 启动动作委托 `launch` 模块（D4 detached 语义，只报启动成败）。

use crate::launch;
use crate::project::handle::JsonHandle;
use crate::project::Settings;
use std::path::{Path, PathBuf};
use tauri::State;

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

// ---------- Tauri command 薄封装（共享单一写者句柄） ----------

#[tauri::command]
pub fn get_settings(state: State<'_, JsonHandle<Settings>>) -> Result<Settings, String> {
    state
        .read(|settings| settings.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_settings(
    state: State<'_, JsonHandle<Settings>>,
    settings: Settings,
) -> Result<Settings, String> {
    let saved = settings.clone();
    state
        .write(move |slot| {
            *slot = saved;
            Ok(())
        })
        .map_err(|e| e.to_string())?;
    Ok(settings)
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
pub fn open_in_editor(
    state: State<'_, JsonHandle<Settings>>,
    path: String,
) -> Result<(), String> {
    let profile = state
        .read(|settings| settings.editor.clone())
        .map_err(|e| e.to_string())?;
    launch::open_in_editor(&profile, Path::new(&path)).map_err(|e| e.to_string())
}
