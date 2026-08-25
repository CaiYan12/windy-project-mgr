# Windy Project Manager Development Status

Current Phase:
Phase 6 — Project Scanner（已完成）

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

In Progress:
- None

Blocked:
- None

Tests:
- Passed: 60（cargo test 全量：lib 2 + crud_commands 8 + dedup 12 + project_store 15 + real_repo 2 + scanner 15 + settings 6，2026-08-25）
- Failed: 0
- Not Run: 0（前端 vitest 尚未引入，属后续票面）

Build:
- Development: PASS（`pnpm tauri dev` 启动验证，2026-08-25）
- Production: PASS（`pnpm tauri build` 双 bundle 产出，2026-08-25；体积实测属 Phase 13 票面）

Known Issues:
- winget 安装 Rustlang.Rustup 时，rustup-init 安装器本体成功，但默认工具链自动安装失败（安装程序退出码 1）；已通过 `rustup default stable` 补装解决
- `%USERPROFILE%\.cargo\bin` 已加入用户 PATH，但当前会话已打开的 shell 不会自动刷新；验证时使用全路径，新开终端可直接使用 `rustc` / `cargo`

Next Step:
- Phase 7 — Git Scanner：调用系统 Git CLI（绝不 fetch）；按 D8 边界（无上游 ahead/behind=0 不显示、空仓库、detached@<短hash>、三态 status、最近 10 条）；专项测试：无上游分支 / 空仓库 / detached HEAD，真实临时仓库

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
