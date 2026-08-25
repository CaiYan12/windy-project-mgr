# HANDOFF — Windy Project Manager MVP 开发交接

> 生成于 2026-08-25（Phase 9 完成后）。新会话请先读本文，再按「第一优先级」读档。

## 当前状态快照

- **进度**：10 / 15 Phase 完成 · 24 / 34 子任务（权威追踪：`docs/PLAN.MD` 第 0 节）
- **仓库**：`d:\Dev\windy-project-mgr`，分支 `main`，工作区干净，最新提交见 `git log`（Phase 9 功能提交 + 本交接文档提交）
- **代码**：Tauri 2 后端（数据层 + CRUD + scan_project/list_scripts + Git Scanner）+ Dashboard UI + Detail UI（七分区 + 全部降级态 + Actions 含编辑器入口与手动刷新）；Run/Build/Open/Editor/Settings 后端属 Phase 10（前端入口已就位，未注册前点击以 Toast 报错）
- **测试**：`cargo test` 75/75；`pnpm vitest run` 25/25（纯逻辑，无组件渲染测试）；`pnpm build` 通过

## 已完成票（本会话，勿重做）

| Phase | 提交 | 要点 |
|---|---|---|
| 0 环境审计 | （无代码提交） | rustup 1.29.0 + rustc/cargo 1.98.0（stable-msvc）；审计表在 `PROJECT_STATUS.md` |
| 1 项目规划 | `82200eb` | CONTEXT.md / CHANGELOG.md / TESTING.md / docs/adr/0001~0003 |
| 2 工程初始化 | `a77fb42` | create-tauri-app react-ts（pnpm + React 19 + TS 5.8 + Vite 7 + Tauri 2.11.5），dev/build 验证通过 |
| 3 基础目录 | `6fa841d` | src/{components,pages,lib,types} 与 src-tauri/src/{commands,project,scanner,git} |
| 4 数据层 | `ded1de7` | projects.json / settings.json Store，原子写，23 测试（TDD） |
| 5 Project CRUD | `b4eb3d4` | 5 个 Tauri command + D3 路径查重（含词法绝对化），43 测试 |
| 6 Project Scanner | `2cb84c6` | 类型/技术栈/活动/启动脚本枚举（D5），60 测试 |
| 7 Git Scanner | `1c1f255` | `git::scan_git` + `scan_git_with`，系统 Git CLI 离线扫描（D8），真实临时仓库测试 11 个，71 测试 |
| 8 Dashboard UI | `468e934` | scan_project/list_scripts command + 全部 Dashboard 组件（D2/D5/D7/D9/D10），vitest 22 + cargo 75 测试，`pnpm build` 通过 |
| 9 Project Detail UI | （见 `git log`） | `pages/ProjectDetail.tsx` 七分区 + 降级态；`selectedId` 视图切换（无路由库）；卡片点击进 Detail（`stopPropagation`）；`shortHash` 纯函数（TDD，vitest 22→25）；`detect_project_type` 返回 `C#`（原 `CSharp`，红→绿）；双轴评审修正内联样式与 `git = null` Commits 文案；零新依赖；未提前实现 Phase 10 command |

## 下一票：Phase 10 — Run / Build / Open / Editor（PLAN.MD 第 0 节两个复选框）

- `open_project` / `run_project` / `build_project` 按 D4 语义：detached 启动（优先 `wt.exe`，回退 `powershell -NoExit`，`cwd = project.path`，只报启动成败，不采集退出码与输出）；测试覆盖：正常 Command / 空 Command / 错误 Command / 无效 Path
- `get_settings` / `update_settings` / `open_in_editor` 实现并测试（D6）；`open_in_editor` 按 `settings.json` 的 `editorCommand` 拉起，未配置返回可诊断错误（前端显示 `Editor not configured` 并引导配置）
- 前端 `lib/api.ts` 六个封装已就位（Phase 8 先行），command 注册后 Toast 报错过渡态自然消除
- 建议沿用 `*_in` / 可注入核心的可测模式；考虑结构化错误（遗留事项第 2 条）

## 关键工作协议（务必遵守，详见 `AGENTS.md`）

1. **先读**：`AGENTS.md` → `docs/PLAN.MD`（第 0 节进度 + 第 4 节决策 + 第 0.1 节勾选协议）→ `PROJECT_STATUS.md`
2. **TDD**：数据/逻辑票先写失败测试再实现（`/tdd`），垂直切片
3. **票尾**：`/code-review` 双轴评审 → 修正 → 提交（提交信息风格见既有历史）
4. **勾选协议**：验证通过才勾，同步汇总行（当前 10/15 · 24/34，日期 2026-08-25）
5. **会话闭环**：结束前同步 `AGENTS.md` 受影响章节、`docs/PLAN.MD` 进度、长期记忆（UpdateMemory）
6. **硬边界**：pnpm-only、零样式/状态框架、不擅自加依赖、删除记录永不删目录、绝不 `git fetch`

## 已知遗留事项（非阻塞）

- command 错误以字符串返回，前端靠子串匹配区分（如 `path not found`）→ Phase 10 前考虑结构化错误码
- `Cargo.toml` description/authors 仍是模板默认值 → 顺手票修正
- `git` 与 `scanner` 模块各有一份 `require_dir`（错误类型不同，未提取共享）→ 如需复用再重构
- `list_scripts` 超出 PLAN §2.5 command 全集（D5 必要的已记录增量）→ 若后续修订 §2.5 一并纳入
- pnpm 11 将移除 `package.json#pnpm.onlyBuiltDependencies`（当前 10.26.2 正常）→ 升级时迁移到 pnpm-workspace.yaml
- UI 运行时行为未经人工验证（浏览器内无法走 Tauri IPC）→ Phase 12 人工清单覆盖
- 评审判断项（未处理，属可接受惯用法）：卡片与 Detail 的 `ScanState` 三态分支及 Run/Build 禁用形状重复；`ProjectCard` 多处 `stopPropagation` 同形；App 中六个操作回调两处结伴传递

## Suggested Skills（新会话）

- `/frontend-design` — Phase 10 若涉前端调整（AGENTS.md #0 强制）
- `/code-review` — 票尾双轴评审
- `/chinese-encoding` — 中文文档写入防护
- `/context7-mcp` — 查 Tauri IPC / Rust std::process 相关文档
- `/handoff` — 上下文接近饱和时再次交接（禁止 Phase 中途压缩）
