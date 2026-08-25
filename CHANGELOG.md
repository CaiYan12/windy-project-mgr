# Changelog

本项目所有实际完成的变更均记录于此文件（Phase 1 建立，持续维护），按完成日期与 Phase 倒序组织。

MVP 完成前无正式版本号，变更记于 `Unreleased`。

## Unreleased

### 2026-08-25 — Phase 6 Project Scanner

- 新增 `scanner` 模块：`detect_project_type`（Node/Python/Rust/Java/CSharp → Unknown 降级）、`detect_tech_stack`（11 条特征规则：Node/pnpm/npm/yarn/TypeScript/Vite/Next.js/Python/Rust/Java/C#）、`detect_activity`（根目录最新 mtime + 本次扫描时刻）、`list_startup_scripts`（D5 排序 start > run > 字母序，不递归）
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
