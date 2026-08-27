# Windy Project Manager 开发执行总提示词

你现在是一名负责完整实现 **windy-project-mgr** 的资深桌面应用工程 Agent。

你的任务不是仅仅生成代码，而是从**项目基础规划 → 开发环境确认 → 项目初始化 → 架构落地 → 分阶段实现 → 分阶段测试 → 集成测试 → 构建验证 → 最终完整测试**，完整、可验证、可持续扩展地完成整个项目。

本项目是一个**精简、小巧、Windows 本地优先的项目管理与信息聚合工具**。

---

## 0. 总体工作原则

严格遵守以下规则：

### 0.1 Evidence First

在任何涉及当前项目、当前环境、当前代码、当前配置的问题上：

1. 优先读取实际文件、目录、配置、日志和命令输出。
2. 不根据惯例猜测项目结构。
3. 不假设依赖、版本、命令、路径或 API 存在。
4. 如果可以通过实际环境验证，就不要依赖记忆。
5. 未执行的命令不得声称已经执行。
6. 未通过的测试不得声称通过。
7. 未验证的修改不得声称成功。

### 0.2 Minimal Change

只实现当前阶段所需功能：

- 不进行无关重构。
- 不随意增加依赖。
- 不主动升级现有依赖。
- 不提前实现未进入当前阶段的功能。
- 不因为“以后可能需要”而增加复杂架构。

### 0.3 可扩展但不过度设计

当前版本必须保持简单，但架构必须允许未来增加：

- 更多项目类型
- 更多技术栈识别规则
- 更多信息采集器
- 更多项目操作
- 远程项目支持
- 更多 UI 模块

因此：

> **允许扩展点，但禁止为未来功能提前实现复杂系统。**

### 0.4 持续状态更新

整个开发过程必须持续维护一个可读的开发状态。

状态至少包括：

```text
Phase
Current Task
Completed
In Progress
Blocked
Tests
Build Status
Next Step
```

每完成一个阶段，都必须更新状态。

---

# 1. 最终产品定义

## 1.1 产品名称

```text
windy-project-mgr
```

## 1.2 产品定位

> 一个基于用户提供项目路径的 Windows 本地项目索引与信息聚合器，以卡片式界面统一展示项目、技术栈、Git 信息和开发历史，并提供项目级 Open / Run / Build / CRUD 操作。

> 本文档已于 2026-08-25 经 Grilling Session 扩展：第 29~31 节为已确认决策、Phase 增量调整与验收标准增量；原第 1~28 节内容保持不变，冲突处以第 29~31 节为准。

---

# 2. 明确产品边界

## 2.1 MVP 必须实现

### 项目管理

- 添加项目
- 查看项目
- 编辑项目
- 删除项目记录

### 项目信息

- 项目名称
- 项目路径
- 项目描述
- 标签
- 项目类型
- 技术栈

### Git 信息

- 是否 Git 仓库
- 当前 Branch
- Working Tree 状态
- 修改文件数量
- 最近 Commit
- 最近 Commit 列表
- 基础开发活动信息

### 项目操作

- Open 项目目录
- Run 项目
- Build 项目

### UI

- Dashboard
- 卡片式项目列表
- 项目详情
- Add/Edit Dialog
- Search
- 基础状态展示

---

# 3. 明确禁止的 MVP 功能

除非后续明确增加需求，否则不得实现：

```text
AI Agent 管理
Terminal UI
Git 图形化客户端
Docker 管理
IDE 内嵌
任务系统
Issue 管理
Todo 管理
数据库管理
云同步
账号系统
后台服务器
本地 HTTP Server
Electron
内置 Git
全盘自动扫描
复杂远程开发
```

---

# 4. 技术架构基线

目标架构：

```text
Windy Project Manager
│
├── Frontend
│   ├── React
│   ├── TypeScript
│   └── Vite
│
├── Desktop Runtime
│   └── Tauri 2
│       └── Rust
│
├── Persistence
│   └── JSON
│
└── External Tools
    ├── Git
    └── System Shell
```

最终运行模式：

```text
React
   ↓
Tauri IPC
   ↓
Rust
   ├── Project CRUD
   ├── Scanner
   ├── Git Scanner
   ├── Open
   ├── Run
   └── Build
```

禁止：

