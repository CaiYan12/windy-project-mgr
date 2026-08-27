//! 分离式启动集成测试（D4 / ADR 0002，D12：真实临时目录 + 真实进程）。
//!
//! 计划构造函数为纯函数直接断言；`spawn_plan` / `run_in_terminal_with` /
//! `open_in_editor` 用真实可执行文件（`hostname`：忽略参数、立即退出，
//! 不残留进程）与不存在的二进制名验证成功拉起、拉起失败与 wt→powershell
//! 回退路径。

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use windy_project_mgr_lib::launch::{
    editor_plan, open_dir_plan, open_in_editor, ps_plan, run_in_terminal_with, spawn_plan,
    wt_plan, LaunchError, LaunchPlan,
};
use windy_project_mgr_lib::project::settings::EditorProfile;

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p10-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

fn editor_profile(executable: &str, arguments: &[&str]) -> EditorProfile {
    EditorProfile {
        executable: executable.to_string(),
        arguments: arguments.iter().map(|arg| arg.to_string()).collect(),
    }
}

// ---------- 计划构造（纯函数） ----------

#[test]
fn pwsh_quote_keeps_plain_commands_unchanged() {
    // 程序 + 参数（含空格）不套 & / 引号，避免破坏 pnpm dev 这种写法。
    assert_eq!(windy_project_mgr_lib::launch::quote_pwsh_command("pnpm dev"), "pnpm dev");
    assert_eq!(
        windy_project_mgr_lib::launch::quote_pwsh_command("cargo build --release"),
        "cargo build --release"
    );
}

#[test]
fn pwsh_quote_wraps_path_commands_with_call_operator() {
    // 路径形态（盘符/UNC/脚本扩展名）→ PS 调用运算符 + 单引号，保证含空格路径可执行。
    assert_eq!(
        windy_project_mgr_lib::launch::quote_pwsh_command(
            r"C:\Users\Einn Tzai\AppData\Local\Temp\windy-accept\node-app\start-dev.bat"
        ),
        r#"& 'C:\Users\Einn Tzai\AppData\Local\Temp\windy-accept\node-app\start-dev.bat'"#
    );
    assert_eq!(
        windy_project_mgr_lib::launch::quote_pwsh_command(r"\\server\share\run.cmd"),
        r#"& '\\server\share\run.cmd'"#
    );
    assert_eq!(
        windy_project_mgr_lib::launch::quote_pwsh_command(r".\scripts\go.ps1"),
        r#"& '.\scripts\go.ps1'"#
    );
}

#[test]
fn pwsh_quote_escapes_internal_single_quotes() {
    // 内部单引号按 PS 规则翻倍（''），避免破坏字面量。
    assert_eq!(
        windy_project_mgr_lib::launch::quote_pwsh_command(r"C:\User's dir\a.bat"),
        r#"& 'C:\User''s dir\a.bat'"#
    );
}

#[test]
fn pwsh_quote_keeps_path_and_arguments_separate() {
    let command = r#"C:\Program Files\Windy Tool\tool.exe --name "value with spaces" --literal %PATH%"#;
    assert_eq!(
        windy_project_mgr_lib::launch::quote_pwsh_command(command),
        r#"& 'C:\Program Files\Windy Tool\tool.exe' --name "value with spaces" --literal %PATH%"#
    );
}

#[test]
fn pwsh_quote_wraps_bare_relative_script_path_with_arguments() {
    assert_eq!(
        windy_project_mgr_lib::launch::quote_pwsh_command(
            r#"scripts\Program Files\run.ps1 --flag value"#
        ),
        r#"& 'scripts\Program Files\run.ps1' --flag value"#
    );
}

#[test]
fn pwsh_quote_does_not_split_unverifiable_extensionless_absolute_path() {
    let command = r#"C:\Program Files\extensionless tool --flag value"#;
    assert_eq!(
        windy_project_mgr_lib::launch::quote_pwsh_command(command),
        command
    );
}

