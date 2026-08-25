# Changelog

本项目所有实际完成的变更均记录于此文件（Phase 1 建立，持续维护），按完成日期与 Phase 倒序组织。

MVP 完成前无正式版本号，变更记于 `Unreleased`。

## Unreleased

### 2026-08-25 — Phase 10 Run / Build / Open / Editor

- **Rust**：新增 `launch` 模块（D4 / ADR 0002）：`LaunchPlan` 纯数据计划（`wt_plan` / `ps_plan` / `editor_plan` / `open_dir_plan`）+ `spawn_plan`（detached 拉起，只报启动成败，不采集退出码与输出）+ `run_in_terminal`（优先 `wt.exe`，拉起失败回退 `powershell -NoExit`，`cwd = project.path`）+ `open_dir`（explorer）+ `open_in_editor`（`editorCommand` 为空 → `Editor not configured`，未配置检查先于路径检查）；二进制名可注入（`run_in_terminal_with`）便于测试
- **Rust**：新增 `commands/actions.rs`：`get_settings` / `update_settings`（`*_in(data_dir)` 可测核心，复用 `Settings` 原子写）/ `open_project` / `run_project` / `build_project` / `open_in_editor`（读 `settings.json` 的 `editorCommand`）六个 Tauri command 注册入 invoke_handler（§2.5 command 全集至此完整）
- **测试**（TDD 红→绿）：`tests/launch.rs` 15 个（计划形态纯断言 + 真实进程拉起：`hostname` 充当终端/编辑器替身、不存在二进制验证回退与失败、无效路径 / 空命令 / 未配置编辑器错误）+ `tests/actions_commands.rs` 4 个（设置默认值 / 持久化往返 / 数据目录自动创建 / 损坏可诊断）；`cargo test` 75→94
- **前端**：`lib/api.ts` 注释更新（六个操作 command 后端已实现；`get_settings` / `update_settings` 前端接入属 Phase 11）；无其它前端改动（Phase 8 封装与 UI 入口原样复用）
- **运行时验证**：WebView2 Runtime 151 对提权宿主禁用 `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`（CDP 无法开启，证据：wry#1782 / WebView2Feedback#5640），改用临时前端探针经真实 IPC 在内置 WebView2 内逐一调用六个 command：10/10 PASS（默认设置 / 持久化往返 / open / run / build / editor 拉起 / 未配置与无效路径与空命令错误文案）；探针代码验收后已移除
- **验证**：`cargo test` 94/94、`pnpm vitest run` 25/25、`pnpm build`（tsc 全量）通过；零新依赖（纯 `std::process::Command`）；双轴评审修正：`LaunchPlan` 未用派生移除、`wt` → `wt.exe` 对齐规格字面

### 2026-08-25 — Phase 9 Project Detail UI

- **前端**：新增 `src/pages/ProjectDetail.tsx`（七分区：Header / Overview / Technology / Git / Recent Commits / Activity / Actions；只消费传入 `scan`，不发起扫描不持久化，D2）；降级态全部可见：`git = null` → `No Git repository`、空仓库 → `No commits yet`、`status = unknown` 状态点与文案、`projectType = null` → `Unknown project type`、`activity.lastModifiedAt = null` → `No recent activity`、loading 骨架、error + Retry；Actions：Open / Run / Build（未配置禁用 + D4 文案）/ Open in editor（D6，后端属 Phase 10，未注册时 Toast 报错为预期过渡态）/ Refresh（调 `scan_project` 单项目重扫，仅内存）/ Edit / Delete（复用 `EditProjectDialog` / `ConfirmDialog`）
- **视图切换**：`App.tsx` 新增 `selectedId` 状态（无路由库）；卡片点击进入 Detail（`ProjectCard` 新增 `onSelect`，内部按钮 / 菜单 / Retry 全部 `stopPropagation`，项目名为键盘可达按钮）；删除选中项后返回 Dashboard；编辑保存后 Detail 经派生查找自动反映新记录
- **纯逻辑（TDD）**：`lib/cards.ts` 新增 `shortHash`（取前 7 位，短输入原样返回），recentCommits 行 = hash 短码 + message + author + 相对时间；新增 vitest 3 个（红→绿）
- **Rust**：`detect_project_type` 对 `*.csproj` 返回 `C#`（原 `CSharp`），与技术栈标签口径统一（projectType 仅运行时数据，D2 不落盘，无持久化影响）；`tests/scanner.rs` 断言同步（红→绿）
- **样式**（ADR 0003）：`App.css` 追加 Detail 分区样式（`.detail*` / `.commit-list` / `.commit-row` / `.card-title-link` / `.card.clickable`），全走既有 CSS 变量，复用 `.status-dot` / `.tech-chip` / `.skel` / `card-in`；双轴评审修正：卡片内联 `cursor` 样式改为 `.card.clickable` 类（ADR 0003 拒内联样式）、`git = null` 时 Recent Commits 分区误显示 `No commits yet` 改为 `No Git repository`
- **测试**：`cargo test` 75/75、`pnpm vitest run` 25/25（cards 12→15）、`pnpm build`（tsc 全量）通过；零新依赖；未提前实现 Phase 10 后端 command；UI 运行时行为属 Phase 12 人工验收清单