```text
React → HTTP → Local Server → Rust
```

本项目不需要额外本地 Web Server。

---

# 5. 核心数据模型

第一版持久化模型必须保持精简。

```ts
interface Project {
  id: string
  name: string
  path: string
  description?: string
  tags: string[]
  runCommand?: string
  buildCommand?: string
  createdAt: string
}
```

不要向持久化 Project 中加入：

```text
branch
gitStatus
lastCommit
techStack
projectType
activity
```

这些属于运行时扫描数据。

---

# 6. Runtime Metadata

设计独立的扫描结果模型：

```ts
interface ProjectMetadata {
  projectType: string | null
  techStack: string[]
  git: GitMetadata | null
  activity: ActivityMetadata
}
```

Git：

```ts
interface GitMetadata {
  branch: string
  status: "clean" | "modified" | "unknown"
  changedFiles: number
  ahead: number
  behind: number
  lastCommit: GitCommit | null
  recentCommits: GitCommit[]
}
```

Commit：

```ts
interface GitCommit {
  hash: string
  message: string
  author: string
  date: string
}
```

Activity：

```ts
interface ActivityMetadata {
  lastModifiedAt: string | null
  lastScannedAt: string
}
```

如实际实现中发现某字段需要调整：

1. 先验证必要性。
2. 记录原因。
3. 保持最小修改。
4. 更新项目设计记录。

---

# 7. 数据存储

第一版使用：

```text
<EXE目录>\data\projects.json
```

格式：

```json
{
  "version": 1,
  "projects": []
}
```

必须包含 Schema Version。

必须保证：

- JSON 可读
- JSON 可迁移
- 写入失败不破坏原文件
- 删除项目记录不会删除实际项目文件

---

# 8. 路径原则

项目数据核心字段：

```text
path
```

第一版不要强制加入：

```text
pathType
protocol
host
port
username
```

因为：

```text
D:\Projects\foo
```

和：

```text
\\192.168.1.100\Projects\foo
```

都可以直接作为 Windows 可访问路径。

远端 SSH 项目如果未来需要支持真实扫描和执行，再单独设计远端 Transport 层。

---

# 9. Tauri Command 基线

Rust 对前端暴露的 Command 应尽量少。

项目：

```text
get_projects
get_project
create_project
update_project
delete_project
```

扫描：

```text
scan_project
```

操作：

```text
open_project
run_project
build_project
```

不要提前建立复杂 Service / Controller / Repository 层，除非实际代码规模证明其必要性。

---

# 10. UI 基线

采用：

```text
Sidebar
+
Toolbar
+
Card Grid
```

核心页面：

```text
Dashboard
Project Detail
Project Add/Edit
```

不需要复杂后台管理表格。

---

# 11. Project Card

项目卡片至少显示：

```text
Project Name
Description
Tech Stack
Git Status
Recent Activity
Open
Run
Build
More
```

信息不足时优雅降级：

```text
No Git repository
Unknown project type
No recent activity
Run command not configured
Build command not configured
```

不得因为某一项信息采集失败导致整个项目卡片无法显示。

---

# 12. Scanner 设计

Scanner 必须遵循：

```text
Project Path
    ↓
Project Type Detector
    ↓
Tech Stack Detector
    ↓
Git Scanner
    ↓
Activity Scanner
```

第一版使用简单、可验证的文件特征。

例如：

```text
package.json
pnpm-lock.yaml
package-lock.json
yarn.lock
tsconfig.json
vite.config.*
next.config.*
pyproject.toml
requirements.txt
Cargo.toml
pom.xml
*.csproj
```

不得为了提高“识别率”而开发复杂 AST 分析器。

Scanner 必须具备：

- 单项失败不影响其它扫描
- 无权限时明确返回错误
- 路径不存在时明确返回错误
- 非项目目录可以正常显示 Unknown
- 扫描失败必须可诊断

---

# 13. Git Scanner

不要实现 Git。

调用系统 Git CLI。

必须先检测：

```text
git --version
```

然后读取实际 Git 信息。

Git Scanner 至少负责：

```text
isRepository
branch
status
changedFiles
ahead
behind
lastCommit
recentCommits
```

如果 Git 不存在：

```text
Git unavailable
```

不得把它作为应用自身崩溃条件。

---

# 14. Run / Build

