# Windy Project Manager — 完整 MVP 执行计划

> 本文档为自包含的最终执行计划：原始需求见根目录《Windy Project Manager - Primary Request&Plan Document.md》（下称"原始文档"），本文档已合并 D1~D14。执行时以本文档为准；与原始文档冲突处以本文档及已接受 ADR 为准。

---

## 0. 执行进度追踪（Checkbox）

> **执行进度：16 / 16 Phase 完成 · 37 / 37 子任务（最后更新：2026-10-05；本次同步领域词汇表文件名与引用，复选框和任务计数未变；Phase 15 图标细节与 Git 扫描窗口回归修复已完成并生成 build 产物，MVP 交付闭环。2026-08-30 post-MVP：结构级 UI 焕新（网易云语法）+ settings v3 主题工坊 + 自绘滚动条完成并验收，vitest 87 / cargo 155 / build / 四档宽度亮暗视觉验收通过；2026-09-12 已用正确的 `node_repl + @oai/sky` 执行 2026-09-11 架构加固真实窗口 A–G 验收，B3 修复后定向复测通过，A1 首帧骨架未留证，G3 因约束未检查，G1/G2 通过，未勾选任何新任务，详见 PROJECT_STATUS）**

### Phase 0 环境审计

- [x] rustup 安装完成（stable + `x86_64-pc-windows-msvc`）
- [x] `rustc --version` / `cargo --version` 验证通过，环境审计表记入 `PROJECT_STATUS.md`

### Phase 1 项目规划

- [x] `README.md` / `GLOSSARY.md` / `CHANGELOG.md` 建立
- [x] 基础 3 份 ADR 落盘（`docs/adr/0001~0003`，D13）及 Settings v2 ADR 0005
- [x] `TESTING.md` 人工验收清单骨架建立

### Phase 2 工程初始化

- [x] create-tauri-app 工程创建（pnpm + React + TS + Vite + Tauri 2，D11）
- [x] `pnpm tauri dev` 启动验证通过
- [x] production build 验证通过

### Phase 3 基础目录

- [x] `src/` 与 `src-tauri/src/` 目录结构建立并可编译

### Phase 4 数据层

- [x] `projects.json` Store（Load/Save/Create/Update/Delete/Get），先写测试再实现，全部测试通过
- [x] `settings.json` 读写（同机制：`version` / 临时文件替换 / 损坏可诊断，D6）

### Phase 5 Project CRUD

- [x] CRUD 单元 + 集成测试全部通过；删除仅删记录不删目录
- [x] 查重测试用例通过：重复路径 / 大小写变体 / 末尾分隔符变体（D3）

### Phase 6 Project Scanner

- [x] 项目类型 / 技术栈 / 活动检测实现，真实目录验证（Node/Python/Unknown/Git/Non-Git）
- [x] 根目录启动脚本枚举：全部 `*.bat` / `*.cmd` / `*.ps1`，按 start > run > 字母序排序（D5）

### Phase 7 Git Scanner

- [x] 按 D8 边界实现（不联网、三态 status、最近 10 条、空仓库 / detached HEAD 降级），真实仓库验证通过
- [x] 专项测试通过：无上游分支 / 空仓库 / detached HEAD

### Phase 8 Dashboard UI

- [x] 卡片骨架先行 + 并发扫描逐卡填充（D2）
- [x] Sidebar：所有项目 + 标签过滤 + 设置入口（D9）
- [x] Search 四字段即时过滤（D7）
- [x] 两步 Add Dialog（含脚本引导与查重，D10/D5）与删除确认 Dialog
- [x] 卡片 More 菜单（含“在编辑器中打开”，D6）

### Phase 9 Project Detail UI

- [x] Header / Overview / Tech / Git / Commits / Activity / Actions 全部呈现，降级文案正确
- [x] Actions 含“在编辑器中打开” + 手动刷新按钮（D6/D2）

### Phase 10 Run / Build / Open / Editor

