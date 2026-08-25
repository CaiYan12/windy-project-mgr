// Tauri IPC 封装：与 §2.5 command 面对应；错误统一为字符串（后端约定）。
// 六个操作 command 后端已于 Phase 10 实现（D4 / D6）；`get_settings` /
// `update_settings` 前端封装于 Phase 11（D1 主题持久化 + D6 设置 Dialog）接入。

import { invoke } from "@tauri-apps/api/core";
import type {
  CreateProjectInput,
  Project,
  ProjectMetadata,
  StartupScript,
} from "../types/project";

/** 应用设置（D6 settings.json）：编辑器命令 + 主题选择。 */
export interface AppSettings {
  editorCommand: string;
  theme: string;
}

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

export function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export function updateSettings(settings: AppSettings): Promise<AppSettings> {
  return invoke<AppSettings>("update_settings", { settings });
}