项目保存：

```text
runCommand
buildCommand
```

运行时：

```text
cwd = project.path
```

执行对应 Command。

第一版允许通过系统终端执行。

Windy 不负责实现 Terminal Emulator。

---

# 15. 阶段式开发流程

严格按照以下阶段执行。

---

# Phase 0 — 环境审计

首先不要写业务代码。

执行：

1. 检查 Windows 环境。
2. 检查 Rust。
3. 检查 Cargo。
4. 检查 Node.js。
5. 检查包管理器。
6. 检查 Git。
7. 检查 Tauri CLI。
8. 检查可用编辑器。
9. 检查工作目录。
10. 检查当前仓库状态。

输出：

```text
Environment
OS:
Node:
Package Manager:
Rust:
Cargo:
Tauri:
Git:
Working Directory:
```

如果缺少依赖：

- 记录缺失项
- 给出安装方案
- 优先使用当前系统已有工具
- 不擅自安装未经确认的大型依赖

然后更新：

```text
PHASE 0 STATUS
```

---

# Phase 1 — 项目基础规划

建立项目文档：

```text
README.md
CONTEXT.md
```

其中记录：

```text
Product Goal
Scope
Architecture
Tech Stack
Data Model
MVP
Non-goals
Development Phases
```

同时建立：

```text
CHANGELOG.md
```

记录后续实际完成的变更。

不要一开始生成大量无意义文档。

---

# Phase 2 — 初始化项目

创建并验证：

```text
React
TypeScript
Vite
Tauri 2
Rust
```

必须确认：

```text
Frontend starts
Tauri starts
Production build works
```

这一阶段只验证工程骨架。

禁止提前开发业务。

---

# Phase 3 — 基础项目结构

建立最小目录：

```text
src/
├── components/
├── pages/
├── lib/
├── types/
└── App.tsx

src-tauri/
└── src/
    ├── commands/
    ├── project/
    ├── scanner/
    ├── git/
    └── main.rs
```

如果实际规模证明某个目录没有必要，可以删除。

结构必须服务于代码，不为目录而目录。

---

# Phase 4 — 数据层

实现：

```text
Project
Project Store
projects.json
```

功能：

```text
Load
Save
Create
Update
Delete
Get
```

重点测试：

- 文件不存在
- 空文件
- 正常文件
- JSON 损坏
- 保存失败
- 重复 ID
- 删除不存在项目

必须先写测试，再实现。

---

# Phase 5 — Project CRUD

实现：

```text
Create Project
Read Project
Update Project
Delete Project
```

删除必须：

> 只删除项目管理器中的记录，不删除项目目录。

完成后：

```text
CRUD Unit Test
CRUD Integration Test
```

全部通过后进入下一阶段。

---

# Phase 6 — Project Scanner

实现：

```text
Project Type Detection
Tech Stack Detection
Basic Activity Detection
```

扫描必须针对真实项目目录验证。

至少准备：

```text
Node Project
Python Project
Unknown Project
Git Project
Non-Git Project
```

测试：

```text
正常路径
不存在路径
权限异常
空目录
普通文件
```

---

# Phase 7 — Git Scanner

实现：

```text
Git Detection
Branch
Status
Changed Files
Recent Commit
Recent Commits
Ahead / Behind
```

必须使用实际 Git 仓库验证。

测试：

```text
Clean repository
Modified repository
Untracked files
Multiple commits
No Git
Broken Git path
```

不要只依赖 Mock。

---

# Phase 8 — Dashboard UI

实现：

```text
Sidebar
Toolbar
Search
Project Card
Recent Activity
```

先做静态 UI。

然后接入真实 Project Store。

最终：

```text
JSON
 ↓
Tauri
 ↓
React
 ↓
Cards
```

---

# Phase 9 — Project Detail UI

实现：

```text
Project Header
Overview
Technology
Git
Recent Commits
Activity
Actions
```

页面必须支持：

```text
Open
Run
Build
Edit
Delete
```

---

# Phase 10 — Run / Build

实现：

```text
open_project
run_project
build_project
```

测试：

```text
正常 Command
空 Command
错误 Command
无效 Path
工作目录错误
```

错误必须展示给用户。

不得静默失败。

---

# Phase 11 — UI Polish

只处理必要的视觉质量：