- [x] `open_project` / `run_project` / `build_project` 按 D4 语义（wt 优先 / PowerShell 回退，只报启动成败）
- [x] `get_settings` / `update_settings` / `open_in_editor` 实现并测试（D6）
  - Task 3 fix3 2026-08-27 已修复 batch 参数边界与 PowerShell 路径分离边界；安全契约和证据见 `.superpowers/sdd/2026-08-27-settings-overhaul/task-3-fix3-report.md`。

### Phase 11 UI Polish + 主题系统

- [x] 亮 / 暗两套 CSS 变量集 + `data-theme` 切换 + 默认跟随系统（D1）
- [x] 设置 Dialog v2：Appearance / Editor / General 三分页、强调色预览、真实编辑器发现与 General 信息/重置界面；已完成 strict re-review 修复，并经真实 Tauri 窗口验收（证据见 `.superpowers/sdd/2026-08-27-settings-overhaul/task-5-rereview2.md`、`task-7-report.md`）
- [x] 主题手动选择重启后保留（持久化到 `settings.json`）

### Phase 12 全链路集成测试

- [x] `TESTING.md` 人工验收清单逐项勾选通过（含脚本引导 / 编辑器打开 / 主题持久化 / 查重拒绝）

### Phase 13 Production Build

- [x] `build.bat` 绿色 ZIP 构建成功；解压 EXE 启动、便携数据路径与产物体积实测记录

### Phase 14 最终测试与验收

- [x] Functional / Error Handling / UX / Stability / Production 全部通过（原始第 25 节；最终人工验收由用户完成）
- [x] 5 条增量验收项全部通过（本文档第 7 节；最终人工验收由用户完成）
- [x] 顶部“执行进度”更新为 15 / 15，MVP 宣布完成

### Phase 15 自绘 SVG 图标系统与全页面 UI 审计

- [x] 建立零依赖自绘 SVG `Icon` 注册表并接入 Dashboard、Detail、Card、Dialog、Settings、Toast、空态、错误态与加载态
- [x] 完成亮 / 暗 / 跟随系统与强调色适配、Git 状态形状与颜色双重表达、焦点命中区、长文本收缩和 320 / 768 / 1024 / 1440 静态响应式几何审计
- [x] 完成图标静态契约测试、真实 Tauri 默认窗口交互复核、`pnpm test`、`pnpm build`、`git diff --check` 验证并同步交付文档；2026-08-28 按需求采用指定 1024×1024 Build path，归一到 24×24 / `currentColor` 并通过 `build.bat`

> 2026-08-28 维护修复（不新增 Phase / 子任务）：Git 扫描的 Windows 子进程统一使用 `CREATE_NO_WINDOW`；真实绿色 EXE 的 Add Project 观察到 Git 子进程均为 `hwnd=0`，Detail Refresh 的 3 秒 UIA 观察无新增顶层窗口，新增回归测试已完成红→绿。

---

## 0.1 状态自动更新协议（执行 Agent 必须遵守）

1. **触发时机**：每个子任务完成且**验证通过**（测试通过 / 构建成功 / 人工确认）后，立即将对应 `- [ ]` 改为 `- [x]`；不得批量补勾。
2. **同步汇总行**：每次勾选后同步更新本节顶部“执行进度”行（Phase 数 / 子任务数 / 最后更新日期）。
3. **禁止预勾**：未验证的任务不得勾选（与个人规则第 5 条、原始文档 0.1 节一致：未执行的不得声称完成）。
4. **阻塞处理**：被阻塞的任务不勾选，在 `PROJECT_STATUS.md` 的 Blocked 节记录证据；恢复后按第 1 条补勾。
5. **一致性**：本清单与 `PROJECT_STATUS.md` 保持一致；冲突时以实际验证结果为准。
6. **阶段汇报**：每完成一个 Phase，按原始文档第 18 节格式汇报，并更新 `PROJECT_STATUS.md` 与 `CHANGELOG.md`。

---

## 1. 产品定位与 MVP 边界

### 1.1 产品定位

一个基于用户提供项目路径的 **Windows 本地项目索引与信息聚合器**，以卡片式界面统一展示项目、技术栈、Git 信息和开发历史，并提供项目级 Open / Run / Build 操作。

产品名称：`windy-project-mgr`。最终目标：**小巧、快速、清晰、可靠**，适合长期管理本地开发项目。

