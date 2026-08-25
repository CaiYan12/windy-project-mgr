// 前端领域类型：与 `docs/PLAN.MD` §2.2 / §2.3 及 Rust 侧 camelCase 序列化对应。

/** 持久化的项目记录（§2.2；运行时扫描字段禁止加入）。 */
export interface Project {
  id: string;
  name: string;
  path: string;
  description?: string;
  tags: string[];
  runCommand?: string;
  buildCommand?: string;
  createdAt: string;
}

/** Add Dialog 提交的项目输入（id / createdAt 由后端生成）。 */
export interface CreateProjectInput {
  name: string;
  path: string;
  description?: string;
  tags: string[];
  runCommand?: string;
  buildCommand?: string;
}

export interface GitCommit {
  hash: string;
  message: string;
  author: string;
  date: string;
}

export type GitStatus = "clean" | "modified" | "unknown";

export interface GitMetadata {
  branch: string;
  status: GitStatus;
  changedFiles: number;
  ahead: number;
  behind: number;
  lastCommit: GitCommit | null;
  recentCommits: GitCommit[];
}

export interface ActivityMetadata {
  lastModifiedAt: string | null;
  lastScannedAt: string;
}

/** 运行时扫描结果（§2.3，仅存前端内存，D2）。 */
export interface ProjectMetadata {
  projectType: string | null;
  techStack: string[];
  git: GitMetadata | null;
  activity: ActivityMetadata;
}

/** 根目录启动脚本候选（D5）。 */
export interface StartupScript {
  name: string;
  path: string;
}

/** 单项目扫描状态（D2：骨架先行，结果到达后逐卡填充）。 */
export type ScanState =
  | { status: "loading" }
  | { status: "ok"; data: ProjectMetadata }
  | { status: "error"; message: string };
