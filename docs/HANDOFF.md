# HANDOFF — Windy Project Manager MVP 开发交接

> 生成于 2026-08-25（Phase 10 完成后）。新会话请先读本文，再按「第一优先级」读档。

## 当前状态快照

- **进度**：11 / 15 Phase 完成 · 26 / 34 子任务（权威追踪：`docs/PLAN.MD` 第 0 节）
- **仓库**：`d:\Dev\windy-project-mgr`，分支 `main`，工作区干净，最新提交见 `git log`（Phase 10 功能提交 + 本交接文档提交）
- **代码**：Tauri 2 后端全量完成（数据层 + CRUD + scan + launch + 13 个 command 全部注册，§2.5 面完整）+ Dashboard UI + Detail UI；前端操作入口（Open/Run/Build/编辑器）自 Phase 8/9 已就位，现已真实可用
- **测试**：`cargo test` 94/94；`pnpm vitest run` 25/25（纯逻辑，无组件渲染测试）；`pnpm build` 通过

## 已完成票（本会话，勿重做）

| Phase | 提交 | 要点 |
|---|---|---|
| 0 环境审计 | （无代码提交） | rustup 1.29.0 + rustc/cargo 1.98.0（stable-msvc）；审计表在 `PROJECT_STATUS.md` |
| 1 项目规划 | `82200eb` | CONTEXT.md / CHANGELOG.md / TESTING.md / docs/adr/0001~0003 |
| 2 工程初始化 | `a77fb42` | create-tauri-app react-ts（pnpm + React 19 + TS 5.8 + Vite 7 + Tauri 2.11.5），dev/build 验证通过 |
| 3 基础目录 | `6fa841d` | src/{components,pages,lib,types} 与 src-tauri/src/{commands,project,scanner,git} |
| 4 数据层 | `ded1de7` | projects.json / settings.json Store，原子写，23 测试（TDD） |
| 5 Project CRUD | `b4eb3d4` | 5 个 Tauri command + D3 路径查重（含词法绝对化），43 测试 |
| 6 Project Scanner | `2cb84c6` | 类型/技术栈/活动/启动脚本枚举（D5），60 测试 |
| 7 Git Scanner | `1c1f255` | `git::scan_git` + `scan_git_with`，系统 Git CLI 离线扫描（D8），真实临时仓库测试 11 个，71 测试 |
| 8 Dashboard UI | `468e934` | scan_project/list_scripts command + 全部 Dashboard 组件（D2/D5/D7/D9/D10），vitest 22 + cargo 75 测试，`pnpm build` 通过 |
| 9 Project Detail UI | `f8067a7` | `pages/ProjectDetail.tsx` 七分区 + 降级态；`selectedId` 视图切换（无路由库）；`shortHash`（TDD，vitest 22→25）；`detect_project_type` 返回 `C#`（红→绿） |
| 10 Run/Build/Open/Editor | （见 `git log`） | `launch` 模块（D4：`wt.exe` 优先回退 `powershell -NoExit`，detached 只报启动成败）+ `commands/actions.rs` 六个 command（D6 settings/editor）；TDD 红→绿：`tests/launch.rs` 15 + `tests/actions_commands.rs` 4（cargo 75→94）；运行时探针经真实 IPC 验证 10/10 PASS（探针已移除）；零新依赖 |

## 下一票：Phase 11 — UI Polish + 主题系统（PLAN.MD 第 0 节三个复选框）

- 亮 / 暗两套 CSS 变量集 + `data-theme` 切换 + 默认跟随系统（D1）；变量集已存在于 `App.css`（`prefers-color-scheme` 双套），本票加 `data-theme` 手动切换机制
- 设置 Dialog：主题选择（亮 / 暗 / 跟随系统）与编辑器配置（预设下拉 `code` / `code-insiders` / `cursor` + 自定义输入，D6）；接入 `get_settings` / `update_settings`（后端已就绪，前端 `lib/api.ts` 尚未封装这两个，属本票工作）
- 主题手动选择重启后保留（持久化到 `settings.json` 的 `theme` 字段）
- 现 `SettingsDialog` 为只读占位（Phase 8），本票替换为可编辑形态

## 关键工作协议（务必遵守，详见 `AGENTS.md`）

1. **先读**：`AGENTS.md` → `docs/PLAN.MD`（第 0 节进度 + 第 4 节决策 + 第 0.1 节勾选协议）→ `PROJECT_STATUS.md`
2. **TDD**：数据/逻辑票先写失败测试再实现（`/tdd`），垂直切片
3. **票尾**：`/code-review` 双轴评审 → 修正 → 提交（提交信息风格见既有历史）
4. **勾选协议**：验证通过才勾，同步汇总行（当前 11/15 · 26/34，日期 2026-08-25）
5. **会话闭环**：结束前同步 `AGENTS.md` 受影响章节、`docs/PLAN.MD` 进度、长期记忆（UpdateMemory）
6. **硬边界**：pnpm-only、零样式/状态框架、不擅自加依赖、删除记录永不删目录、绝不 `git fetch`

## 已知遗留事项（非阻塞）

- command 错误以字符串返回，前端靠子串匹配区分（如 `path not found`）→ 如需结构化错误码另立小票
- `Cargo.toml` description/authors 仍是模板默认值 → 顺手票修正
- `git` 与 `scanner` 模块各有一份 `require_dir`（错误类型不同，未提取共享）→ 如需复用再重构；`launch` 模块另有自己的 `require_dir`（LaunchError 类型）
- `list_scripts` 超出 PLAN §2.5 command 全集（D5 必要的已记录增量）→ 若后续修订 §2.5 一并纳入
- pnpm 11 将移除 `package.json#pnpm.onlyBuiltDependencies`（当前 10.26.2 正常）→ 升级时迁移到 pnpm-workspace.yaml
- UI 运行时行为未经人工验证（浏览器内无法走 Tauri IPC）→ Phase 12 人工清单覆盖
- **WebView2 Runtime ≥150 对提权宿主禁用 `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` 与 HKCU 策略注入的 `--remote-debugging-port`（CDP 无法开启，wry#1782 / WebView2Feedback#5640）**；本机开发终端为提权（High Integrity），故运行时验证用「临时前端探针 + 临时 `debug_log` command」方案（Phase 10 已验证可行，探针代码不提交）
- 评审判断项（未处理，属可接受惯用法）：卡片与 Detail 的 `ScanState` 三态分支及 Run/Build 禁用形状重复；测试文件间 `temp_dir` 助手重复

## Suggested Skills（新会话）

- `/frontend-design` — Phase 11 主题系统与设置 Dialog 前端编码（AGENTS.md #0 强制）
- `/code-review` — 票尾双轴评审
- `/chinese-encoding` — 中文文档写入防护
- `/context7-mcp` — 查 Tauri / React 相关文档
- `/handoff` — 上下文接近饱和时再次交接（禁止 Phase 中途压缩）