### 1.2 MVP 必须实现

- 项目管理：添加 / 查看 / 编辑 / 删除项目记录
- 项目信息：名称、路径、描述、标签、项目类型、技术栈
- Git 信息：是否仓库、分支、Working Tree 状态、修改文件数、最近 Commit 列表、基础开发活动
- 项目操作：Open 目录、Run、Build、在编辑器中打开（D6 新增）
- UI：Dashboard、卡片式项目列表、项目详情、Add/Edit Dialog、Search、主题切换（D1 新增）、基础状态展示

### 1.3 明确禁止

AI Agent 管理、Terminal UI、Git 图形化客户端、Docker 管理、IDE 内嵌、任务/Issue/Todo 系统、数据库管理、云同步、账号系统、后台服务器、本地 HTTP Server、Electron、内置 Git、全盘自动扫描、复杂远程开发。

---

## 2. 技术架构基线与数据模型

### 2.1 架构

```text
React + TypeScript + Vite
        ↓ Tauri IPC
Rust (Tauri 2)
  ├── Project CRUD（projects.json）
  ├── Settings（settings.json，D6）
  ├── Scanner（项目类型 / 技术栈 / 活动）
  ├── Git Scanner（调用系统 Git CLI，不实现 Git）
  └── Open / Run / Build / Open in Editor
```

禁止 `React → HTTP → Local Server → Rust` 形态；无本地 Web Server。

### 2.2 持久化模型

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

持久化中禁止加入 `branch` / `gitStatus` / `lastCommit` / `techStack` / `projectType` / `activity`（均属运行时扫描数据，D2 确认不落盘）。

### 2.3 运行时扫描模型（仅存前端内存，D2）

```ts
interface ProjectMetadata {
  projectType: string | null
  techStack: string[]
  git: GitMetadata | null
  activity: ActivityMetadata
}

interface GitMetadata {
  branch: string
  status: "clean" | "modified" | "unknown"
  changedFiles: number
  ahead: number
  behind: number
  lastCommit: GitCommit | null
  recentCommits: GitCommit[]
}

interface GitCommit {
  hash: string
  message: string
  author: string
  date: string
}

interface ActivityMetadata {
  lastModifiedAt: string | null
  lastScannedAt: string   // 本次运行内的扫描时刻
}
```

### 2.4 存储

- `<EXE目录>\data\projects.json`：`{ "version": 1, "projects": [] }`
- `<EXE目录>\data\settings.json`（D6 / D14）：v2 结构为 `{ "version": 2, "colorMode": "system", "accentColor": { "kind": "preset", "value": "windy-teal" }, "editor": { "executable": "", "arguments": ["{path}"] } }`；读取 v1 `{ "editorCommand", "theme" }` 时自动迁移
- 共同要求：Schema Version、写临时文件再替换（写失败不破坏原文件）、损坏可诊断、删除项目记录不删除实际目录。
- 不读取、不迁移、不删除旧 `%APPDATA%\windy-project-mgr`；所有运行形态统一使用 EXE 相邻 `data\`。开发数据位于 `target\debug\data\`，`cargo clean` 会将其删除。

### 2.5 Tauri Command 全集

```text
get_projects / get_project / create_project / update_project / delete_project
scan_project
list_scripts
open_project / run_project / build_project
get_settings / update_settings / open_in_editor   ← D6 增量
detect_editors / get_windows_accent_color / get_app_info ← Settings v2 增量
```

---

## 3. 环境前置

### 3.1 实测环境（2026-08-25 核查）

| 项目 | 状态 |
|---|---|
| Node | v24.18.0 已就绪 |
| pnpm | 10.26.2 已就绪（选定包管理器，D11） |
| Git | 2.48.1.windows.1 已就绪 |
| MSVC C++ 构建工具 | Visual Studio Community 2022（含 VC x86/x64）已就绪 |
| WebView2 | Windows 11 内置，无需安装 |
| **Rust / Cargo** | **未安装，Phase 0 第一项任务** |

### 3.2 Rust 安装步骤

1. `winget install Rustlang.Rustup`（或下载 rustup-init.exe 执行）
2. 默认选项：stable 工具链 + `x86_64-pc-windows-msvc` target
3. 验证：`rustc --version`、`cargo --version`
4. 结果记入 `PROJECT_STATUS.md` 环境审计表

---

## 4. 已确认决策清单（D1~D14）

### D1 主题系统

- 全局单样式表 + CSS 变量（颜色/间距/圆角/状态色全走变量），零样式依赖
- 亮 / 暗两套完整变量集，根节点 `data-theme` 属性切换；亮 / 暗 / 跟随系统 三选项
- 默认跟随系统；颜色模式与强调色独立持久化到 v2 `settings.json` 的 `colorMode` / `accentColor`
- 强调色支持六个预设、Windows 当前主题色与合法的 `#RRGGBB` 自定义值，并派生 accent / hover / soft / focus / on-accent 变量
- Windows 当前主题色只在启动和设置弹窗打开时读取；读取失败时由前端回退 Windy teal 并提示
- 扩展点：新增强调色预设不改变 `data-theme` 切换机制

