# TESTING.md — 人工验收清单

> 骨架建立于 Phase 1（D12 / D13）；Phase 12 / 14 逐项勾选。自动化测试不在本清单内：Rust 侧 `cargo test`、前端纯逻辑 `vitest`（见 `docs/PLAN.MD` 第 6 节）。
>
> 勾选规则：实际人工操作并确认结果后方可勾选；失败项不勾，记入 `PROJECT_STATUS.md` Blocked / Known Issues。

## 使用说明

1. 验收环境：`pnpm tauri dev`（开发模式）或 Phase 13 production bundle（最终验收）。
2. 离线验收项执行前断开网络。
3. 测试数据准备：至少一个 Node 项目（含 `.bat` / `.ps1` 启动脚本）、一个 Python 项目、一个空目录（Unknown）、一个 Git 仓库、一个非 Git 目录。

---

## A. 全链路用户流程（Phase 12）

- [ ] 启动应用：立即看到卡片骨架，随后逐卡填充扫描结果
- [ ] 添加项目（两步 Add Dialog）：路径选择后 name 自动填充末段、可编辑
- [ ] 添加项目：查重拒绝重复路径并提示（含大小写变体、末尾分隔符变体）
- [ ] Add Dialog Step 2：根目录启动脚本全部列出，按 start > run > 字母序预选；可改选 / 清空 / 跳过
- [ ] 卡片显示：项目类型 / 技术栈 / Git 分支与状态 / 最近活动
- [ ] 项目详情：Header / Overview / Tech / Git / Commits / Activity 全部正确
- [ ] 编辑项目：全字段可改（含 runCommand / buildCommand）
- [ ] 删除项目：确认 Dialog 后仅删记录，项目目录完好
- [ ] Run / Build：分离式终端拉起，cwd 正确；命令为空时按钮禁用并有文案
- [ ] 关闭并重启应用：项目数据与设置全部保留

## B. 异常路径（Phase 12）

- [ ] Git 不可用 / 非 Git 目录：卡片与详情优雅降级，无崩溃
- [ ] 项目路径不存在：明确错误提示，不阻塞其他卡片
- [ ] `projects.json` 损坏：可诊断的错误，不覆盖原文件
- [ ] Run / Build 启动失败（无效路径 / 终端拉起失败）：明确错误提示
- [ ] 空仓库：分支正常、无提交列表、无崩溃
- [ ] detached HEAD：显示 `detached@<短hash>`

## C. 增量验收（决策 5 项）

- [ ] 主题：亮 / 暗 / 跟随系统三种行为正确；手动选择重启后保留
- [ ] 查重：重复路径（含变体）被拒绝并有明确提示
- [ ] 脚本引导：候选列表与预选规则正确
- [ ] 编辑器入口：已配置时卡片 More 与详情可拉起编辑器；未配置时显示 `Editor not configured` 并引导配置
- [ ] 离线可用：断网状态下所有功能正常（无任何 `git fetch` / 网络依赖）

## D. 最终验收（Phase 14）

### Functional

- [ ] Project CRUD 全流程
- [ ] Search：四字段（name / description / tags / path）大小写不敏感即时过滤；空结果 Empty State
- [ ] Sidebar：所有项目 + 标签过滤 + 设置入口
- [ ] Persistence：重启后数据完整

### Error Handling

- [ ] 无效路径 / Git 缺失 / JSON 损坏 / 命令失败 / Scanner 失败 / 权限失败均有明确提示且不崩溃

### UX

- [ ] Empty State / Loading / Error message / Card 布局 / Detail 页 / Dialog / Search 均符合预期

### Stability

- [ ] 启动 / 关闭 / 重复启动 / 连续打开多个项目 / 重复扫描 / 连续 Run / Build 无异常

### Production

- [ ] production bundle 启动、持久化、核心功能全部通过；可执行文件与 bundle 体积实测记录