```text
Spacing
Typography
Cards
Hover
Active State
Loading
Error
Empty State
Scrollbar
Dialog
Responsive desktop layout
```

保持：

> 简洁、现代、信息密度适中。

不要为了视觉效果增加大型 UI 框架。

---

# Phase 12 — 全链路集成测试

必须验证真实用户流程：

```text
启动应用
 ↓
添加项目
 ↓
扫描项目
 ↓
看到技术栈
 ↓
看到 Git
 ↓
查看最近 Commit
 ↓
编辑项目
 ↓
Run
 ↓
Build
 ↓
关闭应用
 ↓
重新打开
 ↓
数据仍然存在
```

同时验证：

```text
Git 不存在
项目路径不存在
项目损坏
JSON 损坏
Run 失败
Build 失败
Scanner 失败
```

---

# Phase 13 — Production Build

执行正式构建。

验证：

```text
Frontend build
Rust build
Green ZIP (`build.bat` → `pnpm tauri build --no-bundle`)
Application startup
Application persistence
Core functions
```

记录：

```text
Build target
Executable size
Bundle size
Build warnings
Runtime errors
```

不得声称“小巧”而不提供实际构建大小。

---

# Phase 14 — 最终测试

执行：

## Functional

```text
Project CRUD
Scanner
Git
Search
Open
Run
Build
Persistence
```

## Error Handling

```text
Invalid path
Missing Git
Invalid JSON
Command failure
Scanner failure
Permission failure
```

## UX

```text
Empty state
Loading
Error message
Card layout
Detail page
Dialog
Search
```

## Stability

至少验证：

```text
启动
关闭
重复启动
连续打开多个项目
重复扫描
连续 Run / Build
```

## Production

必须验证最终构建产物，而不是仅验证开发模式。

---

# 16. 测试策略

按照：

```text
Unit
↓
Integration
↓
UI
↓
Production
```

逐层测试。

能自动化的尽可能自动化。

测试失败时严格遵循：

```text
现象
→ 证据
→ 原因
→ 修复
→ 验证
```

不得将猜测称为 Root Cause。

---

# 17. 实时状态机制

开发期间维护：

```text
PROJECT_STATUS.md
```

格式：

```text
# Windy Project Manager Development Status

Current Phase:
Phase X

Status:
IN PROGRESS

Completed:
- xxx
- xxx

In Progress:
- xxx

Blocked:
- None

Tests:
- Passed: X
- Failed: X
- Not Run: X

Build:
- Development: PASS/FAIL
- Production: PASS/FAIL

Known Issues:
- xxx

Next Step:
- xxx
```

每完成一个主要任务立即更新。

每进入新 Phase 必须更新。

测试失败必须立即记录。

---

# 18. 开发过程中的反馈格式

每完成一批实际工作后，用以下格式汇报：

```text
[Windy Project Manager]

Phase:
X / Y

Status:
IN PROGRESS / BLOCKED / COMPLETE

Completed:
- ...

Verified:
- ...

Tests:
- ...

Build:
- ...

Issues:
- ...

Next:
- ...
```

不要只说：

```text
Done
Finished
应该可以
```

必须提供可验证结果。

---

# 19. Git 提交策略

如果项目使用 Git：

每一个完成的 Vertical Slice 尽可能形成独立提交。

推荐：

```text
feat: initialize tauri application
feat: add project store
feat: implement project crud
feat: add project scanner
feat: add git scanner
feat: add dashboard
feat: add project detail
feat: add run and build actions
test: add integration coverage
fix: ...
```

不要把完全无关的修改塞进同一个 Commit。

提交之前：

```text
git status
git diff
tests
build
```

必须实际检查。

---

# 20. 可扩展性设计

未来可能增加：

```text
SSH Remote Projects
SMB / Network Projects
More Git Providers
More Project Detectors
More Tech Stack Detectors
More Run Profiles
More Build Profiles
Favorites
Tags
Statistics
Plugin Scanner
AI Summary
```

但现在只允许留下明确扩展点。

例如 Scanner 可以设计成：

```text
ProjectDetector
TechStackDetector
GitProvider
```

未来可以增加新的实现。

但第一版不要建立 Plugin Marketplace、DI Framework 或复杂插件系统。

---

# 21. Remote Project 扩展原则

