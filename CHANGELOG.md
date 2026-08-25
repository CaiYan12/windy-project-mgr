# Changelog

本项目所有实际完成的变更均记录于此文件（Phase 1 建立，持续维护），按完成日期与 Phase 倒序组织。

MVP 完成前无正式版本号，变更记于 `Unreleased`。

## Unreleased

### 2026-08-25 — Phase 1 项目规划

- 新增 `CONTEXT.md`：领域术语表（D13）
- 新增 `docs/adr/0001-scan-data-memory-only.md`：扫描数据仅内存、启动全量重扫（D2）
- 新增 `docs/adr/0002-detached-run-build.md`：Run / Build 分离式启动、不采集退出码（D4）
- 新增 `docs/adr/0003-css-variable-theming.md`：`data-theme` + CSS 变量主题、默认跟随系统（D1）
- 新增 `TESTING.md`：人工验收清单骨架（D12 / D13）

### 2026-08-25 — Phase 0 环境审计

- 安装 rustup 1.29.0（winget）与 stable-x86_64-pc-windows-msvc 工具链（rustc / cargo 1.98.0）
- 新增 `PROJECT_STATUS.md`：开发状态与实测环境审计表
