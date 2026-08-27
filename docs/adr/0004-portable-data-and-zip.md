# ADR 0004: EXE 相邻数据目录与绿色 ZIP 分发

- 状态：Accepted（2026-08-26）
- 决策者：项目负责人 + 执行 Agent

## 背景

项目需要以 ZIP 形式交付，解压后直接运行，并能在复制或移动整个程序目录时携带项目记录与设置。原先 `%APPDATA%\windy-project-mgr` 的固定数据目录不满足完全便携语义。

## 决策

- 所有运行形态统一把 `projects.json` 与 `settings.json` 写入当前 EXE 同目录的 `data\`。
- 不使用便携标记，不保留 AppData 回退，不自动读取、复制或删除旧 AppData 数据。
- `build.bat` 调用 `build.ps1`，执行 `pnpm tauri build --no-bundle`，生成 `build\win-unpacked\`、版本化 release 目录及 ZIP；不生成 MSI/NSIS。
- ZIP 内包含版本目录、release EXE 与空 `data\`，解压后可直接运行。

## 权衡与后果

- 整个解压目录可移动和备份，数据随程序移动。
- 程序目录必须可写；放入只读目录时保存会返回现有可诊断 IO 错误。
- 开发 EXE 位于 `src-tauri\target\debug\`，因此开发数据位于 `target\debug\data\`，执行 `cargo clean` 会删除这些开发数据。
- 旧 `%APPDATA%\windy-project-mgr` 数据不会自动出现在新版本中；如需保留，用户必须手动复制 JSON 文件。