#[test]
fn ps_plan_wraps_existing_extensionless_absolute_path_as_one_executable() {
    let fixture = temp_dir("pwsh-extensionless");
    let executable = fixture.join("Program Files").join("extensionless tool");
    std::fs::create_dir_all(executable.parent().expect("parent")).expect("create parent");
    std::fs::write(&executable, b"not executed").expect("create extensionless file");

    let command = format!(r#"{} --flag value"#, executable.display());
    let plan = ps_plan(&fixture, &command, "powershell");
    assert_eq!(
        plan.args[2],
        format!(r#"& '{}' --flag value"#, executable.display())
    );
    cleanup(&fixture);
}

#[test]
fn ps_and_wt_plans_wrap_existing_extensionless_path_without_arguments() {
    let fixture = temp_dir("pwsh-extensionless-no-args");
    let executable = fixture.join("Program Files").join("extensionless tool");
    std::fs::create_dir_all(executable.parent().expect("parent")).expect("create parent");
    std::fs::write(&executable, b"not executed").expect("create extensionless file");

    let command = executable.display().to_string();
    let expected = format!(r#"& '{}'"#, executable.display());
    assert_eq!(ps_plan(&fixture, &command, "powershell").args[2], expected);
    assert_eq!(
        wt_plan(&fixture, &command, "wt", "powershell").args[5],
        expected
    );
    cleanup(&fixture);
}

#[test]
fn ps_plan_wraps_existing_bare_extensionless_file_without_arguments() {
    let fixture = temp_dir("pwsh-bare-extensionless-no-args");
    std::fs::write(fixture.join("tool"), b"not executed").expect("create extensionless file");

    let plan = ps_plan(&fixture, "tool", "powershell");
    assert_eq!(plan.args[2], r#"& 'tool'"#);
    cleanup(&fixture);
}

#[test]
fn ps_plan_wraps_existing_bare_extensionless_file_with_arguments() {
    let fixture = temp_dir("pwsh-bare-extensionless-with-args");
    std::fs::write(fixture.join("tool"), b"not executed").expect("create extensionless file");

    let plan = ps_plan(&fixture, "tool --flag value", "powershell");
    assert_eq!(plan.args[2], r#"& 'tool' --flag value"#);
    cleanup(&fixture);
}

#[test]
fn wt_plan_shape() {
    let cwd = Path::new(r"C:\dev\demo");
    let plan = wt_plan(cwd, "pnpm dev", "wt", "powershell");
    assert_eq!(plan.program, "wt");
    assert_eq!(
        plan.args,
        vec!["-d", r"C:\dev\demo", "powershell", "-NoExit", "-Command", "pnpm dev"]
    );
    assert_eq!(plan.cwd, cwd);
}

#[test]
fn ps_plan_shape() {
    let cwd = Path::new(r"C:\dev\demo");
    let plan = ps_plan(cwd, "cargo build", "powershell");
    assert_eq!(plan.program, "powershell");
    assert_eq!(plan.args, vec!["-NoExit", "-Command", "cargo build"]);
    assert_eq!(plan.cwd, cwd);
}

#[test]
fn editor_plan_shape() {
    let path = Path::new(r"C:\dev\demo folder");
    let plan = editor_plan(&editor_profile("code", &["--reuse-window", "{path}"]), path)
        .expect("configured editor");
    assert_eq!(plan.program, "code");
    assert_eq!(plan.args, vec!["--reuse-window", r"C:\dev\demo folder"]);
    assert_eq!(plan.cwd, path);
}

#[test]
fn editor_plan_empty_command_is_not_configured() {
    match editor_plan(&editor_profile("", &["{path}"]), Path::new(r"C:\dev\demo")) {
        Err(LaunchError::EditorNotConfigured) => {}
        other => panic!("expected EditorNotConfigured, got {other:?}"),
    }
    match editor_plan(&editor_profile("   ", &["{path}"]), Path::new(r"C:\dev\demo")) {
        Err(LaunchError::EditorNotConfigured) => {}
        other => panic!("expected EditorNotConfigured for blanks, got {other:?}"),
    }
}

#[test]
fn editor_plan_replaces_placeholder_inside_single_argument() {
    let path = Path::new(r"C:\dev\demo folder");
    let plan = editor_plan(
        &editor_profile("code", &["--folder={path}", "--wait"]),
        path,
    )
    .expect("configured editor");
    assert_eq!(plan.program, "code");
    assert_eq!(plan.args, vec![r"--folder=C:\dev\demo folder", "--wait"]);
}

#[test]
fn editor_plan_allows_double_quotes_for_direct_executables() {
    let path = Path::new(r"C:\dev\demo folder");
    let plan = editor_plan(
        &editor_profile("code.exe", &[r#"--title="hello world""#, "{path}"]),
        path,
    )
    .expect("direct executable arguments may contain double quotes");
    assert_eq!(plan.program, "code.exe");
    assert_eq!(plan.args, vec![r#"--title="hello world""#, r"C:\dev\demo folder"]);
}

#[test]
fn editor_plan_rejects_missing_placeholder() {
    match editor_plan(
        &editor_profile("code", &["--reuse-window"]),
        Path::new(r"C:\dev\demo"),
    ) {
        Err(LaunchError::InvalidEditorProfile { detail }) => {
            assert!(
                detail.contains("exactly one {path}"),
                "unexpected detail: {detail}"
            );
        }
        other => panic!("expected InvalidEditorProfile, got {other:?}"),
    }
}

#[test]
fn editor_plan_rejects_duplicate_placeholder() {
    match editor_plan(
        &editor_profile("code", &["{path}", "--other={path}"]),
        Path::new(r"C:\dev\demo"),
    ) {
        Err(LaunchError::InvalidEditorProfile { detail }) => {
            assert!(
                detail.contains("exactly one {path}"),
                "unexpected detail: {detail}"
            );
        }
        other => panic!("expected InvalidEditorProfile, got {other:?}"),
    }
}

#[test]
fn editor_plan_uses_cmd_for_bat_executables() {
    let path = Path::new(r"C:\dev\demo folder");
    let plan = editor_plan(
        &editor_profile(
            r"C:\Tools\Open In Editor.BAT",
            &["--flag", "{path}"],
        ),
        path,
    )
    .expect("bat plan");
    assert_eq!(plan.program, "cmd.exe");
    assert_eq!(
        plan.args,
        vec![
            "/d",
            "/s",
            "/c",
            "\"\"%WINDY_EDITOR_EXECUTABLE%\" \"%WINDY_EDITOR_ARGUMENT_0%\" \"%WINDY_EDITOR_ARGUMENT_1%\"\"",
        ]
    );
    assert_eq!(
        plan.env,
        vec![
            (
                "WINDY_EDITOR_EXECUTABLE".to_string(),
                r#"C:\Tools\Open In Editor.BAT"#.to_string(),
            ),
            ("WINDY_EDITOR_ARGUMENT_0".to_string(), "--flag".to_string()),
            (
                "WINDY_EDITOR_ARGUMENT_1".to_string(),
                r#"C:\dev\demo folder"#.to_string(),
            ),
        ]
    );
    assert_eq!(plan.cwd, path);
}

#[test]
fn editor_plan_uses_cmd_for_cmd_executables() {
    let path = Path::new(r"C:\dev\demo folder");
    let plan = editor_plan(
        &editor_profile("open-editor.cmd", &["{path}"]),
        path,
    )
    .expect("cmd plan");
    assert_eq!(plan.program, "cmd.exe");
    assert_eq!(
        plan.args,
        vec![
            "/d",
            "/s",
            "/c",
            "\"\"%WINDY_EDITOR_EXECUTABLE%\" \"%WINDY_EDITOR_ARGUMENT_0%\"\"",
        ]
    );
    assert_eq!(
        plan.env,
        vec![
            (
                "WINDY_EDITOR_EXECUTABLE".to_string(),
                "open-editor.cmd".to_string(),
            ),
            (
                "WINDY_EDITOR_ARGUMENT_0".to_string(),
                r#"C:\dev\demo folder"#.to_string(),
            ),
        ]
    );
    assert_eq!(plan.cwd, path);
}

#[test]
fn editor_plan_uses_cmd_for_mixed_case_script_extensions() {
    let path = Path::new(r"C:\dev\demo folder");
    for executable in [r"C:\Tools\Open In Editor.Cmd", r"C:\Tools\Open In Editor.BaT"] {
        let plan = editor_plan(&editor_profile(executable, &["{path}"]), path)
            .expect("mixed-case script plan");
        assert_eq!(plan.program, "cmd.exe", "executable={executable}");
    }
}

#[test]
fn cmd_script_preserves_percent_ampersand_bang_and_spaces() {
    let fixture = temp_dir("cmd-percent");
    let project = fixture.join("project %PATH% & ! folder");
    std::fs::create_dir_all(&project).expect("create project dir");
    let script = fixture.join("capture.cmd");
    std::fs::write(
        &script,
        b"@echo off\r\nsetlocal DisableDelayedExpansion\r\nset \"arg1=%~1\"\r\nset \"arg2=%~2\"\r\n> \"%~dp0received.txt\" (\r\n  set arg1\r\n  set arg2\r\n)\r\n",
    )
    .expect("write cmd fixture");

    let profile = editor_profile(
        &script.display().to_string(),
        &["--literal=%PATH% & ! value", "{path}"],
    );
    let plan = editor_plan(&profile, &project).expect("cmd plan");
    spawn_plan(&plan).expect("cmd fixture must spawn");

    let output_path = fixture.join("received.txt");
    let deadline = Instant::now() + Duration::from_secs(3);
    let output = loop {
        if let Ok(output) = std::fs::read_to_string(&output_path) {
            break output;
        }
        assert!(Instant::now() < deadline, "cmd fixture did not write output");
        thread::sleep(Duration::from_millis(50));
    };
    assert_eq!(
        output.replace("\r\n", "\n"),
        format!(
            "arg1=--literal=%PATH% & ! value\narg2={}\n",
            project.display()
        )
    );
    cleanup(&fixture);
}

#[test]
fn cmd_script_rejects_arguments_with_double_quotes() {
    let fixture = temp_dir("cmd-injection");
    let project = fixture.join("project");
    std::fs::create_dir_all(&project).expect("create project dir");
    let profile = editor_profile(
        "open-editor.BaT",
        &[r#"safe" & echo INJECTED > INJECTED.txt & "tail"#, "{path}"],
    );
    match editor_plan(&profile, &project) {
        Err(LaunchError::InvalidEditorProfile { detail }) => assert_eq!(
            detail,
            "cmd.exe batch arguments cannot contain the double quote character"
        ),
        other => panic!("expected InvalidEditorProfile, got {other:?}"),
    }
    cleanup(&fixture);
}

#[test]
fn cmd_script_plan_preserves_environment_argument_boundaries() {
    let fixture = temp_dir("cmd-quoted-arguments");
    let project = fixture.join("project %PATH% & ! folder");
    std::fs::create_dir_all(&project).expect("create project dir");
    let script = fixture.join("capture.cmd");
    std::fs::write(
        &script,
        b"@echo off\r\nsetlocal DisableDelayedExpansion\r\nset \"arg1=%~1\"\r\nset \"arg2=%~2\"\r\nset \"arg3=%~3\"\r\n> \"%~dp0received.txt\" (\r\n  set arg1\r\n  set arg2\r\n  set arg3\r\n)\r\n",
    )
    .expect("write cmd fixture");

    let profile = editor_profile(
        &script.display().to_string(),
        &[
            "--title=hello ! world & %PATH%",
            "--literal=%PATH%",
            "{path}",
        ],
    );
    let plan = editor_plan(&profile, &project).expect("cmd plan");
    spawn_plan(&plan).expect("cmd fixture must spawn");

    let output_path = fixture.join("received.txt");
    let deadline = Instant::now() + Duration::from_secs(3);
    let output = loop {
        if let Ok(output) = std::fs::read_to_string(&output_path) {
            break output;
        }
        assert!(Instant::now() < deadline, "cmd fixture did not write output");
        thread::sleep(Duration::from_millis(50));
    };
    assert!(
        output.contains("arg1=--title=hello ! world & %PATH%"),
        "first argument was truncated: {output:?}"
    );
    assert!(
        output.contains("arg2=--literal=%PATH%\r\n"),
        "second argument was not preserved: {output:?}"
    );
    assert!(
        output.contains(&format!("arg3={}\r\n", project.display())),
        "third argument was not preserved: {output:?}"
    );
    cleanup(&fixture);
}

#[cfg(windows)]
#[test]
fn cmd_delayed_expansion_probe_does_not_execute_shell_metacharacters() {
    let fixture = temp_dir("cmd-delayed-expansion-probe");
    let project = fixture.join("project");
    std::fs::create_dir_all(&project).expect("create project dir");
    let script = fixture.join("noop.cmd");
    std::fs::write(
        &script,
        b"@echo off\r\n> \"%~dp0started.txt\" type nul\r\n",
    )
    .expect("write cmd fixture");

    let command_text = r#"""!WINDY_EDITOR_EXECUTABLE!" "!WINDY_EDITOR_ARGUMENT_0!" "!WINDY_EDITOR_ARGUMENT_1!""#;
    let mut command = std::process::Command::new("cmd.exe");
    command
        .current_dir(&project)
        .args(["/d", "/v:on", "/s", "/c"])
        .raw_arg(command_text)
        .env("WINDY_EDITOR_EXECUTABLE", &script)
        .env(
            "WINDY_EDITOR_ARGUMENT_0",
            r#"safe" & echo INJECTED > INJECTED.txt & ! %PATH% "tail"#,
        )
        .env("WINDY_EDITOR_ARGUMENT_1", &project);
    let mut child = command.spawn().expect("cmd probe must spawn");
    let status = child.wait().expect("cmd probe must exit");
    assert!(status.success(), "cmd probe failed: {status}");
    assert!(
        !project.join("INJECTED.txt").exists(),
        "delayed expansion probe executed injected shell text"
    );
    assert!(fixture.join("started.txt").is_file(), "cmd fixture did not run");
    cleanup(&fixture);
}

#[cfg(windows)]
#[test]
fn cmd_quoting_matrix_probe() {
    let fixture = temp_dir("cmd-quoting-matrix");
    let project = fixture.join("project folder");
    std::fs::create_dir_all(&project).expect("create project dir");
    let script = fixture.join("capture.cmd");
    std::fs::write(
        &script,
        b"@echo off\r\nsetlocal DisableDelayedExpansion\r\nset \"arg1=%1\"\r\nset \"arg2=%2\"\r\n> \"%~dp0received.txt\" (\r\n  set arg1\r\n  set arg2\r\n)\r\n",
    )
    .expect("write cmd fixture");

    let variants = [
        (
            "plain-delayed",
            r#"""!WINDY_EDITOR_EXECUTABLE!" "!WINDY_EDITOR_ARGUMENT_0!" "!WINDY_EDITOR_ARGUMENT_1!""#,
            r#"--title="hello world"#,
        ),
        (
            "caret-delayed",
            r#"""!WINDY_EDITOR_EXECUTABLE!" ^"!WINDY_EDITOR_ARGUMENT_0!^" ^"!WINDY_EDITOR_ARGUMENT_1!^""#,
            r#"--title="hello world"#,
        ),
        (
            "plain-delayed-no-wrapper",
            r#"""!WINDY_EDITOR_EXECUTABLE!" !WINDY_EDITOR_ARGUMENT_0! !WINDY_EDITOR_ARGUMENT_1!""#,
            r#"--title="hello world"#,
        ),
        (
            "caret-in-value",
            r#"""!WINDY_EDITOR_EXECUTABLE!" "!WINDY_EDITOR_ARGUMENT_0!" "!WINDY_EDITOR_ARGUMENT_1!""#,
            r#"--title=^"hello world^"#,
        ),
        (
            "percent-plain",
            r#"""%WINDY_EDITOR_EXECUTABLE%" "%WINDY_EDITOR_ARGUMENT_0%" "%WINDY_EDITOR_ARGUMENT_1%""#,
            r#"--title="hello world"#,
        ),
        (
            "percent-caret-wrapper",
            r#"""%WINDY_EDITOR_EXECUTABLE%" ^"%WINDY_EDITOR_ARGUMENT_0%^" ^"%WINDY_EDITOR_ARGUMENT_1%^""#,
            r#"--title="hello world"#,
        ),
        (
            "percent-value-caret",
            r#"""%WINDY_EDITOR_EXECUTABLE%" "%WINDY_EDITOR_ARGUMENT_0%" "%WINDY_EDITOR_ARGUMENT_1%""#,
            r#"--title=^"hello world^"#,
        ),
        (
            "delayed-doubled-value-quotes",
            r#"""!WINDY_EDITOR_EXECUTABLE!" "!WINDY_EDITOR_ARGUMENT_0!" "!WINDY_EDITOR_ARGUMENT_1!""#,
            r#"--title=""hello world""#,
        ),
        (
            "delayed-backslash-value-quotes",
            r#"""!WINDY_EDITOR_EXECUTABLE!" "!WINDY_EDITOR_ARGUMENT_0!" "!WINDY_EDITOR_ARGUMENT_1!""#,
            r#"--title=\"hello world\""#,
        ),
        (
            "literal-caret-quotes",
            r#"""!WINDY_EDITOR_EXECUTABLE!" --title^="hello world" "!WINDY_EDITOR_ARGUMENT_1!""#,
            r#"unused"#,
        ),
        (
            "literal-caret-all",
            r#"""!WINDY_EDITOR_EXECUTABLE!" ^"--title=^"hello world^"^" "!WINDY_EDITOR_ARGUMENT_1!""#,
            r#"unused"#,
        ),
        (
            "literal-escaped-inner-quotes",
            r#"""!WINDY_EDITOR_EXECUTABLE!" --title=^"hello world^" "!WINDY_EDITOR_ARGUMENT_1!""#,
            r#"unused"#,
        ),
        (
            "literal-grouped-escaped-inner-quotes",
            r#"""!WINDY_EDITOR_EXECUTABLE!" "--title=^"hello world^"" "!WINDY_EDITOR_ARGUMENT_1!""#,
            r#"unused"#,
        ),
    ];

    for (label, command_text, argument) in variants {
        let output_path = fixture.join(format!("{label}.txt"));
        let mut command = std::process::Command::new("cmd.exe");
        command
            .current_dir(&project)
            .args(["/d", "/v:on", "/s", "/c"])
            .raw_arg(command_text)
            .env("WINDY_EDITOR_EXECUTABLE", &script)
            .env("WINDY_EDITOR_ARGUMENT_0", argument)
            .env("WINDY_EDITOR_ARGUMENT_1", &project)
            .env("WINDY_CAPTURE_OUTPUT", &output_path);
        std::fs::write(
            &script,
            b"@echo off\r\nsetlocal DisableDelayedExpansion\r\nset \"arg1=%1\"\r\nset \"arg2=%2\"\r\n> \"%~dp0received.txt\" (\r\n  set arg1\r\n  set arg2\r\n)\r\n",
        )
        .expect("reset cmd fixture");
        let status = command.spawn().expect("cmd matrix spawn").wait().expect("cmd matrix exit");
        assert!(status.success(), "{label} failed: {status}");
        let output = std::fs::read_to_string(fixture.join("received.txt")).unwrap_or_default();
        std::fs::write(&output_path, output).expect("save matrix output");
    }

    for (label, _, _) in variants {
        let output = std::fs::read_to_string(fixture.join(format!("{label}.txt")))
            .expect("matrix output");
        eprintln!("{label}: {output:?}");
    }
    cleanup(&fixture);
}

#[test]
fn open_dir_plan_shape() {
    let path = Path::new(r"C:\dev\demo");
    let plan = open_dir_plan(path);
    assert_eq!(plan.program, "explorer");
    assert_eq!(plan.args, vec![r"C:\dev\demo"]);
    assert_eq!(plan.cwd, path);
}

// ---------- 真实进程拉起 ----------

#[test]
fn spawn_plan_launches_real_process() {
    let dir = temp_dir("spawn");
    let plan = LaunchPlan {
        program: "hostname".to_string(),
        args: vec!["ignored".to_string()],
        cwd: dir.clone(),
        env: Vec::new(),
    };
    spawn_plan(&plan).expect("hostname must spawn");
    cleanup(&dir);
}

#[test]
fn spawn_plan_missing_binary_is_launch_failed() {
    let dir = temp_dir("spawn-missing");
    let plan = LaunchPlan {
        program: "windy-no-such-binary-xyz".to_string(),
        args: vec![],
        cwd: dir.clone(),
        env: Vec::new(),
    };
    match spawn_plan(&plan) {
        Err(LaunchError::LaunchFailed { detail }) => {
            assert!(detail.contains("windy-no-such-binary-xyz"), "got: {detail}");
        }
        other => panic!("expected LaunchFailed, got {other:?}"),
    }
    cleanup(&dir);
}

#[test]
fn run_in_terminal_prefers_wt_and_falls_back() {
    let dir = temp_dir("run-fallback");
    // wt 二进制不存在 → 回退到真实存在的 hostname（充当终端替身，立即退出）。
    run_in_terminal_with(&dir, "echo hello", "windy-no-wt-xyz", "hostname")
        .expect("fallback must succeed");
    cleanup(&dir);
}

#[test]
fn run_in_terminal_with_wt_present_succeeds() {
    let dir = temp_dir("run-wt");
    // 注入 hostname 充当 wt：优先路径直接成功。
    run_in_terminal_with(&dir, "echo hello", "hostname", "powershell")
        .expect("wt path must succeed");
    cleanup(&dir);
}

#[test]
fn run_in_terminal_missing_path_is_diagnosable() {
    let missing = std::env::temp_dir().join("windy-p10-does-not-exist");
    match run_in_terminal_with(&missing, "echo hello", "hostname", "hostname") {
        Err(LaunchError::PathNotFound { .. }) => {}
        other => panic!("expected PathNotFound, got {other:?}"),
    }
}

#[test]
fn run_in_terminal_empty_command_is_diagnosable() {
    let dir = temp_dir("run-empty");
    for cmd in ["", "   "] {
        match run_in_terminal_with(&dir, cmd, "hostname", "hostname") {
            Err(LaunchError::EmptyCommand) => {}
            other => panic!("expected EmptyCommand for {cmd:?}, got {other:?}"),
        }
    }
    cleanup(&dir);
}

#[test]
fn run_in_terminal_both_terminals_missing_is_launch_failed() {
    let dir = temp_dir("run-both-missing");
    match run_in_terminal_with(&dir, "echo hello", "windy-no-wt-xyz", "windy-no-ps-xyz") {
        Err(LaunchError::LaunchFailed { .. }) => {}
        other => panic!("expected LaunchFailed, got {other:?}"),
    }
    cleanup(&dir);
}

#[test]
fn open_in_editor_launches_configured_editor() {
    let dir = temp_dir("editor");
    // 注入 hostname 充当编辑器：真实拉起成功且立即退出。
    open_in_editor(&editor_profile("hostname", &["{path}"]), &dir)
        .expect("editor launch must succeed");
    cleanup(&dir);
}

#[test]
fn open_in_editor_not_configured_before_path_check() {
    // 未配置优先于路径检查报错（D6 引导语义）。
    let missing = std::env::temp_dir().join("windy-p10-does-not-exist");
    match open_in_editor(&editor_profile("", &["{path}"]), &missing) {
        Err(LaunchError::EditorNotConfigured) => {}
        other => panic!("expected EditorNotConfigured, got {other:?}"),
    }
}

#[test]
fn open_in_editor_missing_path_is_diagnosable() {
    let missing = std::env::temp_dir().join("windy-p10-does-not-exist");
    match open_in_editor(&editor_profile("hostname", &["{path}"]), &missing) {
        Err(LaunchError::PathNotFound { .. }) => {}
        other => panic!("expected PathNotFound, got {other:?}"),
    }
}

#[test]
fn open_in_editor_existing_file_path_is_diagnosable() {
    let dir = temp_dir("editor-file");
    let file = dir.join("project.txt");
    std::fs::write(&file, "not a directory").expect("write temp file");
    match open_in_editor(&editor_profile("hostname", &["{path}"]), &file) {
        Err(LaunchError::PathNotFound { path }) => assert_eq!(path, file),
        other => panic!("expected PathNotFound, got {other:?}"),
    }
    cleanup(&dir);
}
