# Changelog

本项目所有实际完成的变更均记录于此文件（Phase 1 建立，持续维护），按完成日期与 Phase 倒序组织。

MVP 完成前无正式版本号，变更记于 `Unreleased`。

## Unreleased

### 2026-08-30 — 结构级 UI 焕新（网易云语法）+ 主题工坊（settings v3）

- **设计令牌重构**：新中性色板（亮 `#f5f6f8` 系 / 暗 `#121417` 系）、三档阴影（rest / hover / overlay）、动效令牌（120 / 180 / 280ms + `--ease-out` / `--ease-spring`）；侧栏落于 `--bg` 与内容区分层，卡片 / 面板落于 `--surface`；导航选中态改为实心 accent 胶囊 + `on-accent` 文字；全局 `--r-lg` 16px 对话框 + 背景 blur（reduced-motion 关闭）；toast 改为底部居中深色胶囊。
- **封面式卡片（签名元素）**：`ProjectCard` 新增按项目类型着色的渐变封面条（`card-cover cover-<type>`，68px，大号类型字形），Detail 页新增 72px `detail-cover` 瓷贴呼应；详情页 hero 化（封面瓷贴 + 标题 + 操作行）。
- **主题工坊（settings v3）**：`appearance` 域新增风格预设包（windy / cloud / ink / midnight，完整 token 组合含推荐强调色与中性色）、圆角 0–20px、字号 13–16px、密度 compact / comfortable、正文字体（8 个预设栈下拉 + 自定义栈输入 + `sanitizeFontFamily` 清洗）、中性色物化保存；「Save as custom theme」解除预设归属，「Reset appearance」回 Windy 默认；设置实时预览捕获 / 还原扩展为 accent 5 项 + appearance 10 项。前端纯逻辑 `src/lib/appearance.ts`（预设 / 校验 / 变量派生，14 个 vitest）；Rust `project/settings.rs` 版本化 v1→v3、v2→v3 迁移 + `validate` + 5 个新测试（未新增 Tauri 命令）。
- **自绘滚动条**：全部滚动容器统一细胶囊样式（`scrollbar-width: thin` + `::-webkit-scrollbar*`），`--sb-thumb` 由 `--muted` 经 `color-mix` 派生，亮 / 暗与 settings v3 中性色覆盖下自动适配（用户新增要求）。
- **验收中发现并修复的三个缺陷**：① `.card-grid` 自动行在获得确定高度的容器下被均分压缩（卡片描述 / 技术栈 / Git 行 / 操作区被 `overflow:hidden` 裁掉）→ `grid-auto-rows: max-content`；② `.card-actions` 悬停隐藏但保留布局空间，卡片底部出现大片死白且藏起主操作 → 改为常显（交互流程与 Phase 15 基线一致）；③ `.card-body` 未伸展导致同行矮卡操作行不贴底 → `flex: 1 1 auto`（同行操作行底对齐）。
- **验证**：`pnpm test` 8 个文件 / 87 个测试通过、`cargo test` 155 / 155 通过（含 v3 迁移）、`pnpm build` 通过；静态预览（Tauri IPC mock 注入）在 320 / 768 / 1024 / 1440 亮暗两主题零水平溢出，Dashboard / Detail / Settings 三分页 / 暗色对比 / CJK 长名称换行 / 无 Git / detached / unknown 降级态复核通过；自绘滚动条亮暗双验。沿用 Phase 15 口径：几何验收基于静态 Vite 预览，未跑真实 Tauri 拖拽缩放。

### 2026-08-28 — Git 扫描控制台窗口回归修复

- **根因**：添加项目后，`scan_project` 对 Git 仓库串行调用系统 Git CLI；Git 模块的 `Command::output()` 未设置 Windows `CREATE_NO_WINDOW`，因此绿色 EXE 会在扫描阶段闪现控制台窗口。
- **修复**：`src-tauri/src/git/mod.rs` 为每次 Git 子进程设置 `CREATE_NO_WINDOW`；扫描字段、离线边界和现有命令面均未改变。
- **验证**：新增 Windows 子进程隐藏契约测试并完成红→绿；`cargo test` 150/150、`pnpm test` 7 个文件 / 69 个测试、`pnpm build` 与 `build.bat` 均通过。真实绿色 EXE 走通 `D:\Dev\opia-rss-reader` Add Project 与 Detail Refresh；Add 阶段所有观测 Git 子进程均为 `hwnd=0`，Refresh 的 3 秒 UIA 观察无新增顶层窗口。

### 2026-08-28 — 图标细节修复与 build 产物

