//! 子进程捕获与 `where.exe` 输出分类（B1：自 `commands/system.rs` 拆出）。

use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
pub(crate) const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommandCapture {
    pub(crate) status_code: Option<i32>,
    pub(crate) stdout: String,
    pub(crate) stderr: String,
}

pub fn command_error_message(program: &str, error: &std::io::Error) -> String {
    format!("{program} failed: {error}")
}

pub fn classify_where_output(
    status_code: Option<i32>,
    stdout: &str,
    stderr: &str,
) -> Result<String, String> {
    match status_code {
        Some(0) => Ok(stdout.to_string()),
        Some(1) => Ok(String::new()),
        _ => Err(command_exit_message(
            "where.exe",
            status_code,
            stdout,
            stderr,
        )),
    }
}

pub(crate) fn run_where(alias: &str) -> Result<String, String> {
    let capture = run_command_capture("where.exe", &[alias.to_string()])?;
    classify_where_output(capture.status_code, &capture.stdout, &capture.stderr)
}

pub(crate) fn run_command_capture(
    program: &str,
    args: &[String],
) -> Result<CommandCapture, String> {
    let mut command = Command::new(program);
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let output = command
        .args(args)
        .output()
        .map_err(|error| command_error_message(program, &error))?;
    Ok(CommandCapture {
        status_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

pub(crate) fn command_exit_message(
    program: &str,
    status_code: Option<i32>,
    stdout: &str,
    stderr: &str,
) -> String {
    let stderr_trimmed = stderr.trim();
    let stdout_trimmed = stdout.trim();
    let detail = if !stderr_trimmed.is_empty() {
        stderr_trimmed
    } else {
        stdout_trimmed
    };

    if !detail.is_empty() {
        return format!("{program} failed: {detail}");
    }

    match status_code {
        Some(code) => format!("{program} exited with status {code}"),
        None => format!("{program} terminated without an exit code"),
    }
}

#[cfg(test)]
mod tests {
    #[cfg(windows)]
    #[test]
    fn system_child_processes_use_the_no_window_creation_flag() {
        assert_eq!(super::CREATE_NO_WINDOW, 0x0800_0000);
    }
}