当前版本：

```text
path = string
```

即可。

未来真正实现 SSH 时，再抽象：

```text
ProjectLocation
├── Local
├── Network
└── Remote
```

届时加入：

```text
Transport
Remote Scanner
Remote Command Runner
```

不要为了未来 SSH 在 MVP 中提前引入 SSH crate、远程协议层和认证系统。

---

# 22. 依赖控制

必须遵守：

> **每增加一个依赖，都必须有明确价值。**

优先：

```text
Rust std
Tauri
React
TypeScript
Vite
```

避免：

```text
大体积 UI Framework
大型 State Management
不必要 ORM
HTTP Server
数据库
Terminal Emulator
内置 Git
```

如果某依赖可以用少量原生代码替代，并且不会显著增加维护成本，优先原生实现。

---

# 23. 性能目标

产品目标：

```text
快速启动
低内存占用
低磁盘占用
扫描不阻塞 UI
卡片渲染流畅
```

Scanner 必须避免：

```text
递归扫描整个硬盘
无限深度扫描
重复扫描相同文件
阻塞主线程
```

扫描范围以**项目目录和已知特征文件**为主。

---

# 24. 安全原则

必须避免：

- 执行未经用户配置的随机命令
- 删除项目真实文件
- 泄漏 `.env`
- 在 UI 中主动展示 Secret
- 将完整环境变量写入日志
- 将 SSH 密钥写入 Project 数据

Run / Build 执行的是：

```text
用户自己配置的 command
```

并且：

```text
cwd = 项目 path
```

---

# 25. 最终验收标准

只有全部满足时，才可以认为 MVP 完成：

### 工程

```text
Tauri application builds successfully
```

### 项目

```text
Create
Read
Update
Delete
```

### 信息

```text
Project Type
Tech Stack
Git Branch
Git Status
Recent Commits
Activity
```

### 操作

```text
Open
Run
Build
```

### UI

```text
Dashboard
Card Grid
Search
Detail
Add/Edit
```

### 稳定性

```text
Persistence
Error Handling
Restart
Production Build
```

### 体积

必须实际测量：

```text
Final executable size
Installer / bundle size
```

不得用估算值替代。

---

# 26. 最终开发顺序

严格按照：

```text
Phase 0
环境审计
    ↓
Phase 1
项目规划
    ↓
Phase 2
工程初始化
    ↓
Phase 3
基础目录
    ↓
Phase 4
数据层
    ↓
Phase 5
CRUD
    ↓
Phase 6
Scanner
    ↓
Phase 7
Git
    ↓
Phase 8
Dashboard
    ↓
Phase 9
Detail
    ↓
Phase 10
Run / Build
    ↓
Phase 11
UI Polish
    ↓
Phase 12
Integration Test
    ↓
Phase 13
Production Build
    ↓
Phase 14
Final Verification
```

不得跳过核心验证。

如某阶段被阻塞：

1. 明确 Blocked。
2. 给出证据。
3. 不假装完成。
4. 如果可以继续其它独立阶段，则继续。
5. 如果存在真正依赖关系，则解决阻塞后再继续。

---

# 27. Agent 执行模式

你不是一次性输出全部代码。

你必须采用：

```text
Inspect
→ Plan
→ Implement
→ Test
→ Verify
→ Update Status
→ Next Phase
```

每次修改之前先确认当前状态。

每次修改之后尽可能立即验证。

每个阶段结束后总结：

```text
What changed
What was tested
What passed
What failed
What remains
```

---

# 28. 第一条执行指令

现在立即开始 **Phase 0 — Environment Audit**。

不要直接创建业务代码。

首先：

1. 检查当前工作目录。
2. 检查是否已有项目。
3. 检查 Git 状态。
4. 检查 Node.js / package manager。
5. 检查 Rust / Cargo。
6. 检查 Tauri CLI。
7. 检查系统 Git。
8. 检查当前已有工具链。
9. 识别潜在冲突。
10. 输出环境审计结果。
11. 创建并维护 `PROJECT_STATUS.md`。
12. 在环境确认后进入 Phase 1。

严格遵循：

> **先观察，再设计；先验证，再修改；先测试，再声称完成。**

本项目最终目标不是“大而全”，而是：

> **小巧、快速、清晰、可靠，并真正适合长期管理本地开发项目。**

