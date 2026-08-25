# Windy Project Manager Development Status

Current Phase:
Phase 10 — Run / Build / Open / Editor（已完成）

Status:
COMPLETE

Completed:
- Phase 0：rustup / stable 工具链安装与验证，全环境审计实测并记入下表
- Phase 1：`CONTEXT.md` 术语表、`CHANGELOG.md`、`TESTING.md` 验收清单骨架、`docs/adr/0001~0003` 三份 ADR 落盘（均验证：存在 + 严格 UTF-8 无乱码）
- Phase 2：create-tauri-app 4.6.2 脚手架（react-ts 模板，D11：pnpm + React 19.2.8 + TS 5.8.3 + Vite 7.3.6 + Tauri 2.11.5，identifier `com.windy.project-mgr`）；`pnpm tauri dev` 启动验证通过（Vite 287ms ready，首次 470 crates 编译 1m57s，应用进程拉起无报错）；`pnpm tauri build` 通过（release 编译 2m21s，产出 MSI + NSIS 双 bundle）
- Phase 3：目录结构建立并可编译：`src/{components,pages,lib,types}`（.gitkeep 占位）与 `src-tauri/src/{commands,project,scanner,git}`（各含职责声明的 mod.rs，已接入 lib.rs）；`cargo check` 与 `pnpm build` 双侧验证通过
- Phase 4：数据层 TDD 完成（四个 red→green 切片）：`project::types`（Project / StoreError / Versioned）、`project::store`（Load/Save/Create/Update/Delete/Get + write_atomic + new_id/now_utc，仅 std）、`project::settings`（D6 同机制）；集成测试用真实临时目录，覆盖文件不存在 / 空文件 / JSON 损坏 / 版本不匹配 / 保存失败不损原文件 / 重复 ID / 删除不存在项目 / 删除不删目录
- Phase 5：`project::dedup`（normalize_path / is_same_path / find_duplicate_by_path，D3 三变体 + 正斜杠变体）；`commands::project`（5 个 CRUD Tauri command 已注册入 invoke_handler，核心 `*_in` 函数以 data_dir 参数化可测，app_data_dir = `%APPDATA%\windy-project-mgr`）；查重拒绝覆盖 create 与 update（自排除）
- Phase 6：`scanner` 模块实现：detect_project_type（Node/Python/Rust/Java/CSharp/Unknown；`CSharp` 于 Phase 9 统一为 `C#`）、detect_tech_stack（11 条特征规则）、detect_activity（根目录最新 mtime，单项失败不阻断）、list_startup_scripts（D5：start > run > 字母序，不递归）；路径不存在 / 无权限可诊断错误；真实目录验证：本仓库根目录（Node + pnpm/TS/Vite）与 src-tauri（Rust）实测通过；Git/Non-Git 目录的 Git 元数据扫描属 Phase 7
- Phase 7：`git` 模块实现（TDD，七个垂直切片）：`scan_git(path) -> Result<Option<GitMetadata>, GitError>`（非仓库 Ok(None)；路径不存在 / git 不可用可诊断报错）+ `scan_git_with` 可测核心；调用系统 Git CLI（std::process::Command，零新依赖，绝不 `git fetch`）；D8 全部边界：分支（含 `detached@<短hash>`）、三态 status（porcelain 失败 → Unknown，changed_files 计入 untracked）、ahead/behind 基于本地 `@{u}`（无上游 = 0）、recentCommits 最近 10 条（%x1f 分隔解析）、空仓库 lastCommit = None；新增集成测试 11 个（真实临时 git 仓库：非仓库 / 路径不存在 / git 不可用 / 空仓库 / 单提交 / modified / untracked / 12 提交上限 / 无上游 / 本地上游 ahead/behind / detached HEAD）
- Phase 8：Dashboard UI 落地（模板 UI 与 `greet` command 移除）。Rust：`scanner::scan_project_path` 组装 `ProjectMetadata`（单项失败降级不阻断，原始第 11 节）+ `commands/scan.rs`：`scan_project` / `list_scripts`（D5 所需最小增量，超出 §2.5 面已在代码注释与 AGENTS.md 说明）；新增集成测试 4 个。前端：`types/project.ts`（与 §2.3 对应）、`lib/`（api / search / cards / paths，vitest 纯逻辑 22 个，不写组件渲染测试，D12）、`components/`（Modal+Confirm / Sidebar / ProjectFields / AddDialog 两步（含 D5 文件选择器自选脚本）/ EditDialog / SettingsDialog / ProjectCard+骨架）、App.tsx 编排（启动骨架 + 并发扫描逐卡填充 D2、标签过滤 D9、四字段 Search D7、删除确认 D10、More 菜单含编辑器入口 D6、Run/Build 未配置禁用 + D4 文案）；`App.css` 单一全局样式表全变量化（ADR 0003，亮/暗双套变量集，本阶段 `prefers-color-scheme` 跟随系统，手动切换 + 持久化属 Phase 11）；新依赖：vitest 4.1.11（devDep，D12）、@tauri-apps/plugin-dialog + tauri-plugin-dialog 2（D5 文件选择器）；`pnpm build`（tsc 全量类型检查）通过；UI 运行时验收属 Phase 12 人工清单
- Phase 9：Project Detail UI 落地。新增 `src/pages/ProjectDetail.tsx`：七分区（Header / Overview / Technology / Git / Recent Commits / Activity / Actions）全部呈现，降级态全部可见（`git = null` → `No Git repository`；空仓库 → `No commits yet`；`status = unknown`；`projectType = null` → `Unknown project type`；`activity.lastModifiedAt = null` → `No recent activity`；loading 骨架；error + Retry）；Actions 含 Open / Run / Build（未配置禁用 + D4 文案）/ Open in editor（D6，后端属 Phase 10，未注册时 Toast 报错为预期过渡态）/ Refresh（调 `scan_project` 单项目重扫，仅内存，D2）/ Edit / Delete（复用 `EditProjectDialog` / `ConfirmDialog`）；recentCommits 行 = `shortHash` 短码 + message + author + 相对时间。视图切换：`App.tsx` `selectedId` 状态（无路由库），卡片点击进入 Detail（`ProjectCard` `onSelect` + 内部交互 `stopPropagation`，项目名为键盘可达按钮），删除选中项返回 Dashboard。纯逻辑（TDD 红→绿）：`lib/cards.ts` `shortHash` + vitest 3 个。Rust：`detect_project_type` 对 `*.csproj` 返回 `C#`（原 `CSharp`，与技术栈标签统一；projectType 仅运行时，D2 不落盘），`tests/scanner.rs` 断言同步（红→绿）。样式：`App.css` 追加 Detail 分区样式，全走既有 CSS 变量（ADR 0003）；双轴评审修正内联 `cursor` 样式与 `git = null` 时 Commits 分区文案。零新依赖；未提前实现 Phase 10 后端 command
- Phase 10：Run / Build / Open / Editor 后端落地。新增 `launch` 模块（D4 / ADR 0002）：`LaunchPlan` 纯数据计划 + `spawn_plan`（detached，只报启动成败）+ `run_in_terminal`（优先 `wt.exe` 回退 `powershell -NoExit`，`cwd = project.path`）+ `open_dir`（explorer）+ `open_in_editor`（未配置 → `Editor not configured`，先于路径检查）；二进制名可注入（`run_in_terminal_with`）。新增 `commands/actions.rs`：`get_settings` / `update_settings`（`*_in` 可测核心）/ `open_project` / `run_project` / `build_project` / `open_in_editor` 六个 command 注册（§2.5 全集完整）。测试（TDD 红→绿）：`tests/launch.rs` 15 个（真实进程拉起，`hostname` 替身）+ `tests/actions_commands.rs` 4 个；cargo 75→94。运行时验证：WebView2 Runtime 151 对提权宿主禁用 CDP 环境变量（wry#1782），改用临时前端探针经真实 IPC 在内置 WebView2 内验证：10/10 PASS（含持久化往返与三类错误文案），探针已移除。零新依赖（纯 `std::process::Command`）；双轴评审修正未用派生与 `wt.exe` 字面

