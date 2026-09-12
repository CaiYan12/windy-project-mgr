//! Project CRUD command 层（5 个 Tauri command 中的项目部分）。
//!
//! 逻辑分三层：
//! - `*_in_store`：对 `&mut Store` 的核心操作（供命令层在共享句柄的锁内调用）；
//! - `*_in`：`data_dir` 参数化的内核，可在临时目录中直接测试；
//! - `#[tauri::command]`：从 `State<JsonHandle<Store>>` 取单一写者句柄（ADR 0006）。

use crate::project::dedup::{is_absolute_path, normalize_path};
use crate::project::handle::JsonHandle;
use crate::project::store::{new_id, now_utc};
use crate::project::{Project, Store, StoreError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::State;

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

/// 路径可用性三态（A2/B4 + B3 修复）：绝对路径判定与查重结果一次返回，
/// 使前端 Step 1 能在单次调用内区分「可用 / 重复 / 非绝对」，保持后端为唯一事实源（ADR 0007）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum PathAvailability {
    Available,
    Duplicate { path: String },
    NotAbsolute,
}

/// 便携数据目录：当前可执行文件同目录下的 `data`。
pub fn portable_data_dir(executable_path: &Path) -> Result<PathBuf, StoreError> {
    let executable_dir = executable_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| {
            StoreError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "executable directory is unavailable",
            ))
        })?;
    Ok(executable_dir.join("data"))
}

/// 当前运行实例的数据目录；所有运行形态统一使用 EXE 相邻的 `data`。
pub fn app_data_dir() -> Result<PathBuf, StoreError> {
    let executable_path = std::env::current_exe().map_err(StoreError::Io)?;
    portable_data_dir(&executable_path)
}

pub fn projects_file(data_dir: &Path) -> PathBuf {
    data_dir.join("projects.json")
}

// ---------- 对 &mut Store 的核心操作（命令层与参数化内核共用） ----------

/// 新增项目到给定 Store：路径先规范化，查重命中（D3）拒绝并返回 DuplicatePath。
pub fn create_in_store(
    store: &mut Store,
    input: CreateProjectInput,
) -> Result<Project, StoreError> {
    require_absolute_path(&input.path)?;
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
    Ok(project)
}

/// 更新给定 Store 中的项目：查重排除自身；目标不存在返回 NotFound。
pub fn update_in_store(store: &mut Store, project: Project) -> Result<Project, StoreError> {
    require_absolute_path(&project.path)?;
    let mut normalized = project;
    normalized.path = normalize_path(&normalized.path);
    if let Some(existing) = store.find_duplicate_by_path(&normalized.path, Some(&normalized.id)) {
        return Err(StoreError::DuplicatePath {
            path: existing.path.clone(),
        });
    }
    store.update(normalized.clone())?;
    Ok(normalized)
}

/// 从给定 Store 删除项目记录（永不删除项目目录）。
pub fn delete_in_store(store: &mut Store, id: &str) -> Result<(), StoreError> {
    store.delete(id).map(|_removed| ())
}

/// 入口路径约束（ADR 0007）：仅接受绝对路径，返回可诊断错误。
fn require_absolute_path(path: &str) -> Result<(), StoreError> {
    if is_absolute_path(path) {
        Ok(())
    } else {
        Err(StoreError::Validation {
            detail: format!("path must be absolute: {path}"),
        })
    }
}

// ---------- data_dir 参数化内核（集成测试用） ----------

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
    let project = create_in_store(&mut store, input)?;
    store.save(&file)?;
    Ok(project)
}

/// 更新项目：查重排除自身；目标不存在返回 NotFound。
pub fn update_project_in(data_dir: &Path, project: Project) -> Result<Project, StoreError> {
    let file = projects_file(data_dir);
    let mut store = Store::load(&file)?;
    let normalized = update_in_store(&mut store, project)?;
    store.save(&file)?;
    Ok(normalized)
}

/// 删除项目：仅删记录，永不删除项目目录。
pub fn delete_project_in(data_dir: &Path, id: &str) -> Result<(), StoreError> {
    let file = projects_file(data_dir);
    let mut store = Store::load(&file)?;
    delete_in_store(&mut store, id)?;
    store.save(&file)?;
    Ok(())
}

/// 路径可用性只读查询（D3 / ADR 0007 / B3）：先判绝对路径，再查重。
/// `exclude_id` 供编辑场景排除自身。
pub fn check_path_available_in(
    data_dir: &Path,
    path: &str,
    exclude_id: Option<&str>,
) -> Result<PathAvailability, StoreError> {
    if !is_absolute_path(path) {
        return Ok(PathAvailability::NotAbsolute);
    }
    let store = Store::load(&projects_file(data_dir))?;
    Ok(match store.find_duplicate_by_path(path, exclude_id) {
        Some(existing) => PathAvailability::Duplicate {
            path: existing.path.clone(),
        },
        None => PathAvailability::Available,
    })
}

// ---------- Tauri command 薄封装（共享单一写者句柄） ----------

#[tauri::command]
pub fn get_projects(state: State<'_, JsonHandle<Store>>) -> Result<Vec<Project>, String> {
    state
        .read(|store| store.projects.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_project(state: State<'_, JsonHandle<Store>>, id: String) -> Result<Project, String> {
    let found = state
        .read(|store| store.get(&id).cloned())
        .map_err(|e| e.to_string())?;
    found.ok_or_else(|| StoreError::NotFound { id }.to_string())
}

#[tauri::command]
pub fn create_project(
    state: State<'_, JsonHandle<Store>>,
    input: CreateProjectInput,
) -> Result<Project, String> {
    state
        .write(|store| create_in_store(store, input))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_project(
    state: State<'_, JsonHandle<Store>>,
    project: Project,
) -> Result<Project, String> {
    state
        .write(|store| update_in_store(store, project))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_project(state: State<'_, JsonHandle<Store>>, id: String) -> Result<(), String> {
    state
        .write(|store| delete_in_store(store, &id))
        .map_err(|e| e.to_string())
}

/// 路径可用性只读查询（D3 / ADR 0007 / B3）：返回三态供 Step 1 一步判定。
#[tauri::command]
pub fn check_path_available(
    state: State<'_, JsonHandle<Store>>,
    path: String,
    exclude_id: Option<String>,
) -> Result<PathAvailability, String> {
    state
        .read(|store| {
            if !is_absolute_path(&path) {
                return PathAvailability::NotAbsolute;
            }
            match store.find_duplicate_by_path(&path, exclude_id.as_deref()) {
                Some(existing) => PathAvailability::Duplicate {
                    path: existing.path.clone(),
                },
                None => PathAvailability::Available,
            }
        })
        .map_err(|e| e.to_string())
}
