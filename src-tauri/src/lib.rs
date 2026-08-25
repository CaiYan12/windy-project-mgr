pub mod commands;
pub mod git;
pub mod launch;
pub mod project;
pub mod scanner;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::project::get_projects,
            commands::project::get_project,
            commands::project::create_project,
            commands::project::update_project,
            commands::project::delete_project,
            commands::scan::scan_project,
            commands::scan::list_scripts,
            commands::actions::get_settings,
            commands::actions::update_settings,
            commands::actions::open_project,
            commands::actions::run_project,
            commands::actions::build_project,
            commands::actions::open_in_editor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
