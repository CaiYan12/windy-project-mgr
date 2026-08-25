# HANDOFF — Windy Project Manager MVP 开发交接

> 生成于 2026-08-25（Phase 8 完成后）。新会话请先读本文，再按「第一优先级」读档。

## 当前状态快照

- **进度**：9 / 15 Phase 完成 · 22 / 34 子任务（权威追踪：`docs/PLAN.MD` 第 0 节）
- **仓库**：`d:\Dev\windy-project-mgr`，分支 `main`，工作区干净，最新提交 `468e934`
- **代码**：Tauri 2 后端（数据层 + CRUD + scan_project/list_scripts + Git Scanner）+ Dashboard UI（卡片骨架/并发扫描填充、Sidebar 标签过滤、Search、Add/Edit/删除 Dialog、More 菜单）；Detail 页未做；Run/Build/Open/Editor/Settings 后端属 Phase 10（前端入口已就位，未注册前点击以 Toast 报错）
- **测试**：`cargo test` 75/75；`pnpm vitest run` 22/22（纯逻辑，无组件渲染测试）；`pnpm build` 通过

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

## 下一票：Phase 9 — Project Detail UI（PLAN.MD 第 0 节两个复选框）

- Header / Overview / Tech / Git / Commits / Activity / Actions 全部呈现，降级文案正确（复用 `lib/cards.ts` 与 §11 文案；recentCommits 列表展示：hash 短码 + message + author + 相对时间）
- Actions 含“在编辑器中打开” + 手动刷新按钮（D6/D2：重扫调 `scan_project`，结果仅存内存；Detail 复用 Dashboard 的内存缓存）
- 建议：卡片点击进 Detail（视图切换用 React 状态，无路由库）；编辑入口复用 `EditProjectDialog`；git 为 null / lastCommit 为 null / status unknown 的降级态必须实现并可见

## 关键工作协议（务必遵守，详见 `AGENTS.md`）

1. **先读**：`AGENTS.md` → `docs/PLAN.MD`（第 0 节进度 + 第 4 节决策 + 第 0.1 节勾选协议）→ `PROJECT_STATUS.md`
2. **TDD**：数据/逻辑票先写失败测试再实现（`/tdd`），垂直切片
3. **票尾**：`/code-review` 双轴评审 → 修正 → 提交（提交信息风格见既有历史）
4. **勾选协议**：验证通过才勾，同步汇总行（当前 9/15 · 22/34，日期 2026-08-25）
5. **会话闭环**：结束前同步 `AGENTS.md` 受影响章节、`docs/PLAN.MD` 进度、长期记忆（UpdateMemory）
6. **硬边界**：pnpm-only、零样式/状态框架、不擅自加依赖、删除记录永不删目录、绝不 `git fetch`

## 已知遗留事项（非阻塞）

- 类型名 `CSharp` 与技术栈标签 `C#` 不一致 → Detail/UI 展示前统一
- command 错误以字符串返回，前端靠子串匹配区分（如 `path not found`）→ Phase 10 前考虑结构化错误码
- `Cargo.toml` description/authors 仍是模板默认值 → 顺手票修正
- `git` 与 `scanner` 模块各有一份 `require_dir`（错误类型不同，未提取共享）→ 如需复用再重构
- `list_scripts` 超出 PLAN §2.5 command 全集（D5 必要的已记录增量）→ 若后续修订 §2.5 一并纳入
- pnpm 11 将移除 `package.json#pnpm.onlyBuiltDependencies`（当前 10.26.2 正常）→ 升级时迁移到 pnpm-workspace.yaml
- UI 运行时行为未经人工验证（浏览器内无法走 Tauri IPC）→ Phase 12 人工清单覆盖

## Suggested Skills（新会话）

- `/frontend-design` — Phase 9 Detail 页前端编码（AGENTS.md #0 强制）
- `/code-review` — 票尾双轴评审
- `/chinese-encoding` — 中文文档写入防护
- `/context7-mcp` — 查 Tauri IPC / React 相关文档
- `/handoff` — 上下文接近饱和时再次交接（禁止 Phase 中途压缩）
