# Windy Project Manager Development Status

Current Phase:
Phase 8 — Dashboard UI（已完成）

Status:
COMPLETE

Completed:
- Phase 0：rustup / stable 工具链安装与验证，全环境审计实测并记入下表
- Phase 1：`CONTEXT.md` 术语表、`CHANGELOG.md`、`TESTING.md` 验收清单骨架、`docs/adr/0001~0003` 三份 ADR 落盘（均验证：存在 + 严格 UTF-8 无乱码）
- Phase 2：create-tauri-app 4.6.2 脚手架（react-ts 模板，D11：pnpm + React 19.2.8 + TS 5.8.3 + Vite 7.3.6 + Tauri 2.11.5，identifier `com.windy.project-mgr`）；`pnpm tauri dev` 启动验证通过（Vite 287ms ready，首次 470 crates 编译 1m57s，应用进程拉起无报错）；`pnpm tauri build` 通过（release 编译 2m21s，产出 MSI + NSIS 双 bundle）
- Phase 3：目录结构建立并可编译：`src/{components,pages,lib,types}`（.gitkeep 占位）与 `src-tauri/src/{commands,project,scanner,git}`（各含职责声明的 mod.rs，已接入 lib.rs）；`cargo check` 与 `pnpm build` 双侧验证通过
- Phase 4：数据层 TDD 完成（四个 red→green 切片）：`project::types`（Project / StoreError / Versioned）、`project::store`（Load/Save/Create/Update/Delete/Get + write_atomic + new_id/now_utc，仅 std）、`project::settings`（D6 同机制）；集成测试用真实临时目录，覆盖文件不存在 / 空文件 / JSON 损坏 / 版本不匹配 / 保存失败不损原文件 / 重复 ID / 删除不存在项目 / 删除不删目录
- Phase 5：`project::dedup`（normalize_path / is_same_path / find_duplicate_by_path，D3 三变体 + 正斜杠变体）；`commands::project`（5 个 CRUD Tauri command 已注册入 invoke_handler，核心 `*_in` 函数以 data_dir 参数化可测，app_data_dir = `%APPDATA%\windy-project-mgr`）；查重拒绝覆盖 create 与 update（自排除）
- Phase 6：`scanner` 模块实现：detect_project_type（Node/Python/Rust/Java/CSharp/Unknown）、detect_tech_stack（11 条特征规则）、detect_activity（根目录最新 mtime，单项失败不阻断）、list_startup_scripts（D5：start > run > 字母序，不递归）；路径不存在 / 无权限可诊断错误；真实目录验证：本仓库根目录（Node + pnpm/TS/Vite）与 src-tauri（Rust）实测通过；Git/Non-Git 目录的 Git 元数据扫描属 Phase 7
- Phase 7：`git` 模块实现（TDD，七个垂直切片）：`scan_git(path) -> Result<Option<GitMetadata>, GitError>`（非仓库 Ok(None)；路径不存在 / git 不可用可诊断报错）+ `scan_git_with` 可测核心；调用系统 Git CLI（std::process::Command，零新依赖，绝不 `git fetch`）；D8 全部边界：分支（含 `detached@<短hash>`）、三态 status（porcelain 失败 → Unknown，changed_files 计入 untracked）、ahead/behind 基于本地 `@{u}`（无上游 = 0）、recentCommits 最近 10 条（%x1f 分隔解析）、空仓库 lastCommit = None；新增集成测试 11 个（真实临时 git 仓库：非仓库 / 路径不存在 / git 不可用 / 空仓库 / 单提交 / modified / untracked / 12 提交上限 / 无上游 / 本地上游 ahead/behind / detached HEAD）
- Phase 8：Dashboard UI 落地（模板 UI 与 `greet` command 移除）。Rust：`scanner::scan_project_path` 组装 `ProjectMetadata`（单项失败降级不阻断，原始第 11 节）+ `commands/scan.rs`：`scan_project` / `list_scripts`（D5 所需最小增量，超出 §2.5 面已在代码注释与 AGENTS.md 说明）；新增集成测试 4 个。前端：`types/project.ts`（与 §2.3 对应）、`lib/`（api / search / cards / paths，vitest 纯逻辑 22 个，不写组件渲染测试，D12）、`components/`（Modal+Confirm / Sidebar / ProjectFields / AddDialog 两步（含 D5 文件选择器自选脚本）/ EditDialog / SettingsDialog / ProjectCard+骨架）、App.tsx 编排（启动骨架 + 并发扫描逐卡填充 D2、标签过滤 D9、四字段 Search D7、删除确认 D10、More 菜单含编辑器入口 D6、Run/Build 未配置禁用 + D4 文案）；`App.css` 单一全局样式表全变量化（ADR 0003，亮/暗双套变量集，本阶段 `prefers-color-scheme` 跟随系统，手动切换 + 持久化属 Phase 11）；新依赖：vitest 4.1.11（devDep，D12）、@tauri-apps/plugin-dialog + tauri-plugin-dialog 2（D5 文件选择器）；`pnpm build`（tsc 全量类型检查）通过；UI 运行时验收属 Phase 12 人工清单

In Progress:
- None

Blocked:
- None

Tests:
- Passed: cargo 75/75（lib 2 + crud_commands 8 + dedup 12 + git_scanner 11 + project_store 15 + real_repo 2 + scan_project 4 + scanner 15 + settings 6）；vitest 22/22（search 6 + cards 12 + paths 4，2026-08-25）
- Failed: 0
- Not Run: 0

Build:
- Development: PASS（`pnpm tauri dev` 启动验证，2026-08-25）
- Production: PASS（`pnpm tauri build` 双 bundle 产出，2026-08-25；体积实测属 Phase 13 票面）

Known Issues:
- winget 安装 Rustlang.Rustup 时，rustup-init 安装器本体成功，但默认工具链自动安装失败（安装程序退出码 1）；已通过 `rustup default stable` 补装解决
- `%USERPROFILE%\.cargo\bin` 已加入用户 PATH，但当前会话已打开的 shell 不会自动刷新；验证时使用全路径，新开终端可直接使用 `rustc` / `cargo`

Next Step:
- Phase 9 — Project Detail UI：Header / Overview / Tech / Git / Commits / Activity / Actions 全部呈现，降级文案正确；Actions 含“在编辑器中打开” + 手动刷新按钮（D6/D2，重扫复用 `scan_project`，数据仅内存缓存）；编辑入口可复用 Phase 8 的 EditProjectDialog

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