### D2 扫描策略

- 启动即渲染卡片骨架，并发异步 `scan_project` 全部项目，逐卡填充
- 扫描结果仅存前端内存，不落盘；不做 TTL、不做持久化缓存
- Detail 页复用内存缓存；手动刷新触发单项目重扫

### D3 项目查重

- 路径规范化（绝对路径）后**大小写不敏感**比较
- 命中重复即拒绝创建并提示，引导编辑已有记录；不做合并
- 测试覆盖：重复路径、大小写变体、末尾分隔符变体

### D4 Run / Build 执行语义

- detached 启动：优先 `wt.exe`，探测失败回退 `powershell -NoExit`
- `cwd = project.path`；只报告启动成功 / 失败，不采集命令退出码与输出
- 验收范围：启动动作失败（无效路径 / 终端拉起失败）；命令为空时按钮禁用 + `Run command not configured` / `Build command not configured`

### D5 启动脚本识别引导

- Add Dialog Step 2 扫描项目根目录**全部** `*.bat` / `*.cmd` / `*.ps1`（不递归），列表供选
- 排序预选：含 `start` 优先 > 含 `run` > 字母序；默认预选第一项；可改选 / 清空 / 跳过
- 自选脚本 = 文件选择器选文件，路径写入 `runCommand`；不复制、不解析
- 仅作用于 `runCommand`；`buildCommand` 手动输入；执行走同一套 detached 终端机制

### D6 在编辑器中打开

- `settings.json` 使用 v2 `editor` 配置：真实可执行文件路径与逐行 `arguments`；v1 `editorCommand` / `theme` 自动迁移
- 设置 Dialog：Editor 分页调用真实发现，支持 VS Code、VS Code Insiders、Cursor、Windsurf、VSCodium、Zed 的实际路径，以及 Other `.exe` 文件选择
- 参数必须恰好包含一个 `{path}`；`.exe` 直接启动，`.cmd` / `.bat` 通过 `cmd.exe /d /c` 启动，工作目录为项目目录
- 入口：卡片 More 菜单 + Detail Actions；未配置显示 `Editor not configured` 并引导配置
- 新增 Command：`get_settings` / `update_settings` / `open_in_editor` / `detect_editors` / `get_windows_accent_color` / `get_app_info`

### D7 Search

- `name` / `description` / `tags` / `path` 四字段，大小写不敏感子串匹配
- 纯前端即时过滤，无防抖、无索引；空结果显示 Empty State

### D8 Git Scanner 边界

- 绝不 `git fetch`；ahead/behind 基于本地 `@{u}` 计算，无上游时为 `0` 且 UI 不显示
- `recentCommits` 取最近 10 条；空仓库 `lastCommit = null`、`recentCommits = []`
- detached HEAD：`branch` 返回 `detached@<短hash>`；`status` 三态 `clean` / `modified` / `unknown`
- 未来扩展点（不实现）：VSCode 式 git-history 图形化视图

### D9 Sidebar

- 所有项目 + 标签过滤（从现有项目数据自动提取，不单独维护）+ 底部设置入口

### D10 Add / Edit Dialog 与删除