### 2026-08-25 — Phase 8 Dashboard UI

- **Rust**：`scanner::scan_project_path` 组装 `ProjectMetadata`（字段与 §2.3 对应；单项采集失败降级为缺省值不阻断其它字段，原始第 11 节）；新增 `commands/scan.rs`：`scan_project`（D2 前端并发调用）与 `list_scripts`（Add Dialog Step 2 需在项目登记前枚举脚本，既有 command 面无法覆盖，作为 D5 所需最小增量并在代码注释/AGENTS.md 记录）；模板 `greet` command 随模板 UI 移除；新增集成测试 4 个（真实临时目录 + 真实临时 git 仓库）
- **前端**（模板 UI 全部替换）：`types/project.ts`（与 §2.2/§2.3 对应，含 `ScanState`）、`lib/api.ts`（IPC 封装，Phase 10 命令先行封装并注明）、`lib/search.ts`（D7）、`lib/cards.ts`（降级文案 + D8 ahead/behind 为 0 不显示 + 相对时间）、`lib/paths.ts`（name 自动取末段）；组件：`Modal`+`ConfirmDialog` / `Sidebar`（D9 标签自动提取 + 设置入口）/ `ProjectCard`（骨架屏 + 降级 + More 菜单含编辑器入口）/ `AddProjectDialog`（两步，D10 name 自动填充与查重承接、D5 脚本预选 + 文件选择器自选）/ `EditProjectDialog`（单步）/ `SettingsDialog`（只读状态，手动主题与持久化属 Phase 11）；App.tsx：启动骨架 + 并发扫描逐卡填充（D2）、标签过滤、四字段 Search、删除确认（仅删记录）、Run/Build 未配置禁用（D4 文案）、错误 Toast；`index.html` 标题改为产品名；移除已替代的 `.gitkeep` 占位
- **样式**（ADR 0003）：单一全局样式表 `App.css`，颜色/间距/圆角/状态色全 CSS 变量（评审修正：硬编码 `#ffffff`/`rgba`/微小间距全部改为变量）；亮/暗两套变量集，本阶段经 `prefers-color-scheme` 跟随系统；骨架 shimmer + 卡片入场动效遵循 `prefers-reduced-motion`；签名元素：Sidebar 三道错落“风痕” wordmark
- **依赖**（均说明价值）：vitest 4.1.11（devDep，D12 前端纯逻辑测试，`pnpm test` 脚本）；@tauri-apps/plugin-dialog 2.7.2 + tauri-plugin-dialog 2（D5 自选脚本文件选择器，`dialog:default` 能力已声明）
- **测试**：新增前端纯逻辑测试 22 个（search 6 + cards 12 + paths 4，不写组件渲染测试，D12）+ Rust 集成测试 4 个；`cargo test` 75/75、`pnpm vitest run` 22/22、`pnpm build`（tsc 全量）通过；双轴评审修正：CSS 硬编码、D5 文件选择器缺失、`gitSyncLine` 双调用、TS 文件内 `//!` 注释；UI 运行时行为属 Phase 12 人工验收清单

### 2026-08-25 — Phase 7 Git Scanner（TDD）

- 新增 `git` 模块：`scan_git(path) -> Result<Option<GitMetadata>, GitError>`（非仓库返回 `Ok(None)`）+ `scan_git_with(path, git_bin)` 可测核心（沿用 `*_in` 可测模式）；`lib.rs` 中 `git` 模块改为 `pub`
- 调用系统 Git CLI（`std::process::Command`，零新依赖），全程离线（绝不 `git fetch`，符合 D8 / 增量验收 5）
- D8 边界全部落实：分支名（`symbolic-ref`，detached HEAD → `detached@<短hash>`）；三态 status（`status --porcelain`，失败 → Unknown；口径：changed_files 计入全部条目含 untracked）；ahead/behind 基于本地 `@{u}`（无上游 = 0/0）；recentCommits 最近 10 条（`%x1f` 分隔解析）；空仓库 `last_commit = None`、`recent_commits = []`
- 可诊断错误：`GitError::{PathNotFound, GitNotFound, GitFailed}`；git 不可用不导致崩溃；`GitMetadata` / `GitCommit` / `GitStatus` 字段与 PLAN §2.3 逐一对应（camelCase 序列化）
- 新增集成测试 11 个（真实临时 `git init` 仓库，D12：非仓库 / 路径不存在 / git 不可用 / 空仓库 / 单提交 / modified / untracked / 12 提交上限 10 / 无上游 / 本地上游 ahead/behind / detached HEAD）；`cargo test` 71/71 通过

