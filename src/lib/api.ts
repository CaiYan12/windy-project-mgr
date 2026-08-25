// Tauri IPC 封装：与 §2.5 command 面对应；错误统一为字符串（后端约定）。
// 注：run_project / build_project / open_project / open_in_editor /
// get_settings / update_settings 的后端属 Phase 10，此处先行封装供 UI 接入，
// 命令未注册时调用会以错误字符串返回并由界面提示（不崩溃）。

import { invoke } from "@tauri-apps/api/core";
import type {
  CreateProjectInput,
  Project,
  ProjectMetadata,
  StartupScript,
} from "../types/project";

export function getProjects(): Promise<Project[]> {
  return invoke<Project[]>("get_projects");
}

export function createProject(input: CreateProjectInput): Promise<Project> {
  return invoke<Project>("create_project", { input });
}

export function updateProject(project: Project): Promise<Project> {
  return invoke<Project>("update_project", { project });
}

export function deleteProject(id: string): Promise<void> {
  return invoke<void>("delete_project", { id });
}

export function scanProject(path: string): Promise<ProjectMetadata> {
  return invoke<ProjectMetadata>("scan_project", { path });
}

export function listScripts(path: string): Promise<StartupScript[]> {
  return invoke<StartupScript[]>("list_scripts", { path });
}

export function openProject(path: string): Promise<void> {
  return invoke<void>("open_project", { path });
}

export function runProject(path: string, command: string): Promise<void> {
  return invoke<void>("run_project", { path, command });
}

export function buildProject(path: string, command: string): Promise<void> {
  return invoke<void>("build_project", { path, command });
}

export function openInEditor(path: string): Promise<void> {
  return invoke<void>("open_in_editor", { path });
}
