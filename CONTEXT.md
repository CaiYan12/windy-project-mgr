# CONTEXT.md — 术语表

> 本文件仅定义领域术语（D13）；术语背后的决策与实现规范见 `docs/PLAN.MD` 与 `docs/adr/`，原始需求见 `docs/Windy Project Manager - Primary Request&Plan Document.md`。

## 核心概念

| 术语 | 定义 |
|---|---|
| Project（项目记录） | 用户登记的一个本地项目。持久化字段仅为：id、name、path、description、tags、runCommand、buildCommand、createdAt。 |
| 项目目录 | Project.path 指向的真实文件系统目录。**删除项目记录永远不删除项目目录。** |
| ProjectMetadata（项目元数据） | 对项目目录的运行时扫描结果（项目类型、技术栈、Git 信息、活动信息）。不持久化（见 ADR 0001）。 |
| 卡片（Card） | Dashboard 上呈现单个项目的最小信息单元；扫描完成前先显示骨架（Skeleton），扫描结果到达后逐项填充。 |

## 扫描（Scanner）

| 术语 | 定义 |
|---|---|
| Scanner | 对一个项目路径产出 ProjectMetadata 的完整管线：项目类型检测 → 技术栈检测 → Git 扫描 → 活动扫描。 |
| Project Type（项目类型） | 由根目录特征文件判定的项目类别（如 Node、Python、Rust、Unknown）。 |
| Tech Stack（技术栈） | 由特征文件判定的技术标签列表。 |
| 启动脚本 | 项目根目录下的 `*.bat` / `*.cmd` / `*.ps1` 文件（不递归）。Add Dialog Step 2 枚举并辅助用户配置 runCommand。 |
| Activity（活动） | 项目的最近修改时间与本次运行内的扫描时刻。 |

## Git

| 术语 | 定义 |
|---|---|
| GitMetadata | Git 扫描结果：分支、三态状态（clean / modified / unknown）、修改文件数、ahead/behind、最近提交、最近 10 条提交列表。 |
| 无上游分支 | 分支未设置 `@{u}` 时，ahead/behind 记为 0 且 UI 不显示。 |
| 空仓库 | 无任何 commit 的仓库：分支正常返回，lastCommit 为 null，提交列表为空。 |
| detached HEAD | 不指向分支的检出状态，分支名表示为 `detached@<短hash>`。 |
| 离线约束 | 本应用绝不执行 `git fetch` 或任何网络操作；所有 Git 信息取自本地状态。 |

## 项目操作

| 术语 | 定义 |
|---|---|
| Run Command / Build Command | 用户为项目配置的运行 / 构建命令。为空时对应按钮禁用并显示未配置文案。 |
| 分离式启动（Detached Launch） | 在独立系统终端中启动命令；应用只报告启动动作的成功 / 失败，不采集命令退出码与输出（见 ADR 0002）。 |
| Open | 在系统文件管理器中打开项目目录。 |
| Editor Profile（编辑器配置） | 用于“在编辑器中打开”的可执行文件路径与逐行参数；参数必须恰好包含一个 `{path}`，由启动层替换为项目目录。 |

## 查重与设置

| 术语 | 定义 |
|---|---|
| 查重（Path Dedup） | 添加项目时对路径做规范化后比较；命中重复即拒绝创建，不合并（见 D3）。 |
| Color Mode（颜色模式） | 界面明暗行为：亮 / 暗 / 跟随系统三选项；默认跟随系统，独立于强调色保存。 |
| Accent Color（强调色） | Windy teal、五个其它预设、Windows 当前颜色或合法的 `#RRGGBB` 自定义颜色；用于派生全局 CSS 变量。 |
| Settings（设置） | 版本化的应用级配置：`colorMode`、`accentColor` 与 `editor`；v1 的 `theme` / `editorCommand` 会自动迁移。 |