### 2026-08-25 — Phase 6 Project Scanner

- 新增 `scanner` 模块：`detect_project_type`（Node/Python/Rust/Java/CSharp → Unknown 降级；`CSharp` 于 Phase 9 统一为 `C#`）、`detect_tech_stack`（11 条特征规则：Node/pnpm/npm/yarn/TypeScript/Vite/Next.js/Python/Rust/Java/C#）、`detect_activity`（根目录最新 mtime + 本次扫描时刻）、`list_startup_scripts`（D5 排序 start > run > 字母序，不递归）
- 可诊断错误：`ScannerError::{PathNotFound, PermissionDenied, Io}`；单项失败不影响其它扫描；无 AST、不递归深扫
- 新增测试 17 个（scanner 15 + real_repo 2，含本仓库根目录 / src-tauri 真实目录验证）；`cargo test` 60/60 通过

### 2026-08-25 — Phase 5 Project CRUD

- 新增 `project::dedup`：路径规范化（分隔符统一 / 去尾分隔符）+ 大小写不敏感比较（D3）
- 新增 `commands::project`：`get_projects` / `get_project` / `create_project` / `update_project` / `delete_project` 五个 Tauri command 注册入 invoke_handler；核心逻辑 `*_in(data_dir)` 可测；数据目录 `%APPDATA%\windy-project-mgr`
- create / update 均做路径查重拒绝（DuplicatePath，引导编辑已有记录，不合并）；删除仅删记录。决策记录：D3 字面仅约束“添加时”，此处将其唯一性不变量同等应用到 update（否则 Edit Dialog 改路径即可绕过查重；D10 允许编辑全部字段）；路径规范化含词法绝对化（相对路径按当前目录展开、`.`/`..` 解析），不访问文件系统
- 新增测试 20 个（dedup 12 + crud_commands 8）；`cargo test` 43/43 通过

### 2026-08-25 — Phase 4 数据层（TDD）

- 新增 `project::types`：`Project`（8 字段，camelCase 序列化）、`StoreError`（Io / Corrupted / VersionMismatch / NotFound / DuplicateId）、版本化文件外层 `Versioned<T>`
- 新增 `project::store`：`Store` Load/Save/Create/Update/Delete/Get；原子写（临时文件 + sync + 重命名，失败清理不损原文件）；`new_id` / `now_utc`（仅 std，无新依赖）
- 新增 `project::settings`：`settings.json` 读写（D6，同机制，默认 `editorCommand=""` / `theme="system"`）
- 新增集成测试 21 个（真实临时目录，无 Mock）+ 单元测试 2 个；`cargo test` 23/23 通过

### 2026-08-25 — Phase 3 基础目录

- 建立 `src/{components,pages,lib,types}`（.gitkeep 占位）与 `src-tauri/src/{commands,project,scanner,git}`（各含职责声明的 mod.rs）
- `cargo check` 与 `pnpm build` 双侧编译验证通过

### 2026-08-25 — Phase 2 工程初始化

- create-tauri-app 4.6.2 脚手架：react-ts 模板（D11：pnpm + React 19.2.8 + TypeScript 5.8.3 + Vite 7.3.6 + Tauri 2.11.5）
- 依赖锁定 `pnpm-lock.yaml`；esbuild 构建脚本经 `pnpm.onlyBuiltDependencies` 白名单化（pnpm 10 默认拦截）
- `pnpm tauri dev` 启动验证通过；`pnpm tauri build` 产出 MSI + NSIS 双 bundle

### 2026-08-25 — Phase 1 项目规划

- 新增 `CONTEXT.md`：领域术语表（D13）
- 新增 `docs/adr/0001-scan-data-memory-only.md`：扫描数据仅内存、启动全量重扫（D2）
- 新增 `docs/adr/0002-detached-run-build.md`：Run / Build 分离式启动、不采集退出码（D4）
- 新增 `docs/adr/0003-css-variable-theming.md`：`data-theme` + CSS 变量主题、默认跟随系统（D1）
- 新增 `TESTING.md`：人工验收清单骨架（D12 / D13）

### 2026-08-25 — Phase 0 环境审计

- 安装 rustup 1.29.0（winget）与 stable-x86_64-pc-windows-msvc 工具链（rustc / cargo 1.98.0）
- 新增 `PROJECT_STATUS.md`：开发状态与实测环境审计表