- 修复 Card / Detail 动作按钮图标与文字的垂直中轴：统一 16px 图标盒、固定行高和显式上下居中。
- 将 `hammer` 自绘 SVG glyph 替换为用户提供的 1024×1024 填充 path，按 `scale(0.0234375)` 归一到 24×24，并使用 `currentColor` / `stroke="none"` 适配主题。
- 按需求不启动实机窗口；`build.bat` 通过，产物写入 `build\\win-unpacked\\windy-project-mgr.exe`。
- 静态契约回归：`pnpm test` 7 个文件 / 69 个测试通过。

### 2026-08-27 — Phase 15 自绘 SVG 图标系统与全页面 UI 审计

- **图标系统**：新增零依赖 `src/components/Icon.tsx`，集中维护 24×24、`currentColor`、圆端/圆角线条的自绘 SVG glyph；装饰图标默认隐藏于辅助技术，icon-only 控件提供明确的 `aria-label` 与 tooltip。
- **全页面接入**：Dashboard、Sidebar、ProjectCard、Detail、Add/Edit/Confirm、Settings 三分页、More 菜单、Toast、空态、加载态和错误态均接入语义图标；Git clean / modified / unknown 同时使用形状与颜色表达。
- **UI 审计**：补齐图标尺寸 / 间距 / 命中区 / 焦点 / flex 收缩 / 长文本换行 / 状态色规则，保留 reduced-motion 行为；静态 Vite 页面在 320 / 768 / 1024 / 1440 下无水平溢出或图标重叠，真实 Tauri 默认窗口完成主要交互路径复核。
- **验证**：新增 `src/lib/iconUi.test.ts`；`pnpm test` 7 个文件 / 68 个测试通过，`pnpm build` 与 `git diff --check` 通过。未修改 Rust、IPC、持久化或命令契约。

### 2026-08-27 — 构建版系统查询窗口稳定性修复

- **问题证据**：真实绿色构建版快速打开 Settings 时，进程采样确认反复启动 `reg.exe query HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Accent`；用户同时观察到短暂系统窗口、卡顿及 `reg.exe` `0xc0000142` 错误对话框。
- **修复**：Windows 下系统发现与 Windows accent 查询共用的 `Command::output()` 现在设置 `CREATE_NO_WINDOW`，不再为控制台子进程创建可见窗口；新增 Windows 编译回归断言。
- **回归**：修复后重建绿色目录与 ZIP，真实窗口中 Settings 与 Add Project 各连续开关 5 次均成功，无错误窗口，结束后无残留 `reg.exe`；`cargo test` 149/149、Vitest 64/64、`pnpm build` 均通过。

### 2026-08-27 — Phase 14 最终验收完成与计划文档清理

- **最终验收**：用户确认 Functional、Error Handling、UX、Stability、Production 及五项增量验收全部通过，进度更新为 15 / 15 Phase、34 / 34 子任务，MVP 交付闭环。
- **文档同步**：同步 README、AGENTS、PROJECT_STATUS、HANDOFF、TESTING 与 `docs/PLAN.MD` 的最终状态；仅删除本次 Settings v2 的设计规格和实施计划，保留执行证据、源码测试与正式交付文档。

### 2026-08-27 — Settings v2 验收收尾与 Other 编辑器路径修复

- **设置中心 v2**：完成 Appearance / Editor / General 三分页、颜色模式与强调色分离、六个预设色、Windows 当前主题色、自定义 `#RRGGBB`、真实编辑器发现、Other `.exe` 选择、逐行参数与 `{path}` 文件夹工作区约束。
- **验收中修复的 bug**：修复含空格路径的 Run / Build PowerShell 启动参数边界；修复 `.cmd` / `.bat` 编辑器启动与参数边界；修复 Windows accent 异步响应在关闭后覆盖预览；补齐 pending close 防护、Reset alertdialog 焦点/描述和旧 v1 CSS 清理；修复 Other 选择未扫描到的可执行文件后列表不显示所选路径的问题。
- **真实窗口验收**：Light / Dark / Follow system 即时切换与重启持久化、六种预设 / Windows / Custom、Cancel 恢复、General 信息、编辑器发现、VS Code 与 Qoder 文件夹工作区、Other 选择与保存/重开、`{path}` 校验及未配置提示均通过。
- **边界说明**：任意 Other 程序只保证启动，不保证支持文件夹工作区；Notepad 启动后由自身提示无法打开文件夹，VS Code / Qoder 的实测证明工作区能力。物理断网、真实多宽度调整和破坏性 Reset now 本轮未执行。
- **验证**：`pnpm vitest run` 64/64、`cargo test --manifest-path src-tauri/Cargo.toml` 148/148、`pnpm build`、`git diff --check` 通过。证据：`.superpowers/sdd/2026-08-27-settings-overhaul/task-7-report.md` 与 `task-8-bugfix-report.md`。

### 2026-08-27 — Task 5 / Stage 6 strict re-review

