// Tauri IPC 封装：与 §2.5 command 面对应；错误统一为字符串（后端约定）。
// 注：get_settings / update_settings 的前端接入属 Phase 11（设置 Dialog）；
// 其余六个操作 command 后端已于 Phase 10 实现（D4 / D6）。

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
