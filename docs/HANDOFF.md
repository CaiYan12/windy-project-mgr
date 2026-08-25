# HANDOFF — Windy Project Manager MVP 开发交接

> 生成于 2026-08-25（Phase 7 完成后）。新会话请先读本文，再按「第一优先级」读档。

## 当前状态快照

- **进度**：8 / 15 Phase 完成 · 17 / 34 子任务（权威追踪：`docs/PLAN.MD` 第 0 节）
- **仓库**：`d:\Dev\windy-project-mgr`，分支 `main`，最新提交即 Phase 7 票提交（`git log -1` 可查）
- **代码**：Tauri 2 工程骨架 + 数据层 + CRUD commands + Scanner + Git Scanner 已实现；前端仍是模板 UI；`scan_project` command 尚未实现（Phase 8 组装时接入 `git::scan_git`）
- **测试**：`cargo test` 71/71 通过（src-tauri）；前端 vitest 尚未引入（属后续票面）

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
| 7 Git Scanner | （本次会话提交，见 `git log`） | `git::scan_git` + `scan_git_with`，系统 Git CLI 离线扫描（D8），真实临时仓库测试 11 个，71 测试 |

## 下一票：Phase 8 — Dashboard UI（PLAN.MD 第 0 节五个复选框）

- 卡片骨架先行 + 并发 `scan_project` 逐卡填充（D2）；`scan_project` command 需在本票实现并组装完整 `ProjectMetadata`（接入已有 `scanner` + `git::scan_git`，D8 git 字段为 `Option`）
- Sidebar：所有项目 + 标签过滤（从项目数据自动提取，D9）+ 设置入口
- Search：name / description / tags / path 四字段，大小写不敏感子串，纯前端即时过滤（D7）
- 两步 Add Dialog（含脚本引导 D5 与查重 D3/D10）与删除确认 Dialog；卡片 More 菜单含“在编辑器中打开”入口（D6，`open_in_editor` command 本体属 Phase 10）
- 前端约束（D11）：仅 React 内置 hooks + 纯 CSS 变量，零样式/状态框架；纯逻辑测试用 vitest（本票引入，仅搜索过滤/路径规范化/卡片数据组装，不写组件渲染测试）

## 关键工作协议（务必遵守，详见 `AGENTS.md`）

1. **先读**：`AGENTS.md` → `docs/PLAN.MD`（第 0 节进度 + 第 4 节决策 + 第 0.1 节勾选协议）→ `PROJECT_STATUS.md`
2. **TDD**：数据/逻辑票先写失败测试再实现（`/tdd`），垂直切片
3. **票尾**：`/code-review` 双轴评审 → 修正 → 提交（提交信息风格见既有历史）
4. **勾选协议**：验证通过才勾，同步汇总行（当前 8/15 · 17/34，日期 2026-08-25）
5. **会话闭环**：结束前同步 `AGENTS.md` 受影响章节、`docs/PLAN.MD` 进度、长期记忆（UpdateMemory）
6. **硬边界**：pnpm-only、零样式/状态框架、不擅自加依赖、删除记录永不删目录、绝不 `git fetch`

## 已知遗留事项（非阻塞）

- 类型名 `CSharp` 与技术栈标签 `C#` 不一致 → Phase 8 UI 前统一
- command 错误以字符串返回，Phase 8 前端需区分查重拒绝/NotFound → 届时考虑结构化错误码
- `Cargo.toml` description/authors 仍是模板默认值 → 顺手票修正
- `git` 与 `scanner` 模块各有一份 `require_dir`（错误类型不同，未提取共享）→ 如需复用再重构
- pnpm 11 将移除 `package.json#pnpm.onlyBuiltDependencies`（当前 10.26.2 正常）→ 升级时迁移到 pnpm-workspace.yaml

## Suggested Skills（新会话）

- `/frontend-design` — Phase 8 前端编码（AGENTS.md #0 强制）
- `/code-review` — 票尾双轴评审
- `/chinese-encoding` — 中文文档写入防护
- `/context7-mcp` — 查 Tauri IPC / React 相关文档
- `/handoff` — 上下文接近饱和时再次交接（禁止 Phase 中途压缩）
