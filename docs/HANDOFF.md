# HANDOFF — Windy Project Manager MVP 开发交接

> 生成于 2026-08-25（Phase 6 完成后）。新会话请先读本文，再按「第一优先级」读档。

## 当前状态快照

- **进度**：7 / 15 Phase 完成 · 15 / 34 子任务（权威追踪：`docs/PLAN.MD` 第 0 节）
- **仓库**：`d:\Dev\windy-project-mgr`，分支 `main`，工作区干净，最新提交 `2cb84c6`
- **代码**：Tauri 2 工程骨架 + 数据层 + CRUD commands + Scanner 已实现；前端仍是模板 UI
- **测试**：`cargo test` 60/60 通过（src-tauri）；前端 vitest 尚未引入（属后续票面）

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

## 下一票：Phase 7 — Git Scanner（PLAN.MD 第 0 节两个复选框）

- 按 D8 边界实现（`docs/PLAN.MD` 第 4 节）：绝不 `git fetch`；ahead/behind 基于本地 `@{u}`，无上游 = 0 且 UI 不显示；`recentCommits` 最近 10 条；空仓库 `lastCommit = null`；detached HEAD → `detached@<短hash>`；status 三态 clean / modified / unknown
- 专项测试：无上游分支 / 空仓库 / detached HEAD，用**真实临时 git 仓库**（D12，不只 Mock）
- 实现位置：`src-tauri/src/git/`（当前空模块）；GitMetadata 结构见 `docs/PLAN.MD` §2.3
- 建议接缝：`git/mod.rs` 暴露 `scan_git(path) -> Result<Option<GitMetadata>, GitError>`（非仓库返回 None，git 不可用返回明确错误），内部以 `Command::new("git")` 调用系统 CLI；沿用 scanner 的容错风格（可诊断错误、不崩溃）

## 关键工作协议（务必遵守，详见 `AGENTS.md`）

1. **先读**：`AGENTS.md` → `docs/PLAN.MD`（第 0 节进度 + 第 4 节决策 + 第 0.1 节勾选协议）→ `PROJECT_STATUS.md`
2. **TDD**：数据/逻辑票先写失败测试再实现（`/tdd`），垂直切片
3. **票尾**：`/code-review` 双轴评审 → 修正 → 提交（提交信息风格见既有历史）
4. **勾选协议**：验证通过才勾，同步汇总行（当前 7/15 · 15/34，日期 2026-08-25）
5. **会话闭环**：结束前同步 `AGENTS.md` 受影响章节、`docs/PLAN.MD` 进度、长期记忆（UpdateMemory）
6. **硬边界**：pnpm-only、零样式/状态框架、不擅自加依赖、删除记录永不删目录、绝不 `git fetch`

## 已知遗留事项（非阻塞）

- 类型名 `CSharp` 与技术栈标签 `C#` 不一致 → Phase 8 UI 前统一
- command 错误以字符串返回，Phase 8 前端需区分查重拒绝/NotFound → 届时考虑结构化错误码
- `Cargo.toml` description/authors 仍是模板默认值 → 顺手票修正
- pnpm 11 将移除 `package.json#pnpm.onlyBuiltDependencies`（当前 10.26.2 正常）→ 升级时迁移到 pnpm-workspace.yaml

## Suggested Skills（新会话）

- `/tdd` — Phase 7 测试先行
- `/code-review` — 票尾双轴评审
- `/chinese-encoding` — 中文文档写入防护
- `/context7-mcp` — 查 Tauri / git2-free CLI 调用相关文档
- `/handoff` — 上下文接近饱和时再次交接（禁止 Phase 中途压缩）
