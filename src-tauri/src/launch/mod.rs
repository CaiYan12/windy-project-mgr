//! 分离式终端启动（D4 / ADR 0002）：优先 `wt.exe`，探测失败回退 `powershell -NoExit`。
//!
//! 启动后应用立即返回，只报告启动动作的成功 / 失败，不采集命令退出码与输出。
//! “启动失败”= 无效路径 / 终端拉起失败，命令本身执行失败不属于应用缺陷。
//!
//! 设计为可测：`*_plan` 函数纯构造启动计划（程序 / 参数 / 工作目录），
//! `spawn_plan` 才真正拉起进程；二进制名可注入，便于在测试中替换。

use std::fmt;
use std::path::{Path, PathBuf};

use crate::project::settings::{is_batch_executable, EditorProfile};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// 启动层错误。文案需可诊断（前端以字符串呈现）。
#[derive(Debug)]
pub enum LaunchError {
    /// 目标路径不存在或不是目录。
    PathNotFound { path: PathBuf },
    /// 命令为空（run/build 未配置；前端已禁用按钮，此处为后端兜底）。
    EmptyCommand,
    /// 编辑器未配置（D6：`editorCommand` 为空）。
    EditorNotConfigured,
    /// 编辑器配置非法（绕过 settings 验证时的启动层兜底）。
    InvalidEditorProfile { detail: String },
    /// 终端 / 进程拉起失败。
    LaunchFailed { detail: String },
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathNotFound { path } => write!(f, "path not found: {}", path.display()),
            Self::EmptyCommand => write!(f, "command is empty"),
            Self::EditorNotConfigured => write!(f, "Editor not configured"),
            Self::InvalidEditorProfile { detail } => {
                write!(f, "invalid editor profile: {detail}")
            }
            Self::LaunchFailed { detail } => write!(f, "launch failed: {detail}"),
        }
    }
}

impl std::error::Error for LaunchError {}

/// 启动计划（纯数据）：要拉起的程序、参数、工作目录与子进程环境变量。
#[derive(Debug)]
pub struct LaunchPlan {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
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
            quote_pwsh_command_in(cwd, command),
        ],
        cwd: cwd.to_path_buf(),
        env: Vec::new(),
    }
}

/// PowerShell 回退计划：`<ps> -NoExit -Command <command>`（工作目录设在进程上）。
pub fn ps_plan(cwd: &Path, command: &str, ps_bin: &str) -> LaunchPlan {
    LaunchPlan {
        program: ps_bin.to_string(),
        args: vec![
            "-NoExit".to_string(),
            "-Command".to_string(),
            quote_pwsh_command_in(cwd, command),
        ],
        cwd: cwd.to_path_buf(),
        env: Vec::new(),
    }
}

/// PowerShell `-Command` 命令的安全包装：
/// 命令形如**路径**（盘符 / UNC / `.` 相对 / .bat/.cmd/.ps1 结尾）时用调用运算符
/// `& '...'` 包裹 —— 否则含空格路径会被 PowerShell 按空格切开（如
/// `C:\Users\Einn Tzai\...` 被当成命令 `C:\Users\Einn`）。
/// 仅可执行路径进入单引号，后续参数保持原始命令文本。
/// 用**单引号**而非双引号：Windows 的 CommandLineToArgvW 会把参数内双引号剥离，
/// 单引号作为普通字符完整传递；PowerShell 中单引号即字面字符串。
/// 普通「程序 + 参数」命令（如 `pnpm dev`）原样返回，不受影响。
pub fn quote_pwsh_command(command: &str) -> String {
    quote_pwsh_command_in(Path::new("."), command)
}

fn quote_pwsh_command_in(cwd: &Path, command: &str) -> String {
    let Some((leading, executable, arguments)) = split_pwsh_path_command(cwd, command) else {
        return command.to_string();
    };

    format!(
        "{leading}& '{}'{}",
        executable.replace('\'', "''"),
        arguments
    )
}

fn split_pwsh_path_command<'a>(cwd: &Path, command: &'a str) -> Option<(&'a str, &'a str, &'a str)> {
    let input = command.trim_start();
    let leading = &command[..command.len() - input.len()];

    if let Some(quoted_input) = input.strip_prefix('"') {
        let closing_quote = quoted_input.find('"')?;
        let executable = &quoted_input[..closing_quote];
        if !is_path_like(executable) {
            return None;
        }
        return Some((leading, executable, &quoted_input[closing_quote + 1..]));
    }

    if let Some(executable_end) = find_path_extension_end(input)
        .filter(|end| is_path_like(&input[..*end]))
    {
        return Some((leading, &input[..executable_end], &input[executable_end..]));
    }

    if path_is_existing_file(cwd, input) {
        return Some((leading, input, ""));
    }

    if let Some(executable_end) = find_existing_path_end(cwd, input) {
        return Some((leading, &input[..executable_end], &input[executable_end..]));
    }

    None
}

fn is_path_like(value: &str) -> bool {
    is_path_prefix(value) || (has_known_path_extension(value) && !value.contains(char::is_whitespace))
}

fn is_path_prefix(value: &str) -> bool {
    value.starts_with("\\\\")
        || value.starts_with(".\\")
        || value.starts_with("./")
        || (value.len() >= 2
            && value.as_bytes()[0].is_ascii_alphabetic()
            && value.as_bytes()[1] == b':')
        || value.contains(['\\', '/'])
}

fn find_existing_path_end(cwd: &Path, input: &str) -> Option<usize> {
    input
        .char_indices()
        .filter_map(|(index, character)| character.is_whitespace().then_some(index))
        .filter(|index| path_is_existing_file(cwd, &input[..*index]))
        .next()
}