- Add Dialog 两步：Step 1 基础信息（路径选定后 `name` 自动取末段填充、可编辑；描述、标签；路径校验 + 查重），Step 2 启动配置引导（可跳过）
- Edit Dialog 单步，可编辑全部字段（含 `runCommand` / `buildCommand`）
- 删除必须确认 Dialog；仅删记录，不删目录

### D11 包管理与工程链

- pnpm + `pnpm-lock.yaml`；模板 create-tauri-app（React + TypeScript + Vite + Tauri 2）
- 状态管理仅用 React 内置 hooks；不引入任何样式框架与状态库

### D12 测试边界

- Rust：`cargo test` 全量覆盖，临时目录 + 真实临时 git 仓库，不只依赖 Mock
- 前端：仅纯逻辑用 `vitest`（搜索过滤、路径查重规范化、卡片数据组装）；不写组件渲染测试
- UI 全链路：人工验收清单（`TESTING.md` 逐项勾选），不搭 WebDriver

### D13 Phase 1 文档交付物

- `GLOSSARY.md`（术语表）、`docs/adr/0001-scan-data-memory-only.md`、`docs/adr/0002-detached-run-build.md`、`docs/adr/0003-css-variable-theming.md`、`TESTING.md` 验收清单骨架

### D14 绿色 ZIP 与完全便携数据

