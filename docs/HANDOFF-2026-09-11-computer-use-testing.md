# HANDOFF · Computer Use 人工测试任务（2026-09-11）

> 交接对象：负责在另一 agent/会话中用 Computer Use 做点击式验收的执行者。
> 目标：验证 2026-09-11 架构加固改动（A1/A2/A3/A4 + B1/B2/B4）在真实 Tauri 应用上的行为。
> 本文档自包含：环境、启动方式、已知限制、逐项测试清单与预期结果、汇报格式。

## 1. 仓库与当前状态

- 仓库：`D:\Dev\windy-project-mgr`（Windy Project Manager，Tauri 2 + React 19 + TS + Vite，Rust 后端）。
- 改动**尚未提交**（工作区脏，符合预期）。门禁已全绿：`tsc --noEmit` 0 错误；`vitest run` 9 文件 / 88 通过；`cargo test` 160 通过。
- 依据文档：
  - 计划：`docs/superpowers/plans/2026-09-11-architecture-hardening.md`（含“实施记录/计划偏差”）。
  - ADR：`docs/adr/0006-single-writer-store.md`、`docs/adr/0007-path-dedup-server-authority.md`。
  - 领域：`GLOSSARY.md`；验收清单：`TESTING.md`。
- **约束**：不要提交、不要 `cargo clean`、不要读写 `%APPDATA%`、不要动用户既有 `data\` 数据；测试产生的记录用完请删除。

## 2. 启动应用（重要：有环境坑）

- **`start.bat` 在 LobsterAI 环境下会失败**：
  `error: unrecognized subcommand 'D:\Program Files\LobsterAI\LobsterAI.exe'`。
  原因：Tauri CLI 依据 `argv[0]` 文件名判断运行时，而此处 Node 以 `LobsterAI.exe` 形态运行，CLI 便把该 exe 路径当成子命令。
- **用新脚本启动**（已写好，等价 `start.bat`）：
  ```
  cd D:\Dev\windy-project-mgr
  .\start.agent.bat dev      :: 开发模式（Vite 1420 + Tauri 窗口）
  .\start.agent.bat build    :: 生产构建
  ```
  脚本会自动挑选真实 `node.exe`（优先 `C:\Program Files\nodejs\node.exe`）去跑 `node_modules\@tauri-apps\cli\tauri.js`。
- 或在**独立终端**直接跑 `start.bat`（`node` 解析到真 node.exe 时不会触发该问题）。
- 开发数据目录：`D:\Dev\windy-project-mgr\src-tauri\target\debug\data\`（`projects.json` / `settings.json`）。
- 停止：在启动窗口按 Ctrl+C，或 `taskkill /IM windy-project-mgr.exe /F`。

## 3. Computer Use 环境限制（已知）

- 能：浏览器专用 `cua_repl` 会返回 `apps: []`；应使用配置好的 `mcp__node_repl__js` + `@oai/sky`，此入口的 `list_windows` / `activate_window` / `get_window_state`（截图）、坐标点击和键盘输入（Tab/Enter/Escape/文本）均可用。
- 本轮文本输入观察：弹窗打开后从 WebView 根焦点直接 `Shift+Tab` 未稳定落到 Path；按回退规则用窗口坐标明确聚焦 Path，再使用 `Ctrl+A` + `type_text`，随后用键盘提交。
- 结论：先使用正确的原生 Computer Use 入口；坐标点击可用时按坐标优先，焦点不确定或点击无反应时按键盘方案（下面是键盘路径）。

### 键盘驱动要点（本仓库实测）
- 窗口 1404×1104（窗口相对坐标；截图与坐标 1:1）。
- Dashboard 焦点顺序：`All projects` → 6 个标签(edited/react/rust/tauri/web/windows) → `Settings` → 搜索框 → `Add project`。
- 从“无焦点”起：`Tab×10` + `Enter` 打开 Add Dialog。
- Add Dialog 焦点顺序：Path → Browse… → Name → Description → Tags → Cancel → Next；Path 自动聚焦（Name 会按路径末段自动填充）。
- 弹窗：`Escape` 关闭。

## 4. 测试前状态与注意事项

- 当前 Dashboard 有 3 条历史记录（路径指向已不存在的临时目录 `...\Temp\windy-accept\...`），卡片会显示 `Scan failed: path not found` —— **这是预期降级**，不是缺陷。
- 你新加的测试记录请记下 id/name，测试后删除（删除只删记录、不删目录，见下 G2）。

## 5. 测试清单（逐项给出预期）

标记：`PASS` / `FAIL` / `BLOCKED`（附证据：截图时刻或现象文本）。

### A. 冒烟
1. 启动后先显示卡片骨架，随后逐卡填充；无红色 loadError。
2. Settings 弹窗可打开，三分页（Appearance / Editor / General）可切换。

### B. 路径查重单源化（A2/B4，本次重点）
1. Add Dialog：Path 填入一条**已存在记录**的变体（大小写 + 正斜杠 + 末尾斜杠，例：把 `C:\...\windy-accept\node-app` 写成 `c:/.../WINDY-ACCEPT/node-app/`），Name 任意 → 点 `Next`。
   预期：Step 1 立即提示
   `duplicate project path: <规范化后的存储路径> (edit the existing record instead)`，且**不进入** Step 2。
2. Add Dialog：Path 填**相对路径** `.\app` → `Next`。
   预期：Step 1 提示 `path must be absolute: .\app`（绝对路径判定已提前到第一步）。
3. Add Dialog：Path 填任一相对路径（如 `src-tauri\src`，无论目录是否存在）→ `Next`。
   预期：Step 1 提示 `path must be absolute: <输入>`，**不再**报 `Path not found`。
4. Add Dialog：Browse 选一个全新目录 → 正常进入 Step 2 → `Add project` 成功，卡片出现。
5. Edit Dialog：把某项目 Path 改成另一项目的路径 → `Save changes`。
   预期：提示 duplicate，且**不保存**。
6. Edit Dialog：只改名字/描述 → 保存成功（自排除生效，不与自身冲突）。

### C. 存储单一写者（A1）
1. 快速连续新增 3–4 个项目，再删除其中 2 个 → 关闭应用并重启。
   预期：列表与内容完全正确（不丢不重）。
2. 在 Settings 改颜色并保存后，立刻 Edit 某个项目并保存 → 重启。
   预期：设置与项目改动都在。

### D. 编辑器规则收敛（A3/B2）
1. Settings → Editor：Launch arguments 填两行、每行含一个 `{path}`（或一行不含）→ 预期：提示 `must contain exactly one {path}` 并禁用 `Save changes`。
2. 恰一个 `{path}` → 可保存；重启后保留。
3. 选 `.cmd`/`.bat` 编辑器且参数含双引号 `"` → 预期：提示双引号不允许并禁用保存。
4. 编辑器设为 `Not configured` → 卡片 More 与详情的编辑器入口提示 `Editor not configured`。
5. 配置 VS Code → 卡片/详情 `Open in editor` 能真实打开文件夹工作区。