fn path_is_existing_file(cwd: &Path, value: &str) -> bool {
    let path = Path::new(value);
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    };
    std::fs::metadata(resolved).is_ok_and(|metadata| metadata.is_file())
}

fn find_path_extension_end(input: &str) -> Option<usize> {
    const EXTENSIONS: [&str; 5] = [".exe", ".com", ".bat", ".cmd", ".ps1"];

    for (index, _) in input.char_indices() {
        let suffix = &input[index..];
        for extension in EXTENSIONS {
            let Some(candidate) = suffix.get(..extension.len()) else {
                continue;
            };
            if !candidate.eq_ignore_ascii_case(extension) {
                continue;
            }

            let candidate_end = index + extension.len();
            if candidate_end == input.len()
                || input[candidate_end..]
                    .chars()
                    .next()
                    .is_some_and(char::is_whitespace)
            {
                return Some(candidate_end);
            }
        }
    }
    None
}

fn has_known_path_extension(value: &str) -> bool {
    [".exe", ".com", ".bat", ".cmd", ".ps1"]
        .iter()
        .any(|extension| {
            value
                .get(value.len().saturating_sub(extension.len())..)
                .is_some_and(|suffix| suffix.eq_ignore_ascii_case(extension))
        })
}

/// 在编辑器中打开的计划（D6）：接受完整 `EditorProfile` 并替换唯一 `{path}`。
pub fn editor_plan(profile: &EditorProfile, path: &Path) -> Result<LaunchPlan, LaunchError> {
    let executable = profile.executable.trim();
    if executable.is_empty() {
        return Err(LaunchError::EditorNotConfigured);
    }

    let substituted_args = substitute_editor_path(&profile.arguments, path)?;
    if is_batch_executable(executable) {
        validate_batch_arguments(&substituted_args)?;
        return Ok(build_cmd_script_plan(
            executable,
            &substituted_args,
            path,
        ));
    }

    Ok(LaunchPlan {
        program: executable.to_string(),
        args: substituted_args,
        cwd: path.to_path_buf(),
        env: Vec::new(),
    })
}

/// 打开目录的计划（Open）：`explorer <path>`。
pub fn open_dir_plan(path: &Path) -> LaunchPlan {
    LaunchPlan {
        program: "explorer".to_string(),
        args: vec![path.display().to_string()],
        cwd: path.to_path_buf(),
        env: Vec::new(),
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

fn substitute_editor_path(arguments: &[String], path: &Path) -> Result<Vec<String>, LaunchError> {
    const PLACEHOLDER: &str = "{path}";

    let placeholder_count: usize = arguments
        .iter()
        .map(|arg| arg.matches(PLACEHOLDER).count())
        .sum();
    if placeholder_count != 1 {
        return Err(LaunchError::InvalidEditorProfile {
            detail: "editor.arguments must contain exactly one {path} placeholder when editor.executable is configured".to_string(),
        });
    }

    let path_string = path.display().to_string();
    Ok(arguments
        .iter()
        .map(|arg| arg.replacen(PLACEHOLDER, &path_string, 1))
        .collect())
}

fn validate_batch_arguments(arguments: &[String]) -> Result<(), LaunchError> {
    if arguments.iter().any(|argument| argument.contains('"')) {
        return Err(LaunchError::InvalidEditorProfile {
            detail: "cmd.exe batch arguments cannot contain the double quote character".to_string(),
        });
    }
    Ok(())
}

fn build_cmd_script_plan(executable: &str, arguments: &[String], cwd: &Path) -> LaunchPlan {
    const EXECUTABLE_ENV: &str = "WINDY_EDITOR_EXECUTABLE";
    const ARGUMENT_ENV_PREFIX: &str = "WINDY_EDITOR_ARGUMENT_";

    let mut env = vec![(EXECUTABLE_ENV.to_string(), executable.to_string())];
    for (index, argument) in arguments.iter().enumerate() {
        env.push((
            format!("{ARGUMENT_ENV_PREFIX}{index}"),
            argument.to_string(),
        ));
    }

    let mut command = format!("\"\"%{EXECUTABLE_ENV}%\"");
    for index in 0..arguments.len() {
        command.push_str(&format!(" \"%{ARGUMENT_ENV_PREFIX}{index}%\""));
    }
    command.push('"');

    LaunchPlan {
        program: "cmd.exe".to_string(),
        args: vec![
            "/d".to_string(),
            "/s".to_string(),
            "/c".to_string(),
            command,
        ],
        cwd: cwd.to_path_buf(),
        env,
    }
}

/// 执行计划：detached 拉起进程并立即返回（不等待、不采集输出）。
pub fn spawn_plan(plan: &LaunchPlan) -> Result<(), LaunchError> {
    let mut command = std::process::Command::new(&plan.program);
    command.current_dir(&plan.cwd);
    #[cfg(windows)]
    if plan.program.eq_ignore_ascii_case("cmd.exe") && !plan.env.is_empty() {
        // `/c` 的命令文本含有固定引号；raw_arg 避免 Rust 为嵌入引号添加反斜杠。
        command.args(&plan.args[..3]).raw_arg(&plan.args[3]);
    } else {
        command.args(&plan.args);
    }
    #[cfg(not(windows))]
    command.args(&plan.args);
    command.envs(plan.env.iter().map(|(name, value)| (name, value)));
    command
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
pub fn open_in_editor(profile: &EditorProfile, path: &Path) -> Result<(), LaunchError> {
    let plan = editor_plan(profile, path)?;
    require_dir(path)?;
    spawn_plan(&plan)
}
