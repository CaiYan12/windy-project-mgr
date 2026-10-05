# HANDOFF — Windy Project Manager MVP 开发交接

> 生成于 2026-08-28（Phase 15 图标细节修复与 build 产物后）。新会话请先读本文，再按「第一优先级」读档。

## 当前状态快照

- **进度**：16 / 16 Phase 完成 · 37 / 37 子任务（权威追踪：`docs/PLAN.MD` 第 0 节）
- **仓库**：`d:\Dev\windy-project-mgr`，分支 `main`；工作区包含本次图标细节修复、构建版稳定性修复与 Git 扫描窗口修复，尚未创建新提交
- **代码**：Tauri 2 后端全量完成（16 个 command，含编辑器发现、Windows 色和应用信息）+ Dashboard / Detail UI + Settings v2 三分页 + 零依赖自绘 SVG `Icon.tsx`；Card / Detail 动作图标统一 16px 对齐，Build 使用指定 1024×1024 path 并归一到 24×24；颜色模式、强调色和编辑器配置独立持久化
- **测试**：`cargo test --manifest-path src-tauri/Cargo.toml` 150/150；本轮 `pnpm test` 7 个文件 / 69 个测试通过（纯逻辑/静态契约，无组件渲染测试）；`pnpm build`、`build.bat` 与 `git diff --check` 通过

## 已完成票（本会话，勿重做）

| Phase | 提交 | 要点 |
|---|---|---|
| 0 环境审计 | （无代码提交） | rustup 1.29.0 + rustc/cargo 1.98.0（stable-msvc）；审计表在 `PROJECT_STATUS.md` |
| 1 项目规划 | `82200eb` | GLOSSARY.md / CHANGELOG.md / TESTING.md / docs/adr/0001~0003 |
| 2 工程初始化 | `a77fb42` | create-tauri-app react-ts（pnpm + React 19 + TS 5.8 + Vite 7 + Tauri 2.11.5），dev/build 验证通过 |
| 3 基础目录 | `6fa841d` | src/{components,pages,lib,types} 与 src-tauri/src/{commands,project,scanner,git} |
| 4 数据层 | `ded1de7` | projects.json / settings.json Store，原子写，23 测试（TDD） |
| 5 Project CRUD | `b4eb3d4` | 5 个 Tauri command + D3 路径查重（含词法绝对化），43 测试 |
| 6 Project Scanner | `2cb84c6` | 类型/技术栈/活动/启动脚本枚举（D5），60 测试 |
| 7 Git Scanner | `1c1f255` | `git::scan_git` + `scan_git_with`，系统 Git CLI 离线扫描（D8），真实临时仓库测试 11 个，71 测试 |
| 8 Dashboard UI | `468e934` | scan_project/list_scripts command + 全部 Dashboard 组件（D2/D5/D7/D9/D10），vitest 22 + cargo 75，`pnpm build` 通过 |
| 9 Project Detail UI | `f8067a7` | `pages/ProjectDetail.tsx` 七分区 + 降级态；`selectedId` 视图切换；`shortHash`（vitest 22→25）；`CSharp`→`C#`（红→绿） |
| 10 Run/Build/Open/Editor | `f36ef4e` | `launch` 模块（D4）+ `commands/actions.rs` 六个 command（D6）；TDD 红→绿 19 个（cargo 75→94）；运行时探针 10/10 PASS；零新依赖 |
| 11 UI Polish + 主题系统 | `c53e454` | 主题系统（D1）：`data-theme` 三态（`:root[data-theme="dark"]` 强制暗 + `@media:not([data-theme="light"])` 跟随系统）；`lib/theme.ts` 纯逻辑（TDD 红→绿 4 个，vitest 25→29）；SettingsDialog 由只读占位改可编辑（主题单选组 + 编辑器 datalist 预设 code/code-insiders/cursor + 自定义）；App 启动读取并应用主题（重启保留）；运行时探针验证 D1 全链路（`data-theme=dark` 生效、`--bg=#14181a`、IPC 往返一致，探针已移除）；零新依赖 |
| 15 自绘 SVG 图标与 UI 审计 | （未提交） | `components/Icon.tsx` + `lib/iconUi.test.ts`；全页面语义接入、主题 / Git 状态双重表达、焦点 / 间距 / 收缩 / reduced-motion 规则；静态 320 / 768 / 1024 / 1440 无溢出，真实 Tauri 默认窗口主要路径复核 |

## 当前交付状态：MVP 与 Phase 15 UI 审计完成；任务按用户要求暂停

