// Search 纯逻辑（D7）：name / description / tags / path 四字段，
// 大小写不敏感子串匹配，纯前端即时过滤，无防抖、无索引。

import type { Project } from "../types/project";

export function filterProjects(projects: Project[], query: string): Project[] {
  const q = query.trim().toLowerCase();
  if (!q) {
    return projects;
  }
  return projects.filter((p) => {
    return (
      p.name.toLowerCase().includes(q) ||
      (p.description ?? "").toLowerCase().includes(q) ||
      p.tags.some((t) => t.toLowerCase().includes(q)) ||
      p.path.toLowerCase().includes(q)
    );
  });
}
