# Windy Project Manager

一个精简、小巧、**Windows 本地优先的项目管理与信息聚合工具**。基于用户提供的项目路径，以卡片式界面统一展示项目、技术栈、Git 信息和开发历史，并提供项目级 Open / Run / Build 操作。

## 功能（MVP）

- **项目管理**：添加 / 查看 / 编辑 / 删除项目记录（删除仅移除记录，不删除项目目录）
- **信息聚合**：项目类型与技术栈自动识别、Git 分支 / 状态 / 最近提交、最近活动
- **项目操作**：打开目录、Run、Build（分离式终端启动）、在编辑器中打开（真实编辑器发现、Other `.exe` 与文件夹工作区参数）
- **界面**：Dashboard 卡片列表、项目详情、标签过滤、搜索、亮 / 暗 / 跟随系统主题、六种强调色预设与自定义颜色
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
- 持久化仅当前 EXE 同目录 `data\` 下两个带版本号的 JSON 文件；整个程序目录可移动和备份

## 构建绿色版

```powershell
.\build.bat
```

脚本执行 Tauri release 无安装器构建，并生成：

- `build\win-unpacked\`：可直接运行的绿色目录
- `release\windy-project-mgr-<version>-win32-x64\`：版本化绿色目录
- `release\windy-project-mgr-<version>-win32-x64.zip`：解压即用的正式交付物

项目与设置数据写入绿色目录内的 `data\`；不读取或迁移 `%APPDATA%` 旧数据。开发模式同样使用 EXE 相邻 `data\`，因此 `cargo clean` 会删除开发数据。

## 文档

| 文档 | 说明 |
|---|---|
| [docs/Windy Project Manager - Primary Request&Plan Document.md](./docs/Windy%20Project%20Manager%20-%20Primary%20Request&Plan%20Document.md) | 原始需求与完整开发规范（第 0~28 节基线 + 第 29~31 节决策扩展） |
| [docs/PLAN.MD](./docs/PLAN.MD) | 自包含的最终执行计划：决策清单、Phase 0~14、验收标准、进度追踪清单 |

## 当前状态

项目已完成 MVP 最终测试与验收；Phase 0~14 全部完成，完整进度与验收记录见 `docs/PLAN.MD` 第 0 节和 `PROJECT_STATUS.md`。

工具链前置（2026-08-25 实测）：Node v24.18.0、pnpm 10.26.2、Git 2.48.1、Visual Studio Community 2022（VC x86/x64）已就绪；Rust 已安装（rustup 1.29.0，rustc / cargo 1.98.0，stable-x86_64-pc-windows-msvc），完整审计表见 `PROJECT_STATUS.md`。

## TODO

后续待开发内容（MVP 之外，均已明确排除在当前版本范围外）：

- 完整主题预设与主题编辑机制（当前版本已支持六种强调色预设、Windows 色和自定义色；完整主题编辑仍未实现）
- VSCode 式 git-history 图形化视图
- SSH 远程项目支持（届时引入 `ProjectLocation` 抽象：Local / Network / Remote 与 Transport 层）
- SMB / 网络路径项目的深度支持
- 更多项目类型与技术栈识别规则（Detector 扩展点）
- 多 Run / Build Profile
- 收藏（Favorites）与统计（Statistics）
- Plugin Scanner
- AI Summary
- 扫描结果持久化缓存与 TTL
