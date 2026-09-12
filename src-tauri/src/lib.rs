pub mod commands;
pub mod git;
pub mod launch;
pub mod project;
pub mod scanner;

use project::{JsonHandle, Settings, Store};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 数据目录固定为 EXE 相邻的 `data`（ADR 0004）；进程内加载一次并缓存，
            // 之后所有读写经同一把锁，消除并发命令之间的丢失更新（ADR 0006）。
            let data_dir = commands::project::app_data_dir()?;
            app.manage(JsonHandle::<Store>::new(commands::project::projects_file(
                &data_dir,
            )));
            app.manage(JsonHandle::<Settings>::new(commands::actions::settings_file(
                &data_dir,
            )));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::project::get_projects,
            commands::project::get_project,
            commands::project::create_project,
            commands::project::update_project,
            commands::project::delete_project,
            commands::project::check_path_available,
            commands::scan::scan_project,
            commands::scan::list_scripts,
            commands::actions::get_settings,
            commands::actions::update_settings,
            commands::actions::open_project,
            commands::actions::run_project,
            commands::actions::build_project,
            commands::actions::open_in_editor,
            commands::system::detect_editors,
            commands::system::get_windows_accent_color,
            commands::system::get_app_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