### E. 默认值与主题（A4）
1. Settings → General → `Reset settings`（二次确认）→ 预期：颜色/强调色/编辑器回到默认（system / Windy teal / 未配置），**项目记录不变**。
2. 亮/暗/跟随系统切换即时生效，重启后保留。

### F. system 拆分回归（B1）
1. Settings → Editor：检测列表正常，来源标签（Path / Registry / Standard）与路径正确。
2. 强调色选 `Windows current` → 能读到系统色；读不到时提示并回退 Windy teal。

### G. 数据契约（锁定项，回归）
1. `src-tauri\target\debug\data\projects.json` 版本为 `1`、`settings.json` 版本为 `3`，字段结构与改动前一致。
2. 删除一个项目后，其**磁盘目录仍存在**。
3. 不产生 `%APPDATA%\windy-project-mgr` 目录。

## 6. 已知修复（2026-09-12 跟进）

- B3 已修复：`check_path_available` 返回值由 `Option<String>`（冲突路径 / null）改为三态
  `PathAvailability`（`available` / `duplicate{path}` / `notAbsolute`），Add/Edit 的 Step 1 一次调用即可
  判定绝对路径与重复。原先“相对路径先报 `Path not found`、`must be absolute` 需提交才触发”的缺口不再存在。
  修复后门禁：`tsc` 0 错误、`vitest` 88 通过、`cargo test` 160 通过。
- **验收结论（2026-09-12）**：整轮 19 PASS、1 FAIL（B3）、2 BLOCKED（A1 首帧、G3 约束）。B3 定向复测通过：
  B2 `\.\app` 与 B3 `src-tauri\\src` 均提示 `path must be absolute: <输入>`，未进入 Step 2，项目数仍为 3 → **B3 缺口关闭**。
- Computer Use 再补充：坐标点击对个别目标（聚焦输入框）**可能生效**，但按钮点击仍不可靠；`Shift+Tab` 聚焦也不稳定，必要时用坐标聚焦再键盘输入。

## 7. 汇报格式

- 逐项给出 `编号 · 结果(PASS/FAIL/BLOCKED) · 现象/证据`。
- FAIL 项附：输入、点击/按键序列、期望 vs 实际、截图时刻。
- 结尾：本次新增缺陷清单（如有）+ 清理说明（删除了哪些测试记录）。
- 不要提交代码；如需修复，把发现交回主会话处理。