- `TESTING.md` 人工验收清单逐项勾选通过：Functional（CRUD / 扫描 / Detail / Search / 主题）/ Error Handling（Git 缺失 / 路径不存在 / JSON 损坏 / Run 失败）/ UX / Stability（重启持久化）/ Production
- 增量验收：脚本引导（D5）、编辑器打开与 `Editor not configured` 引导（D6）、主题切换与重启持久化（D1）、查重拒绝（D3，大小写/分隔符变体）
- Phase 12 B/C、Settings v2 以及 Phase 14 最终 Functional / Error Handling / UX / Stability / Production 验收均已完成，最终结果由用户确认。
- Phase 15 图标系统审计已完成：Dashboard、Detail、Add/Edit/Confirm、More、Settings 三分页、Toast、空态、错误态、加载态均已复核；320 / 768 / 1024 / 1440 数值检查来自静态 Vite 页面，原生 Tauri 记录为默认窗口交互结果。
- 2026-08-28 图标细节修复已通过 `pnpm test`（69/69）与 `build.bat`，Build glyph 使用指定 path + `currentColor` / `stroke="none"`，产物位于 `build\\win-unpacked\\windy-project-mgr.exe`；该项本身按需求未做实机验证。
- 2026-08-28 Git 扫描窗口回归已修复：`src-tauri/src/git/mod.rs` 为系统 Git 子进程设置 `CREATE_NO_WINDOW`；真实绿色 EXE 走通 `D:\Dev\opia-rss-reader` 的 Add Project（观测 Git 子进程均为 `hwnd=0`）与 Detail Refresh（3 秒 UIA 观察无新增顶层窗口）；验证数据已清理。
- Other 程序只保证启动，不保证支持文件夹工作区；本机 Notepad 启动后明确拒绝打开文件夹，VS Code 与 Qoder 已证明文件夹工作区路径。

## 关键工作协议（务必遵守，详见 `AGENTS.md`）

1. **先读**：`AGENTS.md` → `docs/PLAN.MD`（第 0 节进度 + 第 4 节决策 + 第 0.1 节勾选协议）→ `PROJECT_STATUS.md`
2. **TDD**：数据/逻辑票先写失败测试再实现（`/tdd`），垂直切片
3. **票尾**：`/code-review` 双轴评审 → 修正 → 提交（提交信息风格见既有历史）
4. **勾选协议**：验证通过才勾，同步汇总行（当前 16/16 · 37/37，日期 2026-08-28）
5. **会话闭环**：结束前同步 `AGENTS.md` 受影响章节、`docs/PLAN.MD` 进度、长期记忆（UpdateMemory）
6. **硬边界**：pnpm-only、零样式/状态框架、不擅自加依赖、删除记录永不删目录、绝不 `git fetch`

## 已知遗留事项（非阻塞）

- command 错误以字符串返回，前端靠子串匹配区分（如 `path not found`）→ 如需结构化错误码另立小票
- `Cargo.toml` description/authors 仍是模板默认值 → 顺手票修正
- `git` / `scanner` / `launch` 模块各有一份 `require_dir`（错误类型不同，未提取共享）→ 如需复用再重构
- `list_scripts` 超出 PLAN §2.5 command 全集（D5 必要的已记录增量）→ 若后续修订 §2.5 一并纳入
- pnpm 11 将移除 `package.json#pnpm.onlyBuiltDependencies`（当前 10.26.2 正常）→ 升级时迁移到 pnpm-workspace.yaml
- Settings / Add Project 构建版重复开关稳定性回归已完成；`reg.exe` 控制台子进程已通过 `CREATE_NO_WINDOW` 隐藏，未发现残留进程
- Git 扫描构建版窗口回归已完成；扫描过程中的 Git 子进程已通过 `CREATE_NO_WINDOW` 隐藏，Add Project / Detail Refresh 的真实窗口观察未发现新增顶层窗口
- **WebView2 Runtime ≥150 对提权宿主禁用 CDP 注入（wry#1782 / WebView2Feedback#5640）**；本机开发终端为提权（High Integrity），运行时验证用「临时前端探针 + 临时 `debug_log` command」方案（Phase 10/11 已验证可行，探针代码不提交）
- App.css 暗色变量集在 `:root[data-theme="dark"]` 与 `@media` 各写一份（纯 CSS 无法复用），已加相互注释避免漂移
- 本轮 Settings v2 的中间验收证据仍保留在 `.superpowers/sdd/2026-08-27-settings-overhaul/`；最终人工验收已由用户完成。

## Suggested Skills（新会话）

- `/frontend-design` — Phase 12 若涉 UI 调整（AGENTS.md #0 强制）
- `/code-review` — 票尾双轴评审
- `/chinese-encoding` — 中文文档写入防护
- `/context7-mcp` — 查 Tauri / React 相关文档
- `/handoff` — 上下文接近饱和时再次交接（禁止 Phase 中途压缩）
