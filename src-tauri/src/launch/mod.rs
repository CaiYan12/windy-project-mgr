//! 分离式终端启动（D4 / ADR 0002）：优先 `wt.exe`，探测失败回退 `powershell -NoExit`。
//!
//! 启动后应用立即返回，只报告启动动作的成功 / 失败，不采集命令退出码与输出。
//! “启动失败”= 无效路径 / 终端拉起失败，命令本身执行失败不属于应用缺陷。
//!
//! 设计为可测：`*_plan` 函数纯构造启动计划（程序 / 参数 / 工作目录），
//! `spawn_plan` 才真正拉起进程；二进制名可注入，便于在测试中替换。

use std::fmt;
use std::path::{Path, PathBuf};

/// 启动层错误。文案需可诊断（前端以字符串呈现）。
#[derive(Debug)]
pub enum LaunchError {
    /// 目标路径不存在或不是目录。
    PathNotFound { path: PathBuf },
    /// 命令为空（run/build 未配置；前端已禁用按钮，此处为后端兜底）。
    EmptyCommand,
    /// 编辑器未配置（D6：`editorCommand` 为空）。
    EditorNotConfigured,
    /// 终端 / 进程拉起失败。
    LaunchFailed { detail: String },
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathNotFound { path } => write!(f, "path not found: {}", path.display()),
            Self::EmptyCommand => write!(f, "command is empty"),
            Self::EditorNotConfigured => write!(f, "Editor not configured"),
            Self::LaunchFailed { detail } => write!(f, "launch failed: {detail}"),
        }
    }
}

impl std::error::Error for LaunchError {}

/// 启动计划（纯数据）：要拉起的程序、参数与工作目录。
#[derive(Debug)]
pub struct LaunchPlan {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
}

/// Windows Terminal 计划：`wt -d <cwd> <ps> -NoExit -Command <command>`。
pub fn wt_plan(cwd: &Path, command: &str, wt_bin: &str, ps_bin: &str) -> LaunchPlan {
    LaunchPlan {
        program: wt_bin.to_string(),
        args: vec![
            "-d".to_string(),
            cwd.display().to_string(),
            ps_bin.to_string(),
            "-NoExit".to_string(),
            "-Command".to_string(),
            command.to_string(),
        ],
        cwd: cwd.to_path_buf(),
    }
}

/// PowerShell 回退计划：`<ps> -NoExit -Command <command>`（工作目录设在进程上）。
pub fn ps_plan(cwd: &Path, command: &str, ps_bin: &str) -> LaunchPlan {
    LaunchPlan {
        program: ps_bin.to_string(),
        args: vec![
            "-NoExit".to_string(),
            "-Command".to_string(),
            command.to_string(),
        ],
        cwd: cwd.to_path_buf(),
    }
}

/// 在编辑器中打开的计划（D6）：`<editor> <path>`，工作目录为项目路径。
pub fn editor_plan(editor_command: &str, path: &Path) -> Result<LaunchPlan, LaunchError> {
    let editor = editor_command.trim();
    if editor.is_empty() {
        return Err(LaunchError::EditorNotConfigured);
    }
    Ok(LaunchPlan {
        program: editor.to_string(),
        args: vec![path.display().to_string()],
        cwd: path.to_path_buf(),
    })
}

/// 打开目录的计划（Open）：`explorer <path>`。
pub fn open_dir_plan(path: &Path) -> LaunchPlan {
    LaunchPlan {
        program: "explorer".to_string(),
        args: vec![path.display().to_string()],
        cwd: path.to_path_buf(),
    }
}

/// 确认目标是存在的目录；不存在 / 非目录 → 可诊断错误。
fn require_dir(path: &Path) -> Result<(), LaunchError> {
    match std::fs::metadata(path) {
        Ok(m) if m.is_dir() => Ok(()),
        _ => Err(LaunchError::PathNotFound {
            path: path.to_path_buf(),
        }),
    }
}

/// 执行计划：detached 拉起进程并立即返回（不等待、不采集输出）。
pub fn spawn_plan(plan: &LaunchPlan) -> Result<(), LaunchError> {
    std::process::Command::new(&plan.program)
        .args(&plan.args)
        .current_dir(&plan.cwd)
        .spawn()
        .map(|_child| ())
        .map_err(|e| LaunchError::LaunchFailed {
            detail: format!("{}: {e}", plan.program),
        })
}

/// 在终端中运行命令（D4）：优先 `wt.exe`，拉起失败回退 `powershell -NoExit`。
pub fn run_in_terminal(cwd: &Path, command: &str) -> Result<(), LaunchError> {
    run_in_terminal_with(cwd, command, "wt.exe", "powershell")
}

/// 可测核心：终端二进制名可注入。
pub fn run_in_terminal_with(
    cwd: &Path,
    command: &str,
    wt_bin: &str,
    ps_bin: &str,
) -> Result<(), LaunchError> {
    require_dir(cwd)?;
    if command.trim().is_empty() {
        return Err(LaunchError::EmptyCommand);
    }
    // 优先 wt；拉起失败（如未安装）回退 powershell。
    if spawn_plan(&wt_plan(cwd, command, wt_bin, ps_bin)).is_ok() {
        return Ok(());
    }
    spawn_plan(&ps_plan(cwd, command, ps_bin))
}

/// 打开项目目录（Open）。
pub fn open_dir(path: &Path) -> Result<(), LaunchError> {
    require_dir(path)?;
    spawn_plan(&open_dir_plan(path))
}

/// 在编辑器中打开（D6）：`editor_command` 为空 → `EditorNotConfigured`。
pub fn open_in_editor(editor_command: &str, path: &Path) -> Result<(), LaunchError> {
    let plan = editor_plan(editor_command, path)?;
    require_dir(path)?;
    spawn_plan(&plan)
}