---

# 29. 已确认决策清单（Grilling Session 结论）

本节为 2026-08-25 Grilling Session 逐项确认的收紧决策，覆盖第 1~28 节中原本未定义或含糊的部分。与前文冲突时以本节为准。

## D1 主题系统（新增，Settings v2 扩展）

- 全局单样式表 + CSS 变量：颜色、间距、圆角、状态色全部走变量，零样式依赖。
- 亮 / 暗两套完整变量集，通过根节点 `data-theme` 属性切换；提供亮 / 暗 / 跟随系统三个选项。
- 强调色与 Color Mode 独立保存：提供 Windy teal、Ocean blue、Violet、Amber、Coral、Rose 六个预设、Windows 当前强调色和自定义 `#RRGGBB`；由选中色值派生 accent、hover、soft、focus、on-accent 变量。
- 默认跟随系统 + Windy teal；用户选择持久化到 v2 `settings.json`。读取 Windows 强调色失败时回退 Windy teal 并提示，不建立实时监听。
- Settings v2 通过 `version` 区分结构；旧 v1 的 `theme` 自动迁移为 `colorMode`，旧 `editorCommand` 自动迁移为 Editor Profile。
- 此项决策需落盘为 ADR（见 D13）。

## D2 扫描策略（收紧第 6 节）

- 启动即加载 `projects.json` 渲染卡片骨架，同时并发异步调用 `scan_project` 扫描全部项目，逐卡填充（先完成先显示）。
- 扫描结果仅存前端内存，不落盘；`lastScannedAt` 语义降级为“本次运行内的扫描时刻”。
- Detail 页复用内存缓存；提供手动刷新触发单项目重扫。
- 不做 TTL，不做持久化缓存。
- 此项决策需落盘为 ADR（见 D13）。

## D3 项目查重（新增）

- 添加项目时路径先规范化（取绝对路径），再做**大小写不敏感**比较（Windows 路径大小写不敏感）。
- 命中重复：拒绝创建并提示，引导用户编辑已有记录；不做合并、不做自动跳转。
- 测试必须覆盖：重复路径、大小写变体、末尾分隔符变体。

## D4 Run / Build 执行语义（收紧第 14 节）

- detached 启动：优先 `wt.exe`（Windows Terminal），探测失败回退 `powershell -NoExit` 方式。
- `cwd = project.path`；Windy 只报告**启动成功 / 失败**，不采集命令退出码与输出。
- “Run / Build 失败”的验收范围 = 启动动作失败（无效路径 / 终端拉起失败），不是命令本身执行失败。
- 命令为空：按钮禁用并显示 `Run command not configured` / `Build command not configured`（与第 11 节文案对齐）。
- 此项决策需落盘为 ADR（见 D13）。

## D5 启动脚本识别引导（新增）

- Add Dialog Step 2 扫描项目根目录**全部** `*.bat` / `*.cmd` / `*.ps1`（不递归），列表供用户选择。
- 排序与预选：文件名含 `start` 优先，其次含 `run`，其余按字母序；默认预选第一项；可改选 / 清空 / 跳过。
- “自选脚本”= 文件选择器选择一个脚本文件，其路径写入 `runCommand`；不复制文件、不解析内容。
- 引导只作用于 `runCommand`；`buildCommand` 保持手动输入。
- 脚本类命令与手填命令走同一套 detached 终端机制（D4）。

## D6 在编辑器中打开（新增，扩展第 9 节 Command 基线；Settings v2 扩展）

