# HANDOFF — Windy Project Manager MVP 开发交接

> 生成于 2026-08-25（Phase 11 完成后）。新会话请先读本文，再按「第一优先级」读档。

## 当前状态快照

- **进度**：12 / 15 Phase 完成 · 29 / 34 子任务（权威追踪：`docs/PLAN.MD` 第 0 节）
- **仓库**：`d:\Dev\windy-project-mgr`，分支 `main`，工作区干净，最新提交见 `git log`（Phase 11 功能提交 + 本交接文档提交）
- **代码**：Tauri 2 后端全量完成（13 个 command，§2.5 面完整）+ Dashboard / Detail UI + 主题系统（`data-theme` 三态 + 持久化）+ 可编辑 Settings Dialog；Open/Run/Build/编辑器/主题已全部真实可用
- **测试**：`cargo test` 94/94；`pnpm vitest run` 29/29（纯逻辑，无组件渲染测试）；`pnpm build` 通过

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
| 8 Dashboard UI | `468e934` | scan_project/list_scripts command + 全部 Dashboard 组件（D2/D5/D7/D9/D10），vitest 22 + cargo 75，`pnpm build` 通过 |
| 9 Project Detail UI | `f8067a7` | `pages/ProjectDetail.tsx` 七分区 + 降级态；`selectedId` 视图切换；`shortHash`（vitest 22→25）；`CSharp`→`C#`（红→绿） |
| 10 Run/Build/Open/Editor | `f36ef4e` | `launch` 模块（D4）+ `commands/actions.rs` 六个 command（D6）；TDD 红→绿 19 个（cargo 75→94）；运行时探针 10/10 PASS；零新依赖 |
| 11 UI Polish + 主题系统 | `c53e454` | 主题系统（D1）：`data-theme` 三态（`:root[data-theme="dark"]` 强制暗 + `@media:not([data-theme="light"])` 跟随系统）；`lib/theme.ts` 纯逻辑（TDD 红→绿 4 个，vitest 25→29）；SettingsDialog 由只读占位改可编辑（主题单选组 + 编辑器 datalist 预设 code/code-insiders/cursor + 自定义）；App 启动读取并应用主题（重启保留）；运行时探针验证 D1 全链路（`data-theme=dark` 生效、`--bg=#14181a`、IPC 往返一致，探针已移除）；零新依赖 |

## 下一票：Phase 12 — 全链路集成测试（PLAN.MD 第 0 节一个复选框）

- `TESTING.md` 人工验收清单逐项勾选通过：Functional（CRUD / 扫描 / Detail / Search / 主题）/ Error Handling（Git 缺失 / 路径不存在 / JSON 损坏 / Run 失败）/ UX / Stability（重启持久化）/ Production
- 增量验收：脚本引导（D5）、编辑器打开与 `Editor not configured` 引导（D6）、主题切换与重启持久化（D1）、查重拒绝（D3，大小写/分隔符变体）
- 属**人工验收**（不搭 WebDriver，D12）；需真实运行 `pnpm tauri dev` 逐项勾选 `TESTING.md` 并更新状态

## 关键工作协议（务必遵守，详见 `AGENTS.md`）

1. **先读**：`AGENTS.md` → `docs/PLAN.MD`（第 0 节进度 + 第 4 节决策 + 第 0.1 节勾选协议）→ `PROJECT_STATUS.md`
2. **TDD**：数据/逻辑票先写失败测试再实现（`/tdd`），垂直切片
3. **票尾**：`/code-review` 双轴评审 → 修正 → 提交（提交信息风格见既有历史）
4. **勾选协议**：验证通过才勾，同步汇总行（当前 12/15 · 29/34，日期 2026-08-25）
5. **会话闭环**：结束前同步 `AGENTS.md` 受影响章节、`docs/PLAN.MD` 进度、长期记忆（UpdateMemory）
6. **硬边界**：pnpm-only、零样式/状态框架、不擅自加依赖、删除记录永不删目录、绝不 `git fetch`

## 已知遗留事项（非阻塞）

- command 错误以字符串返回，前端靠子串匹配区分（如 `path not found`）→ 如需结构化错误码另立小票
- `Cargo.toml` description/authors 仍是模板默认值 → 顺手票修正
- `git` / `scanner` / `launch` 模块各有一份 `require_dir`（错误类型不同，未提取共享）→ 如需复用再重构
- `list_scripts` 超出 PLAN §2.5 command 全集（D5 必要的已记录增量）→ 若后续修订 §2.5 一并纳入
- pnpm 11 将移除 `package.json#pnpm.onlyBuiltDependencies`（当前 10.26.2 正常）→ 升级时迁移到 pnpm-workspace.yaml
- UI 运行时行为未经人工验证 → Phase 12 人工清单覆盖
- **WebView2 Runtime ≥150 对提权宿主禁用 CDP 注入（wry#1782 / WebView2Feedback#5640）**；本机开发终端为提权（High Integrity），运行时验证用「临时前端探针 + 临时 `debug_log` command」方案（Phase 10/11 已验证可行，探针代码不提交）
- App.css 暗色变量集在 `:root[data-theme="dark"]` 与 `@media` 各写一份（纯 CSS 无法复用），已加相互注释避免漂移
- 评审判断项（可接受，未处理）：`AppSettings.theme` 用 `string` 而非 `Theme` 联合类型（靠 `isValidTheme` 守卫）

## Suggested Skills（新会话）

- `/frontend-design` — Phase 12 若涉 UI 调整（AGENTS.md #0 强制）
- `/code-review` — 票尾双轴评审
- `/chinese-encoding` — 中文文档写入防护
- `/context7-mcp` — 查 Tauri / React 相关文档
- `/handoff` — 上下文接近饱和时再次交接（禁止 Phase 中途压缩）