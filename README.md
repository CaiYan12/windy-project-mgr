# Windy Project Manager

一个精简、小巧、**Windows 本地优先的项目管理与信息聚合工具**。基于用户提供的项目路径，以卡片式界面统一展示项目、技术栈、Git 信息和开发历史，并提供项目级 Open / Run / Build 操作。

## 功能（MVP）

- **项目管理**：添加 / 查看 / 编辑 / 删除项目记录（删除仅移除记录，不删除项目目录）
- **信息聚合**：项目类型与技术栈自动识别、Git 分支 / 状态 / 最近提交、最近活动
- **项目操作**：打开目录、Run、Build（分离式终端启动）、在编辑器中打开（VS Code / Cursor 等）
- **界面**：Dashboard 卡片列表、项目详情、标签过滤、搜索、亮 / 暗 / 跟随系统主题
- **添加引导**：自动扫描项目根目录启动脚本（`*.bat` / `*.cmd` / `*.ps1`）并辅助配置启动命令

## 技术架构

```text
React + TypeScript + Vite
        ↓ Tauri IPC
Rust (Tauri 2)
  ├── Project CRUD（projects.json）
  ├── Settings（settings.json）
  ├── Scanner（项目类型 / 技术栈 / 活动 / 启动脚本枚举）
  ├── Git Scanner（调用系统 Git CLI，不实现 Git、不联网）
  └── Open / Run / Build / Open in Editor
```

关键设计决策（详见 `docs/PLAN.MD` 第 4 节）：

- 扫描结果仅存内存，启动时并发扫描、逐卡填充，不落盘
- Run / Build 通过 `wt.exe`（回退 PowerShell）分离式启动，只报告启动成败
- 样式为全局 CSS + CSS 变量，零样式依赖，主题默认跟随系统
- 持久化仅 `%APPDATA%\windy-project-mgr\` 下两个带版本号的 JSON 文件

## 文档

| 文档 | 说明 |
|---|---|
| [docs/Windy Project Manager - Primary Request&Plan Document.md](./docs/Windy%20Project%20Manager%20-%20Primary%20Request&Plan%20Document.md) | 原始需求与完整开发规范（第 0~28 节基线 + 第 29~31 节决策扩展） |
| [docs/PLAN.MD](./docs/PLAN.MD) | 自包含的最终执行计划：决策清单、Phase 0~14、验收标准、进度追踪清单 |

## 当前状态

项目处于**规划完成、尚未初始化代码**阶段。开发将按 `docs/PLAN.MD` 的 Phase 0~14 顺序执行，进度由该文档第 0 节的 Checkbox 清单追踪。

工具链前置（2026-08-25 实测）：Node v24.18.0、pnpm 10.26.2、Git 2.48.1、Visual Studio Community 2022（VC x86/x64）已就绪；Rust 已安装（rustup 1.29.0，rustc / cargo 1.98.0，stable-x86_64-pc-windows-msvc），完整审计表见 `PROJECT_STATUS.md`。

## TODO

后续待开发内容（MVP 之外，均已明确排除在当前版本范围外）：

- 主题预设与主题编辑机制（亮/暗之外的多主题预设）
- VSCode 式 git-history 图形化视图
- SSH 远程项目支持（届时引入 `ProjectLocation` 抽象：Local / Network / Remote 与 Transport 层）
- SMB / 网络路径项目的深度支持
- 更多项目类型与技术栈识别规则（Detector 扩展点）
- 多 Run / Build Profile
- 收藏（Favorites）与统计（Statistics）
- Plugin Scanner
- AI Summary
- 扫描结果持久化缓存与 TTL
