# TESTING.md — 人工验收清单

> 清单建立于 Phase 1（D12 / D13）；Phase 12 / 14 已完成逐项人工验收。自动化测试不在本清单内：Rust 侧 `cargo test`、前端纯逻辑 `vitest`（见 `docs/PLAN.MD` 第 6 节）。
>
> 勾选规则：实际人工操作并确认结果后方可勾选；失败项不勾，记入 `PROJECT_STATUS.md` Blocked / Known Issues。

## 使用说明

1. 验收环境：`pnpm tauri dev`（开发模式）或 Phase 13 绿色 ZIP 解压目录（最终验收）。
2. 离线验收项执行前断开网络。
3. 测试数据准备：至少一个 Node 项目（含 `.bat` / `.ps1` 启动脚本）、一个 Python 项目、一个空目录（Unknown）、一个 Git 仓库、一个非 Git 目录。

---

## A. 全链路用户流程（Phase 12）

- [x] 启动应用：立即看到卡片骨架，随后逐卡填充扫描结果
- [x] 添加项目（两步 Add Dialog）：路径选择后 name 自动填充末段、可编辑
- [x] 添加项目：查重拒绝重复路径并提示（含大小写变体、末尾分隔符变体）
- [x] Add Dialog Step 2：根目录启动脚本全部列出，按 start > run > 字母序预选；可改选 / 清空 / 跳过
- [x] 卡片显示：项目类型 / 技术栈 / Git 分支与状态 / 最近活动
- [x] 项目详情：Header / Overview / Tech / Git / Commits / Activity 全部正确
- [x] 编辑项目：全字段可改（含 runCommand / buildCommand）
- [x] 删除项目：确认 Dialog 后仅删记录，项目目录完好
- [x] Run / Build：分离式终端拉起，cwd 正确；命令为空时按钮禁用并有文案
- [x] 关闭并重启应用：项目数据与设置全部保留

## B. 异常路径（Phase 12）

- [x] Git 不可用 / 非 Git 目录：卡片与详情优雅降级，无崩溃
- [x] 项目路径不存在：明确错误提示，不阻塞其他卡片
- [x] `projects.json` 损坏：可诊断的错误，不覆盖原文件
- [x] Run / Build 启动失败（无效路径 / 终端拉起失败）：明确错误提示
- [x] 空仓库：分支正常、无提交列表、无崩溃
- [x] detached HEAD：显示 `detached@<短hash>`

## C. 增量验收（决策 5 项）

- [x] 主题：亮 / 暗 / 跟随系统三种行为正确；手动选择重启后保留
- [x] 查重：重复路径（含变体）被拒绝并有明确提示
- [x] 脚本引导：候选列表与预选规则正确
- [x] 编辑器入口：已配置时卡片 More 与详情可拉起编辑器；未配置时显示 `Editor not configured` 并引导配置
- [x] 离线可用：断网状态下所有功能正常（无任何 `git fetch` / 网络依赖）

### C.1 Settings v2 收尾验收（2026-08-27）

- [x] Appearance / Editor / General 三分页可切换，亮 / 暗 / 跟随系统即时生效；Dark 保存后重启仍保留
- [x] 六种预设强调色、Windows 当前主题色与自定义 `#RRGGBB` 可预览；非法颜色会提示并禁用保存
- [x] Cancel 恢复颜色模式与强调色的完整未保存草稿
- [x] General 显示版本与便携数据目录；Reset settings 完成二次确认与最终验收，项目记录未受影响
- [x] Editor 展示真实发现路径与来源；PATH 中的 Qoder 路径显示为 `PATH command: code`，不冒认 VS Code
- [x] VS Code 真实打开 `node-app` 文件夹工作区；Qoder 的 `.cmd` 入口也真实打开同一文件夹工作区
- [x] Other 原生选择器限制为 `*.exe`；未扫描到的 `notepad.exe` 显示为选中的 Custom executable，保存/重开后仍显示
- [x] 编辑器参数缺少或重复 `{path}` 时提示并禁用 Save；恰好一个占位符可保存
- [x] 未配置编辑器时入口显示 `Editor not configured`
- [x] 物理断网后的运行验收：最终人工验收通过；无网络请求和 `git fetch`
- [x] 320 / 768 / 1024 / 1440 宽度的真实窗口调整：最终人工验收通过

> Other 可执行文件只保证能启动，不保证支持文件夹工作区。Windows Notepad 被选中后确实启动，但由 Notepad 自己提示无法打开项目文件夹；文件夹工作区能力以 VS Code / Qoder 的实测为准。