- 所有运行形态把 `projects.json` / `settings.json` 写入当前 EXE 同目录 `data\`
- 不设便携标记、不回退 AppData、不自动迁移旧数据
- `build.bat` → `build.ps1` → `pnpm tauri build --no-bundle`，生成绿色目录与版本化 ZIP，不生成 MSI/NSIS
- 开发数据位于 `target\debug\data\`，执行 `cargo clean` 会删除开发数据

---

## 5. 执行顺序（Phase 0~14 + 增量调整）

| Phase | 内容 | 增量调整（冲突时以本列为准） |
|---|---|---|
| 0 环境审计 | 检查 OS/Node/pnpm/Rust/Cargo/Tauri/Git/仓库状态，输出环境表，创建 `PROJECT_STATUS.md` | rustup 安装与验证（见 3.2） |
| 1 项目规划 | `README.md` / `GLOSSARY.md` / `CHANGELOG.md` | 追加基础 ADR、`TESTING.md` 骨架（D13）；Settings v2 规格 / 计划 / ADR 0005 |
| 2 工程初始化 | create-tauri-app 骨架，验证 dev / production build | pnpm（D11）；`pnpm tauri dev` |
| 3 基础目录 | `src/`（components/pages/lib/types）与 `src-tauri/src/`（commands/project/scanner/git） | 不变；结构服务于代码，可裁剪 |
| 4 数据层 | Project Store + `projects.json`：Load/Save/Create/Update/Delete/Get；先写测试再实现 | 增加 `settings.json` 读写（D6），同机制 |
| 5 Project CRUD | CRUD + 单元/集成测试 | 查重测试用例（D3）；删除确认留待 Phase 8 |
| 6 Project Scanner | 项目类型 / 技术栈 / 活动检测，真实目录验证 | 增加根目录脚本枚举（D5） |
| 7 Git Scanner | 调用系统 Git CLI；真实仓库验证 | 按 D8 边界；测试含无上游/空仓库/detached HEAD |
| 8 Dashboard UI | Sidebar + Toolbar + Search + Card Grid | 骨架先行 + 并发扫描填充（D2）；Sidebar（D9）；Search（D7）；卡片 More 含编辑器入口（D6）；两步 Add Dialog（D10）；删除确认 |
| 9 Project Detail | Header/Overview/Tech/Git/Commits/Activity/Actions | Actions 增加编辑器入口（D6）；手动刷新（D2） |
| 10 Run / Build / Open | `open_project` / `run_project` / `build_project` | 按 D4 语义；新增 `get_settings` / `update_settings` / `open_in_editor`（D6） |
| 11 UI Polish | Spacing/Typography/Hover/Loading/Error/Empty State | 主题系统（D1）：双变量集、`data-theme`、六种预设 / Windows / 自定义强调色、持久化；Settings v2 三分页设置中心 |
| 12 全链路集成测试 | 真实用户流程 + 异常路径（Git 缺失 / 路径不存在 / JSON 损坏 / Run 失败） | 人工验收清单覆盖：脚本引导、编辑器打开、主题持久化、查重拒绝 |
| 13 Production Build | 绿色 ZIP；实测 EXE 与 ZIP 体积 | `build.bat` 无安装器构建；解压启动 + EXE 相邻 `data\`（D14） |
| 14 最终测试 | Functional / Error Handling / UX / Stability / Production | 不变 |

开发节奏（原第 27 节）：Inspect → Plan → Implement → Test → Verify → Update Status → Next Phase；每个 Vertical Slice 独立提交（原第 19 节）；`PROJECT_STATUS.md` 持续维护（原第 17 节）。

---

## 6. 测试策略（D12）

| 层 | 方式 | 范围 |
|---|---|---|
| Rust 单元 + 集成 | `cargo test` 自动化 | Store / CRUD / Scanner / Git Scanner；临时目录 + 真实临时 git 仓库；覆盖文件不存在 / 空文件 / JSON 损坏 / 保存失败 / 重复 ID / 删除不存在项目 / 大小写与分隔符查重变体 |
| 前端纯逻辑与静态契约 | `vitest` 自动化 | 搜索过滤、路径查重规范化、卡片数据组装、主题 / 设置校验、设置界面关键契约 |
| UI 全链路 | `TESTING.md` 人工验收清单 | Phase 12 / 14 逐项勾选；不搭 WebDriver |

测试失败处理：现象 → 证据 → 原因 → 修复 → 验证；不得将猜测称为 Root Cause。

---

## 7. 验收标准（原始文档第 25 节 + 增量）

### 基础验收（原始第 25 节）

- 工程：Tauri application builds successfully
- 项目：Create / Read / Update / Delete
- 信息：Project Type / Tech Stack / Git Branch / Git Status / Recent Commits / Activity
- 操作：Open / Run / Build
- UI：Dashboard / Card Grid / Search / Detail / Add/Edit
- 稳定性：Persistence / Error Handling / Restart / Production Build
- 体积：实测绿色版可执行文件与 ZIP 大小，不得用估算值

### 增量验收（2026-08-27 更新）

1. **主题**：亮 / 暗 / 跟随系统三种行为正确；用户手动选择重启后保留
2. **查重**：重复添加同一路径（含大小写变体、末尾分隔符变体）被拒绝并有明确提示
3. **脚本引导**：含启动脚本的目录，Add Dialog Step 2 正确列出候选并按 `start` > `run` > 字母序规则预选
4. **编辑器入口**：Settings v2 的 Editor Profile 已配置时，卡片 More 菜单与 Detail 均可按 `{path}` 打开项目目录；未配置时显示 `Editor not configured` 并引导配置；Other 仅保证程序启动，不保证程序自身支持文件夹工作区
5. **离线可用**：无网络环境下所有功能可用（无任何 `git fetch` / 网络依赖）

---

## 8. 文档交付物索引

| 文档 | 位置 | 落盘时机 |
|---|---|---|
| 原始需求与计划 | 根目录《Windy Project Manager - Primary Request&Plan Document.md》（含第 29~31 节决策扩展） | 已完成 |
| 本执行计划 | `docs/PLAN.MD` | 已完成 |
| 术语表 | `GLOSSARY.md` | Phase 1 |
| ADR：扫描数据仅内存 | `docs/adr/0001-scan-data-memory-only.md` | Phase 1 |
| ADR：Run/Build 分离启动 | `docs/adr/0002-detached-run-build.md` | Phase 1 |
| ADR：CSS 变量主题 | `docs/adr/0003-css-variable-theming.md` | Phase 1 |
| ADR：绿色 ZIP 与便携数据 | `docs/adr/0004-portable-data-and-zip.md` | 2026-08-26 |
| 人工验收清单 | `TESTING.md` | Phase 1 |
| 开发状态 | `PROJECT_STATUS.md` | Phase 0 起持续维护 |
| 变更记录 | `CHANGELOG.md` | Phase 1 起持续维护 |