- Re-review result: `CHANGES REQUIRED`. Build, Vitest and diff checks pass, but a pending Windows accent request can call the parent preview callback after Settings unmounts; the inline reset `alertdialog` lacks complete focus/description semantics; and residual v1 Settings CSS remains. Evidence: `.superpowers/sdd/2026-08-27-settings-overhaul/task-5-rereview.md`.

### 2026-08-27 — Task 5 UI review 修复

- 修复 Settings 的 Windows accent 父子状态同步、旧异步响应覆盖草稿、Save/Reset pending 关闭竞态与 system `matchMedia` 亮暗切换；`applyTheme` 支持显式系统偏好。
- Modal 增加 `aria-modal` 对应的初始焦点、Tab 陷阱、关闭后焦点回收与 `closeDisabled`；补齐 tabpanel、accent radio、Editor listitem 语义及 320px alert 断词收缩。
- 新增 `src/lib/settingsUi.ts` 与 9 个纯逻辑/静态契约回归测试；移除确认无引用的 v1 settings CSS 块。验证：`pnpm vitest run` 59/59、`pnpm build`、`git diff --check` 通过。

### 2026-08-27 — Settings v2 Task 5 三分页设置界面

- **SettingsDialog**：新增 Appearance / Editor / General 三分页，使用浏览器 tab 语义与 Arrow/Home/End 键盘导航；Appearance 即时预览 color mode、六个 `ACCENT_PRESETS`、Windows 色和自定义 `#RRGGBB`，Cancel/Esc/遮罩/关闭恢复已保存主题状态
- **Editor**：仅进入页面时调用 `detect_editors`，展示 DTO 返回的真实名称、路径和 Path/Registry/Standard 来源；Other 通过 Tauri dialog 选择 `.exe`，参数逐行编辑并复用 `validateEditorProfile` 校验 `{path}` 与 batch 引号约束
- **General / App**：进入页面调用 `get_app_info`；Reset settings 二次确认后只调用 `update_settings`，App 启动读取 v2 settings 与 Windows accent，读取失败保留可见错误并回退 Windy teal
- **样式与验证**：保留 Windy teal / paper 基底，增加宽版设置布局和 320/768/1024/1440 适配规则；分页指示器仅使用 180ms cubic-bezier transform/opacity，并支持 reduced motion。`pnpm vitest run` 50/50、`pnpm build`、`git diff --check` 通过

### 2026-08-27 — Task 3 launcher fix3 最终修复

- **PowerShell**：run/build 保留普通命令原文；盘符/UNC/已知扩展名路径按可执行路径与参数分离包装；有 cwd 且文件真实存在的 extensionless 路径在 `ps_plan` / `wt_plan` 中正确分离；无法无歧义识别的裸 extensionless 路径保持原文
- **Batch 安全契约**：`.cmd` / `.bat` 完全大小写不敏感地直接走 `cmd.exe /d /s /c`；脚本路径和每个参数只经子进程专属环境变量传入，`/c` 使用 raw argument；字面双引号在 settings 校验和 launcher 兜底均拒绝，错误文案为 `cmd.exe batch arguments cannot contain the double quote character`；直接 `.exe` 不受该限制
- **测试**：真实 Windows batch 夹具覆盖空格、字面 `%PATH%`、`&`、`!` 及组合值，并覆盖 batch 双引号拒绝、混合扩展名、路径+参数、路径-only 与错误语义
- **验证**：`cargo test --manifest-path src-tauri/Cargo.toml --test launch --test actions_commands`（42/42）；`cargo test --manifest-path src-tauri/Cargo.toml`（146/146）；`git diff --check` 通过

### 2026-08-26 — Phase 12 全链路集成测试验收

- **人工验收**：`TESTING.md` B 节 6 项全部通过（非 Git / 不存在路径 / 损坏 JSON / Run 与 Build 启动失败 / 空仓库 / detached HEAD）；C 节主题三态与重启持久化、编辑器卡片与详情入口、未配置提示、离线边界全部通过；既有查重与脚本引导保持通过
- **验收中修复**：主题在 Settings 中切换后即时预览，并在取消时恢复已保存主题；Add 路径 Browse 按钮记忆位置；查重提前到 Add 第一步；详情卡片等高；Created 使用本地时间；Modal 头尾分隔线内缩、删除确认框窄版；菜单 Delete 红字；Run / Build 对含空格路径使用 PowerShell `& '...'` 包裹
- **异常证据**：损坏文件保留为原始 `###` 未被覆盖；无效路径不影响其它卡片，Run / Build 仅显示 `path not found` Toast；空仓库显示 `No commits yet`；detached 仓库显示 `detached@<短hash>`；离线检查确认 Git 仅调用本地 CLI 且无 `git fetch` / 网络连接
- **验证**：`pnpm vitest run` 37/37、`cargo test` 101/101、`pnpm build` 通过；测试结束后恢复开发数据为原始 `node-app-e` 与 `plain-dir` 两条记录，settings 恢复 `theme=system` / `editorCommand=""`

