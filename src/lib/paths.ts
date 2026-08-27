// 路径纯逻辑：从项目路径取末段作为 name 自动填充（D10）；
// Windows 语义的路径相等（D3，与后端 `project::dedup` 口径一致）。

/** 取路径末段：兼容 `/` 与 `\` 分隔符，忽略尾部分隔符；无有效段返回空串。 */
export function lastSegment(path: string): string {
  const parts = path.split(/[\\/]+/).filter((s) => s.length > 0);
  return parts.length > 0 ? parts[parts.length - 1] : "";
}

/** Windows 路径相等（D3）：分隔符统一、去尾分隔符后大小写不敏感比较。
 *  文件选择器保证输入为绝对路径；与后端相比不复制词法 `.`/`..` 解析与
 *  相对路径绝对化（Add/Edit 的路径来自 Browse 选择或用户手输绝对路径）。 */
export function samePath(a: string, b: string): boolean {
  const norm = (s: string) => s.replace(/[\\/]+/g, "/").replace(/\/+$/, "").toLowerCase();
  return norm(a) === norm(b);
}
