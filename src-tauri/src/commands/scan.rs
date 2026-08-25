//! 扫描相关 command：`scan_project`（D2 前端并发调用）与 `list_scripts`（D5 引导）。
//!
//! 核心逻辑在 `scanner` 模块（可直接对临时目录测试）；此处仅薄封装。
//! `list_scripts` 为 D5 所需的最小增量：Add Dialog Step 2 需要在项目**登记前**
//! 对用户输入的路径枚举启动脚本，既有 command 面无法覆盖，故新增只读查询。

use crate::scanner::{list_startup_scripts, scan_project_path, ProjectMetadata, StartupScript};

#[tauri::command]
pub fn scan_project(path: String) -> Result<ProjectMetadata, String> {
    scan_project_path(std::path::Path::new(&path)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_scripts(path: String) -> Result<Vec<StartupScript>, String> {
    list_startup_scripts(std::path::Path::new(&path)).map_err(|e| e.to_string())
}
