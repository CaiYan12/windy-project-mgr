# Windy Project Manager Development Status

Current Phase:
Phase 15 — 自绘 SVG 图标系统与全页面 UI 审计已完成；2026-08-28 图标细节与 Git 扫描窗口回归修复构建完成

Status:
PASS — Phase 0~15 全部完成；Settings v2、绿色 ZIP、图标系统、图标细节与 Git 扫描窗口回归修复均已验证

Completed:
- 2026-08-28 Git 扫描窗口回归修复：定位 `scan_project` 在 Git 仓库中串行调用 6 次系统 Git CLI 时未设置 `CREATE_NO_WINDOW`，导致绿色 EXE 添加项目后闪现控制台窗口；新增 Windows 子进程契约测试并完成红→绿。真实绿色 EXE 通过 `D:\Dev\opia-rss-reader` Add Project 与 Detail Refresh，Add 后卡片在 266.9ms 出现且所有观测 Git 子进程均为 `hwnd=0`，Refresh 返回 40.0ms 且 3 秒 UIA 监控无新增顶层窗口；验证数据已清理。
- 2026-08-28 图标细节修复：Card / Detail 动作按钮统一 16px 图标盒、固定行高与上下居中；将 hammer 替换为用户指定的 1024×1024 填充 path，按 `scale(0.0234375)` 归一到 24×24，并使用 `currentColor` / `stroke="none"`。按用户要求未启动实机；`build.bat` 与随后静态 `pnpm test`（7 个文件 / 69 个测试）均通过；产物为 `build\\win-unpacked\\windy-project-mgr.exe`（9,881,088 bytes）。
- 2026-08-27 Phase 15 图标系统与 UI 审计：新增零依赖自绘 `src/components/Icon.tsx` 与 `src/lib/iconUi.test.ts`，接入全页面语义图标；完成主题 / 强调色、Git 状态形状 + 颜色、焦点命中区、长文本收缩和 reduced-motion 契约复核。静态 Vite 页面在 320 / 768 / 1024 / 1440 下无水平溢出；真实 Tauri 默认窗口完成 Dashboard、More、Add/Edit/Confirm、Detail、Settings、Toast/错误路径复核。响应式数值尺寸未冒充原生 Tauri 拖拽缩放结果。
- 2026-08-27 Settings v2 Stage 8/9：真实 Tauri 窗口完成主题模式与强调色、General、编辑器发现、VS Code/Qoder 文件夹工作区、Other 选择、参数校验和未配置提示验收；修复未扫描到的 Other 可执行文件不显示所选路径的问题。证据见 .superpowers/sdd/2026-08-27-settings-overhaul/task-7-report.md 与 task-8-bugfix-report.md
- 2026-08-27 构建版稳定性回归修复：定位 Settings 打开时系统查询子进程未隐藏导致的 `reg.exe` 瞬时窗口与卡顿；为 Windows 系统查询统一设置 `CREATE_NO_WINDOW`，并在新绿色构建版中完成 Settings / Add Project 各 5 次快速开关回归，未再出现错误窗口或残留 `reg.exe`。
- 最终自动化验证：本轮 `pnpm test` 7 个文件 / 68 个测试通过，`pnpm build` 与 `git diff --check` 通过；Cargo 149/149 的后端基线保持通过。
- 用户确认最终人工测试与验收全部通过，Phase 14 三项收尾复选框已完成；随后构建版系统查询窗口稳定性缺陷已修复并回归通过。
- Phase 0：rustup / stable 工具链安装与验证，全环境审计实测并记入下表
- Phase 1：`CONTEXT.md` 术语表、`CHANGELOG.md`、`TESTING.md` 验收清单骨架、`docs/adr/0001~0003` 三份 ADR 落盘（均验证：存在 + 严格 UTF-8 无乱码）
- Phase 2：create-tauri-app 4.6.2 脚手架（react-ts 模板，D11：pnpm + React 19.2.8 + TS 5.8.3 + Vite 7.3.6 + Tauri 2.11.5，identifier `com.windy.project-mgr`）；`pnpm tauri dev` 启动验证通过（Vite 287ms ready，首次 470 crates 编译 1m57s，应用进程拉起无报错）；`pnpm tauri build` 通过（release 编译 2m21s，产出 MSI + NSIS 双 bundle）
- Phase 3：目录结构建立并可编译：`src/{components,pages,lib,types}`（.gitkeep 占位）与 `src-tauri/src/{commands,project,scanner,git}`（各含职责声明的 mod.rs，已接入 lib.rs）；`cargo check` 与 `pnpm build` 双侧验证通过
- Phase 4：数据层 TDD 完成（四个 red→green 切片）：`project::types`（Project / StoreError / Versioned）、`project::store`（Load/Save/Create/Update/Delete/Get + write_atomic + new_id/now_utc，仅 std）、`project::settings`（D6 同机制）；集成测试用真实临时目录，覆盖文件不存在 / 空文件 / JSON 损坏 / 版本不匹配 / 保存失败不损原文件 / 重复 ID / 删除不存在项目 / 删除不删目录
- Phase 5：`project::dedup`（normalize_path / is_same_path / find_duplicate_by_path，D3 三变体 + 正斜杠变体）；`commands::project`（5 个 CRUD Tauri command 已注册入 invoke_handler，核心 `*_in` 函数以 data_dir 参数化可测；初始 AppData 路径已于 D14 替换为 EXE 相邻 `data\`）；查重拒绝覆盖 create 与 update（自排除）
- Phase 6：`scanner` 模块实现：detect_project_type（Node/Python/Rust/Java/CSharp/Unknown；`CSharp` 于 Phase 9 统一为 `C#`）、detect_tech_stack（11 条特征规则）、detect_activity（根目录最新 mtime，单项失败不阻断）、list_startup_scripts（D5：start > run > 字母序，不递归）；路径不存在 / 无权限可诊断错误；真实目录验证：本仓库根目录（Node + pnpm/TS/Vite）与 src-tauri（Rust）实测通过；Git/Non-Git 目录的 Git 元数据扫描属 Phase 7
- Phase 7：`git` 模块实现（TDD，七个垂直切片）：`scan_git(path) -> Result<Option<GitMetadata>, GitError>`（非仓库 Ok(None)；路径不存在 / git 不可用可诊断报错）+ `scan_git_with` 可测核心；调用系统 Git CLI（std::process::Command，零新依赖，绝不 `git fetch`）；D8 全部边界：分支（含 `detached@<短hash>`）、三态 status（porcelain 失败 → Unknown，changed_files 计入 untracked）、ahead/behind 基于本地 `@{u}`（无上游 = 0）、recentCommits 最近 10 条（%x1f 分隔解析）、空仓库 lastCommit = None；新增集成测试 11 个（真实临时 git 仓库：非仓库 / 路径不存在 / git 不可用 / 空仓库 / 单提交 / modified / untracked / 12 提交上限 / 无上游 / 本地上游 ahead/behind / detached HEAD）
- Phase 8：Dashboard UI 落地（模板 UI 与 `greet` command 移除）。Rust：`scanner::scan_project_path` 组装 `ProjectMetadata`（单项失败降级不阻断，原始第 11 节）+ `commands/scan.rs`：`scan_project` / `list_scripts`（D5 所需最小增量，超出 §2.5 面已在代码注释与 AGENTS.md 说明）；新增集成测试 4 个。前端：`types/project.ts`（与 §2.3 对应）、`lib/`（api / search / cards / paths，vitest 纯逻辑 22 个，不写组件渲染测试，D12）、`components/`（Modal+Confirm / Sidebar / ProjectFields / AddDialog 两步（含 D5 文件选择器自选脚本）/ EditDialog / SettingsDialog / ProjectCard+骨架）、App.tsx 编排（启动骨架 + 并发扫描逐卡填充 D2、标签过滤 D9、四字段 Search D7、删除确认 D10、More 菜单含编辑器入口 D6、Run/Build 未配置禁用 + D4 文案）；`App.css` 单一全局样式表全变量化（ADR 0003，亮/暗双套变量集，本阶段 `prefers-color-scheme` 跟随系统，手动切换 + 持久化属 Phase 11）；新依赖：vitest 4.1.11（devDep，D12）、@tauri-apps/plugin-dialog + tauri-plugin-dialog 2（D5 文件选择器）；`pnpm build`（tsc 全量类型检查）通过；UI 运行时验收属 Phase 12 人工清单
- Phase 9：Project Detail UI 落地。新增 `src/pages/ProjectDetail.tsx`：七分区（Header / Overview / Technology / Git / Recent Commits / Activity / Actions）全部呈现，降级态全部可见（`git = null` → `No Git repository`；空仓库 → `No commits yet`；`status = unknown`；`projectType = null` → `Unknown project type`；`activity.lastModifiedAt = null` → `No recent activity`；loading 骨架；error + Retry）；Actions 含 Open / Run / Build（未配置禁用 + D4 文案）/ Open in editor（D6，后端属 Phase 10，未注册时 Toast 报错为预期过渡态）/ Refresh（调 `scan_project` 单项目重扫，仅内存，D2）/ Edit / Delete（复用 `EditProjectDialog` / `ConfirmDialog`）；recentCommits 行 = `shortHash` 短码 + message + author + 相对时间。视图切换：`App.tsx` `selectedId` 状态（无路由库），卡片点击进入 Detail（`ProjectCard` `onSelect` + 内部交互 `stopPropagation`，项目名为键盘可达按钮），删除选中项返回 Dashboard。纯逻辑（TDD 红→绿）：`lib/cards.ts` `shortHash` + vitest 3 个。Rust：`detect_project_type` 对 `*.csproj` 返回 `C#`（原 `CSharp`，与技术栈标签统一；projectType 仅运行时，D2 不落盘），`tests/scanner.rs` 断言同步（红→绿）。样式：`App.css` 追加 Detail 分区样式，全走既有 CSS 变量（ADR 0003）；双轴评审修正内联 `cursor` 样式与 `git = null` 时 Commits 分区文案。零新依赖；未提前实现 Phase 10 后端 command
- Phase 10：Run / Build / Open / Editor 后端落地。新增 `launch` 模块（D4 / ADR 0002）：`LaunchPlan` 纯数据计划 + `spawn_plan`（detached，只报启动成败）+ `run_in_terminal`（优先 `wt.exe` 回退 `powershell -NoExit`，`cwd = project.path`）+ `open_dir`（explorer）+ `open_in_editor`（未配置 → `Editor not configured`，先于路径检查）；二进制名可注入（`run_in_terminal_with`）。新增 `commands/actions.rs`：`get_settings` / `update_settings`（`*_in` 可测核心）/ `open_project` / `run_project` / `build_project` / `open_in_editor` 六个 command 注册（§2.5 全集完整）。测试（TDD 红→绿）：`tests/launch.rs` 15 个（真实进程拉起，`hostname` 替身）+ `tests/actions_commands.rs` 4 个；cargo 75→94。运行时验证：WebView2 Runtime 151 对提权宿主禁用 CDP 环境变量（wry#1782），改用临时前端探针经真实 IPC 在内置 WebView2 内验证：10/10 PASS（含持久化往返与三类错误文案），探针已移除。零新依赖（纯 `std::process::Command`）；双轴评审修正未用派生与 `wt.exe` 字面
- Phase 11：UI Polish + 主题系统落地。主题系统（D1 / ADR 0003）：`App.css` 暗色变量集拆为 `:root[data-theme="dark"]`（强制暗）+ `@media prefers-color-scheme: dark { :root:not([data-theme="light"]) }`（跟随系统，强制亮被排除），三态 system/light/dark 齐全；新增 `lib/theme.ts`（`Theme` / `isValidTheme` / `applyTheme` / `THEMES` / `EDITOR_PRESETS`，纯逻辑 TDD 红→绿 4 个 vitest）。设置 Dialog（D6）：由只读占位改为可编辑 — 主题单选组（含固定预览色样）+ 编辑器 `datalist` 预设（code/code-insiders/cursor）+ 自由输入 + Not configured/Running 提示，保存经 `update_settings` 持久化。接线：`lib/api.ts` 新增 `getSettings`/`updateSettings`+`AppSettings`；`App.tsx` 启动读取并应用主题（重启保留），`onSaved` 实时切换。运行时验证：临时前端探针在内置 WebView2 内实测 D1 全链路 — 预置 `theme=dark` 后 `data-theme=dark` 生效、`--bg=#14181a`、真实 IPC `update_settings` 往返一致（探针已移除）。`pnpm vitest run` 29/29、`pnpm build` 通过、`cargo test` 94/94 无回归；零新依赖；双轴评审修正未用 `data-swatch` 属性与 CSS 暗色块同步注释
- 2026-08-26 构建缓存治理：`src-tauri/Cargo.toml` 的 dev/test profile 使用 `debug = "line-tables-only"`。同机完整清理后的 `start.bat` 冷启动成功（374 构建单元，56.84s），`target` 2.169 GiB（原完整调试信息基线 4.529 GiB）；随后 `cargo test` 97/97 通过（10.09s），`target` 2.448 GiB（原历史状态 9.99 GiB），其中 `debug/deps` PDB 0.240 GiB（原 1.964 GiB）
- Phase 13 / D14：数据目录统一为当前 EXE 同级 `data\`，无 AppData 回退或自动迁移；新增 `build.bat` + `build.ps1`，以 `pnpm tauri build --no-bundle` 生成 `build\win-unpacked`、版本化 release 目录与绿色 ZIP。真实构建 1m42s；EXE 9.27 MiB、ZIP 2.86 MiB；ZIP 解压后 EXE 启动且窗口正常响应，ZIP 精确包含版本目录 / 空 `data\` / 单个 EXE；便携设置组合测试确认写入 `data\settings.json`
- 2026-08-27 Task 5 UI review 修复：Settings accent 结果回写与 generation 防旧响应覆盖；Save/Reset pending 关闭防守；system `matchMedia` change 重派生草稿；Modal 焦点初始/陷阱/回收；tabpanel/radio/listitem 语义与 320px alert 收缩；新增静态/纯逻辑回归测试 9 个。证据见 `.superpowers/sdd/2026-08-27-settings-overhaul/task-5-fix-report.md`

In Progress:
- None

Blocked:
- None

Tests:
- Passed: cargo 150/150（2026-08-28 full run；含 Git 扫描 Windows 子进程隐藏回归断言、Windows 系统子进程隐藏回归断言、actions_commands 6、launch 38、settings 15、system_commands 17）；vitest 69/69（含 Settings UI 与 Phase 15 图标纯逻辑/静态契约回归，2026-08-28）
- Failed: 0
- Not Run: 0

Build:
- Development: PASS（完整清理后 `start.bat` 冷启动验证，2026-08-26；56.84s，`target` 2.169 GiB）
- Production: PASS（`build.bat` 绿色 ZIP，2026-08-28 Git 扫描窗口修复后重建；绿色 EXE 实际 Add Project / Refresh 无新增顶层窗口；EXE 9,881,088 bytes，ZIP 3,077,042 bytes）

Known Issues:
- Other 可执行文件只保证启动，不保证支持文件夹工作区；本机 Notepad 启动后明确提示无法打开项目文件夹，VS Code 与 Qoder 的真实窗口验证通过
- 验收夹具当前 server.js 与 start-dev.bat 均为修改状态，因此 node-app-e 卡片显示 main · 2 changed；未修改项目记录或夹具文件
- winget 安装 Rustlang.Rustup 时，rustup-init 安装器本体成功，但默认工具链自动安装失败（安装程序退出码 1）；已通过 `rustup default stable` 补装解决
- `%USERPROFILE%\.cargo\bin` 已加入用户 PATH，但当前会话已打开的 shell 不会自动刷新；验证时使用全路径，新开终端可直接使用 `rustc` / `cargo`

Next Step:
- Git 扫描窗口回归修复已完成，`build\\win-unpacked` 与版本化 ZIP 已生成；无未完成代码工作

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