In Progress:
- None

Blocked:
- None

Tests:
- Passed: cargo 94/94（lib 2 + actions_commands 4 + crud_commands 8 + dedup 12 + git_scanner 11 + launch 15 + project_store 15 + real_repo 2 + scan_project 4 + scanner 15 + settings 6）；vitest 25/25（search 6 + cards 15 + paths 4，2026-08-25）
- Failed: 0
- Not Run: 0

Build:
- Development: PASS（`pnpm tauri dev` 启动验证，2026-08-25）
- Production: PASS（`pnpm tauri build` 双 bundle 产出，2026-08-25；体积实测属 Phase 13 票面）

Known Issues:
- winget 安装 Rustlang.Rustup 时，rustup-init 安装器本体成功，但默认工具链自动安装失败（安装程序退出码 1）；已通过 `rustup default stable` 补装解决
- `%USERPROFILE%\.cargo\bin` 已加入用户 PATH，但当前会话已打开的 shell 不会自动刷新；验证时使用全路径，新开终端可直接使用 `rustc` / `cargo`

Next Step:
- Phase 11 — UI Polish + 主题系统：亮 / 暗两套 CSS 变量集 + `data-theme` 切换 + 默认跟随系统（D1）；设置 Dialog：主题选择与编辑器配置（预设下拉 + 自定义，接入 `get_settings` / `update_settings`）；主题手动选择持久化到 `settings.json`

---

## 环境审计表（2026-08-25 实测）

| 项目 | 状态 / 版本 | 来源 |
|---|---|---|
| OS | Windows 11（10.0.26200，Build 26200，25H2） | `Get-CimInstance Win32_OperatingSystem` |
| Node | v24.18.0 | `node -v` |
| Package Manager | pnpm 10.26.2（D11 选定，锁定 `pnpm-lock.yaml`） | `pnpm -v` |
| Rust | rustc 1.98.0 (88d9e12ae 2026-08-18)，stable 工具链 | `rustc --version` |
| Cargo | cargo 1.98.0 (797e8a9bc 2026-08-05) | `cargo --version` |
| rustup | 1.29.0 (28d1352db 2026-03-05)，默认工具链 `stable-x86_64-pc-windows-msvc`，target `x86_64-pc-windows-msvc` | `rustup show` |
| Tauri CLI | `@tauri-apps/cli` 2.11.4（工程 devDependency，经 `pnpm tauri` 调用；Tauri 运行时 2.11.5） | `pnpm install` 输出 |
| Git | 2.48.1.windows.1 | `git --version` |
| MSVC C++ 构建工具 | Visual Studio Community 2022（17.14.37216.2，`C:\Program Files\Microsoft Visual Studio\2022\Community`），vswhere 确认含 `Microsoft.VisualStudio.Component.VC.Tools.x86.x64` | `vswhere -requires VC.Tools.x86.x64` |
| WebView2 | Windows 11 内置，无需安装 | 系统事实 |
| 可用编辑器 | `code` 命令可用（`D:\Program Files\QoderCN\bin\code.cmd`） | `Get-Command code` |
| winget | v1.29.290 | `winget --version` |
| Working Directory | `d:\Dev\windy-project-mgr` | — |
| 仓库状态 | 分支 `main`，最近提交 `caca81b primal plan`；文档重组后有未提交变更（旧根目录文档移入 `docs/`，新增 `AGENTS.md` / `README.md`） | `git status` |