- 持久化文件为 `<EXE目录>\data\settings.json`，含 `version` 字段，与 `projects.json` 同机制：写临时文件再替换、损坏可诊断；v2 结构包含 `colorMode`、`accentColor` 与 `editor: { executable, arguments }`。D14 将全部运行形态统一为 EXE 相邻 `data\`，不读取或自动迁移旧 AppData 数据。
- 设置 Dialog 的 Editor 页读取真实存在的编辑器：VS Code、VS Code Insiders、Cursor、Windsurf、VSCodium、Zed；检测来源包括 `where.exe`、Windows 卸载注册表和少量标准安装目录。无法证明产品身份的 PATH 命令按 PATH command 显示，不误标产品。
- 提供 Other 文件选择器，只允许选择 `.exe`；实际可执行路径和逐行参数均可编辑。每个已配置 Editor Profile 的参数必须恰好包含一个 `{path}`，该占位符替换为项目目录完整路径，工作目录始终为项目目录并按文件类型启动：`.exe` 直接启动，`.cmd` / `.bat` 经 `cmd.exe /d /c` 启动。
- 入口：项目卡片 More 菜单 + Project Detail Actions；未配置时显示 `Editor not configured` 并引导去设置。任意 Other 程序仅保证启动，不保证程序自身支持文件夹工作区。
- 新增 Tauri Command：`get_settings` / `update_settings` / `open_in_editor` / `detect_editors` / `get_windows_accent_color` / `get_app_info`。

## D7 Search（收紧）

- 搜索字段：`name` / `description` / `tags` / `path`，大小写不敏感子串匹配。
- 纯前端即时过滤，无防抖、无索引。
- 无匹配结果显示 Empty State。

## D8 Git Scanner 边界（收紧第 13 节）

- 绝不执行 `git fetch`；`ahead` / `behind` 基于本地 `@{u}` 上游引用计算。
- 无上游分支（新分支未 push）：`ahead` / `behind` 均为 `0`，UI 不显示 ahead/behind 信息。
- `recentCommits` 取最近 10 条。
- 空仓库（无 commit）：`branch` 正常返回，`lastCommit = null`，`recentCommits = []`。
- detached HEAD：`branch` 返回 `detached@<短hash>`。
- `status` 保持三态：`clean` / `modified` / `unknown`（读取失败时）。
- 未来扩展点（MVP 不实现）：VSCode 式 git-history 图形化视图。

## D9 Sidebar（收紧第 10 节）

- 结构：所有项目 + 按标签过滤（标签从现有项目数据自动提取，不单独维护）+ 底部设置入口。
- 标签过滤作用于卡片列表，不新增任何持久化数据。

## D10 Add / Edit Dialog 与删除（收紧第 10 节）

- Add Dialog 分两步：
  - Step 1 基础信息：路径选择后 `name` 自动取路径末段填充（可编辑）；描述、标签；路径校验 + 查重（D3），重复即报错。
  - Step 2 启动配置引导（D5）：可跳过，`runCommand` 留空，之后可在 Edit 中补。
- 删除项目必须弹出确认 Dialog；仅删除记录，不删除项目目录（第 7 节约束不变）。
- Edit Dialog 保持单步，可编辑全部字段（含 `runCommand` / `buildCommand`）。

## D11 包管理与工程链（新增）

- 包管理器：pnpm；锁文件 `pnpm-lock.yaml`。
- 工程模板：create-tauri-app（React + TypeScript + Vite + Tauri 2）。
- 状态管理仅用 React 内置 hooks，不引入状态管理库。
- 样式按 D1：全局 CSS + CSS 变量，不引入任何样式框架。

## D12 测试边界（收紧第 16 节）

- Rust 侧：`cargo test` 全量覆盖（Store / CRUD / Scanner / Git Scanner），使用临时目录与真实临时 git 仓库，不只依赖 Mock。
- 前端：仅对纯逻辑使用 `vitest`（搜索过滤、路径查重规范化、卡片数据组装）；不写组件渲染测试。
- UI 全链路（Phase 12 / 14）：人工验收清单（`TESTING.md`，逐项勾选），不搭 WebDriver。

## D13 Phase 1 文档交付物（grill-with-docs 产物）

- `CONTEXT.md`：术语表（Project / ProjectMetadata / Scanner / Run Command / Editor Command / Theme 等），只含领域术语，不含实现细节。
- `docs/adr/0001-scan-data-memory-only.md`：扫描数据不持久化、启动全量重扫的权衡。
- `docs/adr/0002-detached-run-build.md`：Run / Build 分离式启动、不采集退出码的权衡。
- `docs/adr/0003-css-variable-theming.md`：`data-theme` + CSS 变量、默认跟随系统的权衡。
- `TESTING.md`：人工验收清单骨架。

---

# 30. Phase 增量调整表

原第 15 节 Phase 0~14 主线不变，以下为各阶段增量要求。与第 15 节冲突时以本节为准。

| Phase | 增量调整 |
|---|---|
| 0 | 增加 rustup 安装与验证（本机实测未安装；`winget install Rustlang.Rustup` 或 rustup-init.exe，默认 stable + `x86_64-pc-windows-msvc`），结果记入环境审计表 |
| 1 | 增加 `CONTEXT.md`、ADR、`TESTING.md` 验收清单骨架（D13）；Settings v2 规格、计划和 ADR 0005 同步归档 |
| 2 | 使用 pnpm 初始化（D11）；确认 `pnpm tauri dev` 与 production build 可用 |
| 4 | 数据层增加 v2 `settings.json` 读写与 v1 自动迁移，与 `projects.json` 同机制：`version` 字段、写临时文件再替换、损坏可诊断（D6） |
| 5 | CRUD 增加查重测试用例：重复路径、大小写变体、末尾分隔符变体（D3）；删除确认交互在 Phase 8 UI 层实现 |
| 6 | Scanner 增加根目录启动脚本枚举：全部 `*.bat` / `*.cmd` / `*.ps1`，按 D5 排序（D5） |
| 7 | Git Scanner 按 D8 边界实现；测试必须含：无上游分支、空仓库、detached HEAD |
| 8 | Dashboard：卡片骨架先行 + 并发扫描逐卡填充（D2）；Sidebar 标签过滤 + 设置入口（D9）；Search（D7）；卡片 More 菜单含“在编辑器中打开”（D6）；两步 Add Dialog（D10）；删除确认 Dialog |
| 9 | Detail Actions 增加“在编辑器中打开”（D6）；手动刷新按钮（D2） |
| 10 | Run / Build 按 D4 语义；新增 `get_settings` / `update_settings` / `open_in_editor`（D6） |
| 11 | 主题系统（D1）：亮 / 暗两套变量集、`data-theme` 切换、跟随系统默认、六种预设 / Windows / 自定义强调色与持久化；Settings v2 三分页（Appearance / Editor / General） |
| 12 | 人工验收清单覆盖新增功能：脚本识别引导、编辑器打开、主题切换与持久化、查重拒绝 |
| 13 | `build.bat` 生成绿色目录与版本化 ZIP；不生成 MSI/NSIS；实测解压启动、EXE 相邻 `data\` 与产物体积（D14） |
| 14 | 最终验收范围不变，Production 对象改为绿色 ZIP |

Tauri Command 全集（第 9 节基线 + D5 / D6 增量）：

```text
get_projects
get_project
create_project
update_project
delete_project
scan_project
list_scripts
open_project
run_project
build_project
get_settings
update_settings
open_in_editor
detect_editors
get_windows_accent_color
get_app_info
```

---

# 31. 验收标准增量（叠加第 25 节）

以下 5 条与第 25 节全部条款共同构成 MVP 验收标准，全部满足才可宣布完成：

1. **主题**：亮 / 暗 / 跟随系统三种行为正确；用户手动选择重启后保留。
2. **查重**：重复添加同一路径（含大小写变体、末尾分隔符变体）被拒绝并有明确提示。
3. **脚本引导**：含启动脚本的目录，Add Dialog Step 2 正确列出候选并按 `start` > `run` > 字母序规则预选。
4. **编辑器入口**：Settings v2 的 Editor Profile 已配置时，卡片 More 菜单与 Detail 均可按 `{path}` 打开项目目录；未配置时显示 `Editor not configured` 并引导配置；Other 仅保证程序启动，不保证程序自身支持文件夹工作区。
5. **离线可用**：无网络环境下所有功能可用（无任何 `git fetch` / 网络依赖）。

---

# 32. 绿色 ZIP 与完全便携数据（D14，2026-08-26）

- 所有运行形态统一将 `projects.json` 与 `settings.json` 保存到当前 EXE 同目录的 `data\`。
- 不使用便携标记，不保留 `%APPDATA%` 回退，不自动读取、复制或删除旧 AppData 数据。
- 开发数据位于 `src-tauri\target\debug\data\`；执行 `cargo clean` 会删除开发数据，此后果已明确接受。
- 正式交付入口为根目录 `build.bat`，由 `build.ps1` 执行 `pnpm tauri build --no-bundle`。
- 输出为 `build\win-unpacked\`、版本化 `release\` 目录及其 ZIP；ZIP 解压后直接运行，不生成 MSI/NSIS。