## D. 最终验收（Phase 14）

### Functional

- [x] Project CRUD 全流程
- [x] Search：四字段（name / description / tags / path）大小写不敏感即时过滤；空结果 Empty State
- [x] Sidebar：所有项目 + 标签过滤 + 设置入口
- [x] Persistence：重启后数据完整

### Error Handling

- [x] 无效路径 / Git 缺失 / JSON 损坏 / 命令失败 / Scanner 失败 / 权限失败均有明确提示且不崩溃

### UX

- [x] Empty State / Loading / Error message / Card 布局 / Detail 页 / Dialog / Search 均符合预期

### Stability

- [x] 启动 / 关闭 / 重复启动 / Settings 与 Add Project 重复开关 / 连续打开多个项目 / 重复扫描 / 连续 Run / Build 无异常

### Production

- [x] `build.bat` 生成版本化绿色目录与 ZIP，ZIP 解压后的 EXE 可启动且窗口正常响应
- [x] 便携数据契约：设置写入 EXE 同目录 `data\settings.json`，不使用 `%APPDATA%`
- [x] 绿色 ZIP 内 Project CRUD / 主题持久化 / Run / Build / Open in editor 全流程人工验收

---

## E. Phase 15 自绘 SVG 图标与全页面 UI 审计（2026-08-27）

| 验收项 | 结果 | 证据 / 边界 |
|---|---|---|
| 图标注册表与 SVG 契约 | PASS | `Icon.tsx` 使用零依赖、24×24 `currentColor`、圆端/圆角线条；`iconUi.test.ts` 静态契约通过 |
| 全页面语义接入 | PASS | Dashboard、Sidebar、Card、Detail、Add/Edit/Confirm、Settings 三分页、More 菜单、Toast、空态、错误态、加载态均有语义图标 |
| icon-only 无障碍名称 | PASS | More、Close、Dismiss 等紧凑控件保留 `aria-label` 与 tooltip；装饰图标默认 `aria-hidden` |
| 主题与状态表达 | PASS | Light / Dark / Follow system、强调色预览及 Git clean / modified / unknown 的形状 + 颜色均完成真实窗口复核 |
| 键盘焦点与命中区 | PASS | 搜索、Add project、卡片标题、More 菜单等真实 Tauri 窗口路径均观察到可见焦点环；icon-only 目标保持 32px 命中区 |
| 响应式几何与文本收缩 | PASS | 静态 Vite 页面在 320 / 768 / 1024 / 1440 宽度下 `scrollWidth` 与 viewport 一致；Modal、Settings、菜单和长文本无水平溢出或图标重叠 |
| 真实 Tauri 交互窗口 | PASS | 默认开发窗口约 802×631，完成 Dashboard、Detail、Add/Edit/Confirm、More、Settings、Toast/错误路径交互复核；未将原生拖拽缩放失败冒充为多尺寸证据 |
| reduced-motion | PASS | 保留既有 `prefers-reduced-motion` 规则，并由静态契约测试断言；本轮未声称有独立 runtime 动画截图 |

> 尺寸边界：320 / 768 / 1024 / 1440 的数值几何来自同一 Vite 页面静态浏览器检查；真实 Tauri 只记录默认窗口交互结果。图标审计不改变 Rust、IPC、持久化或命令契约。

### E.1 图标细节 follow-up（2026-08-28）

- Card / Detail 的 Open、Run、Build 图标统一使用 16px 盒、固定文字行高与上下居中规则。
- `hammer` glyph 已替换为用户指定的 1024×1024 填充 path，按 `scale(0.0234375)` 归一到 24×24，并使用 `currentColor` / `stroke="none"` 保持主题适配。
- 本次按需求未启动实机窗口；`build.bat` 与静态 `pnpm test`（7 个文件 / 69 个测试）通过，产物见 `build\\win-unpacked\\windy-project-mgr.exe`。

### E.2 Git 扫描控制台窗口回归（2026-08-28）

- [x] 绿色 EXE 通过 `D:\Dev\opia-rss-reader` 完成 Add Project；Step 2 列出 `start.bat`、`build.bat`、`build.ps1`，提交后卡片正常出现并完成扫描。
- [x] 绿色 EXE 在 Detail 页执行 Refresh；Add Project 阶段所有观测 Git 子进程均为 `hwnd=0`，Refresh 观察 3 秒无新增顶层窗口和错误文案；验证后清理本次写入的项目记录及应用进程。