### 2026-08-26 — Phase 13 绿色 ZIP 与完全便携数据（D14）

- **便携数据**：`app_data_dir()` 不再读取 `%APPDATA%`，统一通过 `current_exe()` 将 `projects.json` / `settings.json` 保存到当前 EXE 同目录 `data\`；不设标记、不回退、不自动迁移或删除旧数据。新增 4 个集成测试覆盖普通路径、空格路径、无父目录错误与真实设置落盘
- **绿色构建**：新增 ASCII-only `build.bat` 与 PowerShell 7 `build.ps1`；执行 `pnpm tauri build --no-bundle`，从 `tauri.conf.json` / `cargo metadata` 解析精确产物名，安全清理并生成 `build\win-unpacked\`、版本化 release 目录和 ZIP，不生成 MSI/NSIS
- **实测**：release 优化编译 1m42s；EXE 9.27 MiB，ZIP 2.86 MiB。ZIP 精确包含版本目录、空 `data\` 与单个 EXE；临时解压后的 EXE 启动且窗口正常响应，便携设置组合测试确认 `data\settings.json` 落点
- **验证**：`cargo test` 101/101、`pnpm vitest run` 36/36、`pnpm build` 通过；PowerShell AST、batch ASCII、严格 UTF-8、ZIP 内容与 Git 差异检查均纳入最终验证
- **文档**：新增 ADR 0004，更新 AGENTS / PLAN / Primary Document / README / TESTING / PROJECT_STATUS；Phase 13 勾选完成，进度更新为 13/15 Phase、30/34 子任务

### 2026-08-26 — Cargo dev/test 构建缓存治理

- **配置**：`src-tauri/Cargo.toml` 为 dev/test profile 设置 `debug = "line-tables-only"`，保留源文件与行号信息，限制 Windows PDB 与 `target/` 增长；release profile 未改
- **冷启动实测**：完整 `cargo clean` 后 `start.bat` 正常启动，Vite 283ms ready，Rust 374 构建单元 56.84s，Tauri 窗口正常响应；`target` 2.169 GiB，对比完整调试信息下同条件基线 4.529 GiB，减少 52.1%
- **测试与体积**：`cargo test` 97/97 通过（10.09s）；测试后 `target` 2.448 GiB，对比治理前历史状态 9.99 GiB 减少 75.5%；`debug/deps` PDB 由 1.964 GiB 降至 0.240 GiB

### 2026-08-25 — Phase 11 UI Polish + 主题系统

- **主题系统（D1 / ADR 0003）**：`App.css` 将暗色变量集拆为 `:root[data-theme="dark"]`（强制暗，覆盖系统偏好）与 `@media (prefers-color-scheme: dark){ :root:not([data-theme="light"]) }`（跟随系统，强制亮的 `data-theme="light"` 被排除）；三态 `system` / `light` / `dark` 齐全。新增 `lib/theme.ts`（`Theme` 类型 / `isValidTheme` / `applyTheme` / `THEMES` / `EDITOR_PRESETS`，纯逻辑 TDD 红→绿 4 个 vitest）
- **设置 Dialog（D6）**：`SettingsDialog` 由只读占位改为可编辑表面 — 主题单选组（跟随系统 / 亮 / 暗，含固定预览色样）+ 编辑器命令 `datalist` 预设（`code` / `code-insiders` / `cursor`）+ 自由输入 + 「Not configured / Running」提示；保存经 `update_settings` 持久化（编辑清空 → 保留 `Editor not configured` 语义，后端未改）
- **接线**：`lib/api.ts` 新增 `getSettings` / `updateSettings` + `AppSettings` 类型；`App.tsx` 启动 `getSettings()` 并按持久化 `theme` 应用 `data-theme`（重启保留），`onSaved` 更新状态实时切换
- **运行时验证**：临时前端探针在内置 WebView2 内实测 D1 全链路 — 预置 `theme=dark` 启动后 `data-theme=dark` 生效、`--bg` 解析为暗色 `#14181a`；经真实 IPC `update_settings` 保存后 `get_settings` 往返一致（持久化验证）；探针代码验收后已移除
- **验证**：`pnpm vitest run` 29/29（原 25 + theme 4）、`pnpm build`（tsc 全量）通过、`cargo test` 94/94 无回归；零新依赖（纯 CSS 变量 + React 内置 hooks，无新 npm 包）；双轴评审修正：移除未用的 `data-swatch` 属性、为 CSS 两处暗色变量块加同步注释

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
