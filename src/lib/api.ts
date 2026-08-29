// Tauri IPC wrappers and the settings v3 wire contract.

import { invoke } from "@tauri-apps/api/core";
import type { AppearanceSettings } from "./appearance";
import type {
  CreateProjectInput,
  Project,
  ProjectMetadata,
  StartupScript,
} from "../types/project";

export type ColorMode = "system" | "light" | "dark";

export type AccentPresetId =
  | "windy-teal"
  | "ocean-blue"
  | "violet"
  | "amber"
  | "coral"
  | "rose";

export type AccentColor =
  | { kind: "preset"; value: AccentPresetId }
  | { kind: "windows" }
  | { kind: "custom"; value: string };

export interface EditorProfile {
  executable: string;
  arguments: string[];
}

/** Settings v3 payload returned by and sent to the Rust settings commands. */
export interface AppSettings {
  colorMode: ColorMode;
  accentColor: AccentColor;
  appearance: AppearanceSettings;
  editor: EditorProfile;
}

export type DetectionSource = "path" | "registry" | "standard";

export interface DetectedEditor {
  id: string;
  name: string;
  executable: string;
  source: DetectionSource;
}

export interface AppInfo {
  version: string;
  dataDir: string;
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

export function detectEditors(): Promise<DetectedEditor[]> {
  return invoke<DetectedEditor[]>("detect_editors");
}

export function getWindowsAccentColor(): Promise<string> {
  return invoke<string>("get_windows_accent_color");
}

export function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("get_app_info");
}
