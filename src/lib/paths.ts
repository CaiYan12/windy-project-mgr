// 路径纯逻辑：从项目路径取末段作为 name 自动填充（D10）。

/** 取路径末段：兼容 `/` 与 `\` 分隔符，忽略尾部分隔符；无有效段返回空串。 */
export function lastSegment(path: string): string {
  const parts = path.split(/[\\/]+/).filter((s) => s.length > 0);
  return parts.length > 0 ? parts[parts.length - 1] : "";
}
