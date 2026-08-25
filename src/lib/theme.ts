// 主题纯逻辑（D1）：类型守卫、生效应用与编辑器预设（D6）。
// CSS 据根节点 `data-theme` 决定配色：`system` 交由 `prefers-color-scheme`
// 跟随系统（ADR 0003 在 `App.css` 中的选择器处理）。

export type Theme = "system" | "light" | "dark";

/** 主题选项文案（Settings Dialog 单选组）。 */
export const THEMES: ReadonlyArray<{ value: Theme; label: string }> = [
  { value: "system", label: "Follow system" },
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
];

/** 校验持久化/外部传入的主题值；非法时回退 system。 */
export function isValidTheme(value: string): value is Theme {
  return value === "system" || value === "light" || value === "dark";
}

/** 将主题应用到根节点；`system` 时 CSS 自动跟随系统。 */
export function applyTheme(root: HTMLElement, theme: Theme): void {
  root.setAttribute("data-theme", theme);
}

/** 编辑器预设（D6）：作为 datalist 建议，可自由输入任意命令名/路径。 */
export const EDITOR_PRESETS: ReadonlyArray<{ label: string; command: string }> = [
  { label: "VS Code", command: "code" },
  { label: "VS Code Insiders", command: "code-insiders" },
  { label: "Cursor", command: "cursor" },
];