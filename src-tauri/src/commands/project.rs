//! Project CRUD command 层（5 个 Tauri command 中的项目部分）。
//!
//! 核心逻辑以 `data_dir` 参数化（`*_in` 函数），可在临时目录中直接测试；
//! `#[tauri::command]` 封装仅负责解析应用数据目录并转换错误为字符串。

use crate::project::dedup::normalize_path;
use crate::project::store::{new_id, now_utc};
use crate::project::{Project, Store, StoreError};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Add Dialog 提交的项目输入（id / createdAt 由后端生成）。
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectInput {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub run_command: Option<String>,
    #[serde(default)]
    pub build_command: Option<String>,
}

/// 应用数据目录：`%APPDATA%\windy-project-mgr`。
pub fn app_data_dir() -> Result<PathBuf, StoreError> {
    let appdata = std::env::var("APPDATA").map_err(|_| {
        StoreError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "APPDATA environment variable is not set",
        ))
    })?;
    Ok(PathBuf::from(appdata).join("windy-project-mgr"))
}

pub fn projects_file(data_dir: &Path) -> PathBuf {
    data_dir.join("projects.json")
}

pub fn get_projects_in(data_dir: &Path) -> Result<Vec<Project>, StoreError> {
    Ok(Store::load(&projects_file(data_dir))?.projects)
}

pub fn get_project_in(data_dir: &Path, id: &str) -> Result<Project, StoreError> {
    let store = Store::load(&projects_file(data_dir))?;
    store
        .get(id)
        .cloned()
        .ok_or_else(|| StoreError::NotFound { id: id.to_string() })
}

/// 创建项目：路径先规范化，查重命中（D3）拒绝并返回 DuplicatePath。
pub fn create_project_in(
    data_dir: &Path,
    input: CreateProjectInput,
) -> Result<Project, StoreError> {
    let file = projects_file(data_dir);
    let mut store = Store::load(&file)?;
    let path = normalize_path(&input.path);
    if let Some(existing) = store.find_duplicate_by_path(&path, None) {
        return Err(StoreError::DuplicatePath {
            path: existing.path.clone(),
        });
    }
    let project = Project {
        id: new_id(),
        name: input.name,
        path,
        description: input.description,
        tags: input.tags,
        run_command: input.run_command,
        build_command: input.build_command,
        created_at: now_utc(),
    };
    store.create(project.clone())?;
    store.save(&file)?;
    Ok(project)
}

/// 更新项目：查重排除自身；目标不存在返回 NotFound。
pub fn update_project_in(data_dir: &Path, project: Project) -> Result<Project, StoreError> {
    let file = projects_file(data_dir);
    let mut store = Store::load(&file)?;
    let mut normalized = project;
    normalized.path = normalize_path(&normalized.path);
    if let Some(existing) = store.find_duplicate_by_path(&normalized.path, Some(&normalized.id)) {
        return Err(StoreError::DuplicatePath {
            path: existing.path.clone(),
        });
    }
    store.update(normalized.clone())?;
    store.save(&file)?;
    Ok(normalized)
}

/// 删除项目：仅删记录，永不删除项目目录。
pub fn delete_project_in(data_dir: &Path, id: &str) -> Result<(), StoreError> {
    let file = projects_file(data_dir);
    let mut store = Store::load(&file)?;
    store.delete(id)?;
    store.save(&file)?;
    Ok(())
}

// ---------- Tauri command 薄封装 ----------

#[tauri::command]
pub fn get_projects() -> Result<Vec<Project>, String> {
    get_projects_in(&app_data_dir().map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_project(id: String) -> Result<Project, String> {
    get_project_in(&app_data_dir().map_err(|e| e.to_string())?, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_project(input: CreateProjectInput) -> Result<Project, String> {
    create_project_in(&app_data_dir().map_err(|e| e.to_string())?, input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_project(project: Project) -> Result<Project, String> {
    update_project_in(&app_data_dir().map_err(|e| e.to_string())?, project).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_project(id: String) -> Result<(), String> {
    delete_project_in(&app_data_dir().map_err(|e| e.to_string())?, &id).map_err(|e| e.to_string())
}
