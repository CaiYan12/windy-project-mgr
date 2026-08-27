//! System discovery and app info commands for settings v2.

use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const UNINSTALL_ROOTS: [&str; 3] = [
    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall",
    r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall",
    r"HKLM\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
];

const PATH_ALIASES: [&str; 6] = [
    "code",
    "code-insiders",
    "cursor",
    "windsurf",
    "codium",
    "zed",
];
const WINDOWS_ONLY_ERROR: &str = "system discovery is only supported on Windows";
const WINDOWS_ACCENT_NOT_FOUND: &str = "Windows accent color was not found in the registry";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DetectionSource {
    Path,
    Registry,
    Standard,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedEditor {
    pub id: String,
    pub name: String,
    pub executable: String,
    pub source: DetectionSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub data_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryQueryBlock {
    pub key_path: String,
    pub values: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandCapture {
    status_code: Option<i32>,
    stdout: String,
    stderr: String,
}

#[derive(Clone, Copy)]
struct EditorProduct {
    id: &'static str,
    name: &'static str,
    executable_names: &'static [&'static str],
    display_markers: &'static [&'static str],
    forbidden_markers: &'static [&'static str],
    key_markers: &'static [&'static str],
    standard_dirs: &'static [&'static str],
}

const PRODUCTS: [EditorProduct; 6] = [
    EditorProduct {
        id: "vscode",
        name: "Visual Studio Code",
        executable_names: &["Code.exe"],
        display_markers: &["visual studio code", "microsoft visual studio code"],
        forbidden_markers: &["insiders"],
        key_markers: &["vscode", "visual studio code"],
        standard_dirs: &["Microsoft VS Code"],
    },
    EditorProduct {
        id: "vscode-insiders",
        name: "Visual Studio Code Insiders",
        executable_names: &["Code - Insiders.exe"],
        display_markers: &[
            "visual studio code insiders",
            "microsoft visual studio code insiders",
        ],
        forbidden_markers: &[],
        key_markers: &["vscodeinsiders", "visual studio code insiders"],
        standard_dirs: &["Microsoft VS Code Insiders"],
    },
    EditorProduct {
        id: "cursor",
        name: "Cursor",
        executable_names: &["Cursor.exe"],
        display_markers: &["cursor"],
        forbidden_markers: &[],
        key_markers: &["cursor"],
        standard_dirs: &["Cursor"],
    },
    EditorProduct {
        id: "windsurf",
        name: "Windsurf",
        executable_names: &["Windsurf.exe"],
        display_markers: &["windsurf"],
        forbidden_markers: &[],
        key_markers: &["windsurf"],
        standard_dirs: &["Windsurf"],
    },
    EditorProduct {
        id: "vscodium",
        name: "VSCodium",
        executable_names: &["VSCodium.exe"],
        display_markers: &["vscodium"],
        forbidden_markers: &[],
        key_markers: &["vscodium"],
        standard_dirs: &["VSCodium"],
    },
    EditorProduct {
        id: "zed",
        name: "Zed",
        executable_names: &["Zed.exe"],
        display_markers: &["zed"],
        forbidden_markers: &[],
        key_markers: &["zed"],
        standard_dirs: &["Zed"],
    },
];

pub fn get_app_info_in(data_dir: &Path) -> Result<AppInfo, String> {
    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        data_dir: strip_verbatim_prefix(&data_dir.display().to_string()),
    })
}

pub fn parse_registry_query_blocks(raw: &str) -> Vec<RegistryQueryBlock> {
    let mut blocks = Vec::new();
    let mut current_key: Option<String> = None;
    let mut current_values = BTreeMap::new();

    for line in raw.lines() {
        let trimmed_end = line.trim_end();
        if trimmed_end.trim().is_empty() {
            continue;
        }

        let starts_new_key = !trimmed_end.starts_with(' ') && !trimmed_end.starts_with('\t');
        if starts_new_key {
            if let Some(key_path) = current_key.take() {
                blocks.push(RegistryQueryBlock {
                    key_path,
                    values: current_values,
                });
                current_values = BTreeMap::new();
            }
            current_key = Some(trimmed_end.trim().to_string());
            continue;
        }

        if let Some((name, value)) = parse_registry_value_line(trimmed_end) {
            current_values.insert(name, value);
        }
    }

    if let Some(key_path) = current_key {
        blocks.push(RegistryQueryBlock {
            key_path,
            values: current_values,
        });
    }

    blocks
}

pub fn detect_registry_editors_from_blocks<F>(
    blocks: &[RegistryQueryBlock],
    path_exists: F,
) -> Vec<DetectedEditor>
where
    F: Fn(&Path) -> bool,
{
    let mut editors = Vec::new();

    for product in PRODUCTS {
        for block in blocks {
            if let Some(editor) = resolve_registry_editor(product, block, &path_exists) {
                editors.push(editor);
            }
        }
    }

    editors
}

pub fn detect_path_editors_from_where_output<F>(
    alias: &str,
    raw: &str,
    path_exists: F,
) -> Vec<DetectedEditor>
where
    F: Fn(&Path) -> bool,
{
    let mut editors = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let path = PathBuf::from(trimmed);
        if path_exists(path.as_path()) {
            editors.push(DetectedEditor {
                id: format!("path:{alias}"),
                name: format!("PATH command: {alias}"),
                executable: normalize_existing_path(path.as_path()),
                source: DetectionSource::Path,
            });
        }
    }

    editors
}

pub fn deduplicate_detected_editors(editors: Vec<DetectedEditor>) -> Vec<DetectedEditor> {
    let mut deduped: Vec<DetectedEditor> = Vec::new();
    let mut index_by_key: BTreeMap<String, usize> = BTreeMap::new();

    for editor in editors {
        let key = normalized_path_key(&editor.executable);
        if let Some(existing_index) = index_by_key.get(&key).copied() {
            if detection_source_priority(editor.source)
                < detection_source_priority(deduped[existing_index].source)
            {
                deduped[existing_index] = editor;
            }
        } else {
            index_by_key.insert(key, deduped.len());
            deduped.push(editor);
        }
    }

    deduped.sort_by(|left, right| {
        detection_source_priority(left.source)
            .cmp(&detection_source_priority(right.source))
            .then_with(|| product_sort_key(&left.id).cmp(&product_sort_key(&right.id)))
            .then_with(|| {
                left.executable
                    .to_ascii_lowercase()
                    .cmp(&right.executable.to_ascii_lowercase())
            })
    });
    deduped
}

pub fn finalize_editor_discovery(
    editors: Vec<DetectedEditor>,
    source_queried: &[bool],
    errors: &[String],
) -> Result<Vec<DetectedEditor>, String> {
    let deduped = deduplicate_detected_editors(editors);
    if !deduped.is_empty() || source_queried.iter().copied().any(|queried| queried) {
        return Ok(deduped);
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    Err("no detection source could be queried".to_string())
}

pub fn parse_registry_dword(raw: &str) -> Option<u32> {
    let token = raw
        .trim()
        .split_whitespace()
        .next()?
        .split('(')
        .next()?
        .trim();
    if let Some(hex) = token
        .strip_prefix("0x")
        .or_else(|| token.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16).ok()
    } else {
        token.parse::<u32>().ok()
    }
}

pub fn windows_bgr_dword_to_css_hex(value: u32) -> String {
    let red = (value & 0xFF) as u8;
    let green = ((value >> 8) & 0xFF) as u8;
    let blue = ((value >> 16) & 0xFF) as u8;
    format!("#{red:02X}{green:02X}{blue:02X}")
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

pub fn resolve_windows_accent_candidates(
    candidates: &[Result<Option<String>, String>],
) -> Result<String, String> {
    let mut retained_error: Option<String> = None;
    let mut all_failed_to_execute = !candidates.is_empty();

    for candidate in candidates {
        match candidate {
            Ok(Some(raw_value)) => {
                all_failed_to_execute = false;
                if let Some(value) = parse_registry_dword(raw_value) {
                    return Ok(windows_bgr_dword_to_css_hex(value));
                }
            }
            Ok(None) => {
                all_failed_to_execute = false;
            }
            Err(error) => {
                if retained_error.is_none() {
                    retained_error = Some(error.clone());
                }
            }
        }
    }

    if all_failed_to_execute {
        return Err(retained_error.unwrap_or_else(|| WINDOWS_ACCENT_NOT_FOUND.to_string()));
    }

    Err(WINDOWS_ACCENT_NOT_FOUND.to_string())
}

#[tauri::command]
pub fn get_app_info() -> Result<AppInfo, String> {
    let data_dir = super::project::app_data_dir().map_err(|e| e.to_string())?;
    get_app_info_in(&data_dir)
}

#[tauri::command]
pub fn detect_editors() -> Result<Vec<DetectedEditor>, String> {
    if !cfg!(windows) {
        return Err(WINDOWS_ONLY_ERROR.to_string());
    }

    let mut editors = Vec::new();
    let mut source_queried = vec![false, false, false];
    let mut errors = Vec::new();

    let (registry_blocks, registry_queried, registry_errors) = query_uninstall_registry_blocks();
    source_queried[0] = registry_queried;
    errors.extend(registry_errors);
    editors.extend(detect_registry_editors_from_blocks(
        &registry_blocks,
        |path| path.is_file(),
    ));

    let standard_roots = standard_search_roots();
    source_queried[1] = !standard_roots.is_empty();
    editors.extend(detect_standard_editors_in_roots(&standard_roots, |path| {
        path.is_file()
    }));

    let (path_editors, path_queried, path_errors) = query_path_editors();
    source_queried[2] = path_queried;
    errors.extend(path_errors);
    editors.extend(path_editors);

    finalize_editor_discovery(editors, &source_queried, &errors)
}

#[tauri::command]
pub fn get_windows_accent_color() -> Result<String, String> {
    if !cfg!(windows) {
        return Err(WINDOWS_ONLY_ERROR.to_string());
    }

    let registry_targets = [
        (
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Accent",
            "AccentColorMenu",
        ),
        (r"HKCU\Software\Microsoft\Windows\DWM", "AccentColor"),
    ];

    let candidates = registry_targets
        .iter()
        .map(|(key_path, value_name)| query_registry_value(key_path, value_name))
        .collect::<Vec<_>>();

    resolve_windows_accent_candidates(&candidates)
}

fn query_uninstall_registry_blocks() -> (Vec<RegistryQueryBlock>, bool, Vec<String>) {
    let mut blocks = Vec::new();
    let mut queried_any = false;
    let mut errors = Vec::new();

    for root in UNINSTALL_ROOTS {
        match run_command_capture(
            "reg.exe",
            &[String::from("query"), root.to_string(), String::from("/s")],
        ) {
            Ok(capture) if capture.status_code == Some(0) => {
                queried_any = true;
                blocks.extend(parse_registry_query_blocks(&capture.stdout));
            }
            Ok(capture) => {
                errors.push(command_exit_message(
                    "reg.exe",
                    capture.status_code,
                    &capture.stdout,
                    &capture.stderr,
                ));
            }
            Err(error) => errors.push(error),
        }
    }

    (blocks, queried_any, errors)
}

fn detect_standard_editors_in_roots<F>(roots: &[PathBuf], path_exists: F) -> Vec<DetectedEditor>
where
    F: Fn(&Path) -> bool,
{
    let mut editors = Vec::new();
    for product in PRODUCTS {
        for root in roots {
            for directory in product.standard_dirs {
                for executable_name in product.executable_names {
                    let candidate = root.join(directory).join(executable_name);
                    if path_exists(candidate.as_path()) {
                        editors.push(DetectedEditor {
                            id: product.id.to_string(),
                            name: product.name.to_string(),
                            executable: normalize_existing_path(candidate.as_path()),
                            source: DetectionSource::Standard,
                        });
                    }
                }
            }
        }
    }
    editors
}

fn resolve_registry_editor<F>(
    product: EditorProduct,
    block: &RegistryQueryBlock,
    path_exists: &F,
) -> Option<DetectedEditor>
where
    F: Fn(&Path) -> bool,
{
    if !registry_block_matches_product(product, block) {
        return None;
    }

    if let Some(path) = block
        .values
        .get("DisplayIcon")
        .and_then(|value| parse_registry_path(value))
        .filter(|path| product_path_matches(product, path.as_path()))
        .filter(|path| path_exists(path.as_path()))
    {
        return Some(DetectedEditor {
            id: product.id.to_string(),
            name: product.name.to_string(),
            executable: normalize_existing_path(path.as_path()),
            source: DetectionSource::Registry,
        });
    }

    if let Some(install_location) = block
        .values
        .get("InstallLocation")
        .and_then(|value| parse_registry_path(value))
    {
        for executable_name in product.executable_names {
            let candidate = install_location.join(executable_name);
            if path_exists(candidate.as_path()) {
                return Some(DetectedEditor {
                    id: product.id.to_string(),
                    name: product.name.to_string(),
                    executable: normalize_existing_path(candidate.as_path()),
                    source: DetectionSource::Registry,
                });
            }
        }
    }

    None
}

fn registry_block_matches_product(product: EditorProduct, block: &RegistryQueryBlock) -> bool {
    let key_path = block.key_path.to_ascii_lowercase();
    let display_name = block
        .values
        .get("DisplayName")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_default();
    let display_icon_name = block
        .values
        .get("DisplayIcon")
        .and_then(|value| parse_registry_path(value))
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_default();

    let contains_marker = |haystack: &str, markers: &[&str]| {
        markers
            .iter()
            .any(|marker| haystack.contains(&marker.to_ascii_lowercase()))
    };
    let contains_forbidden = |haystack: &str| {
        product
            .forbidden_markers
            .iter()
            .any(|marker| haystack.contains(&marker.to_ascii_lowercase()))
    };

    if !display_icon_name.is_empty()
        && product
            .executable_names
            .iter()
            .any(|name| display_icon_name.eq_ignore_ascii_case(name))
    {
        return true;
    }

    (contains_marker(&display_name, product.display_markers) && !contains_forbidden(&display_name))
        || (contains_marker(&key_path, product.key_markers) && !contains_forbidden(&key_path))
}

fn product_path_matches(product: EditorProduct, path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|file_name| {
            product
                .executable_names
                .iter()
                .any(|expected| file_name.eq_ignore_ascii_case(expected))
        })
        .unwrap_or(false)
}

fn parse_registry_value_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_start();
    let name_end = trimmed.find(char::is_whitespace)?;
    let name = trimmed[..name_end].trim();
    let type_and_value = trimmed[name_end..].trim_start();
    let reg_type_end = type_and_value.find(char::is_whitespace)?;
    let reg_type = type_and_value[..reg_type_end].trim();
    if name.is_empty() || reg_type.is_empty() {
        return None;
    }
    let value = type_and_value[reg_type_end..].trim_start().to_string();
    if value.is_empty() {
        return None;
    }
    Some((name.to_string(), value))
}

fn parse_registry_path(raw: &str) -> Option<PathBuf> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let candidate = if let Some(rest) = trimmed.strip_prefix('"') {
        rest.split('"').next()?.trim().to_string()
    } else {
        trimmed.split(',').next()?.trim().to_string()
    };
    if candidate.is_empty() {
        return None;
    }

    Some(PathBuf::from(expand_windows_env_vars(&candidate)))
}

fn expand_windows_env_vars(value: &str) -> String {
    let mut expanded = String::new();
    let mut rest = value;

    while let Some(start) = rest.find('%') {
        expanded.push_str(&rest[..start]);
        let after_start = &rest[start + 1..];
        if let Some(end) = after_start.find('%') {
            let key = &after_start[..end];
            if let Some(resolved) = std::env::var_os(key) {
                expanded.push_str(&resolved.to_string_lossy());
            } else {
                expanded.push('%');
                expanded.push_str(key);
                expanded.push('%');
            }
            rest = &after_start[end + 1..];
        } else {
            expanded.push_str(&rest[start..]);
            rest = "";
        }
    }

    expanded.push_str(rest);
    expanded
}

fn run_where(alias: &str) -> Result<String, String> {
    let capture = run_command_capture("where.exe", &[alias.to_string()])?;
    classify_where_output(capture.status_code, &capture.stdout, &capture.stderr)
}

fn run_command_capture(program: &str, args: &[String]) -> Result<CommandCapture, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|error| command_error_message(program, &error))?;
    Ok(CommandCapture {
        status_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

fn standard_search_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
        roots.push(PathBuf::from(local_app_data).join("Programs"));
    }
    if let Some(program_files) = std::env::var_os("ProgramFiles") {
        roots.push(PathBuf::from(program_files));
    }
    if let Some(program_files_x86) = std::env::var_os("ProgramFiles(x86)") {
        roots.push(PathBuf::from(program_files_x86));
    }
    roots
}

fn normalize_existing_path(path: &Path) -> String {
    match path.canonicalize() {
        Ok(canonical) => strip_verbatim_prefix(&canonical.display().to_string()),
        Err(_) => strip_verbatim_prefix(&path.display().to_string()),
    }
}

fn normalized_path_key(path: &str) -> String {
    strip_verbatim_prefix(path).to_ascii_lowercase()
}

fn strip_verbatim_prefix(path: &str) -> String {
    path.strip_prefix(r"\\?\").unwrap_or(path).to_string()
}

fn command_exit_message(
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

fn detection_source_priority(source: DetectionSource) -> usize {
    match source {
        DetectionSource::Registry => 0,
        DetectionSource::Standard => 1,
        DetectionSource::Path => 2,
    }
}

fn product_sort_key(id: &str) -> (usize, usize, String) {
    if let Some(index) = PRODUCTS.iter().position(|product| product.id == id) {
        return (0, index, String::new());
    }
    if let Some(alias) = id.strip_prefix("path:") {
        if let Some(index) = PATH_ALIASES
            .iter()
            .position(|item| item.eq_ignore_ascii_case(alias))
        {
            return (1, index, alias.to_ascii_lowercase());
        }
        return (1, PATH_ALIASES.len(), alias.to_ascii_lowercase());
    }
    (2, usize::MAX, id.to_ascii_lowercase())
}

fn query_path_editors() -> (Vec<DetectedEditor>, bool, Vec<String>) {
    let mut editors = Vec::new();
    let mut queried_any = false;
    let mut errors = Vec::new();

    for alias in PATH_ALIASES {
        match run_where(alias) {
            Ok(output) => {
                queried_any = true;
                editors.extend(detect_path_editors_from_where_output(
                    alias,
                    &output,
                    |path| path.is_file(),
                ));
            }
            Err(error) => errors.push(error),
        }
    }

    (editors, queried_any, errors)
}

fn query_registry_value(key_path: &str, value_name: &str) -> Result<Option<String>, String> {
    let capture = run_command_capture("reg.exe", &[String::from("query"), key_path.to_string()])?;

    match capture.status_code {
        Some(0) => {
            let blocks = parse_registry_query_blocks(&capture.stdout);
            Ok(blocks
                .iter()
                .find(|block| block.key_path.eq_ignore_ascii_case(key_path))
                .and_then(|block| block.values.get(value_name))
                .cloned()
                .or_else(|| {
                    blocks
                        .first()
                        .and_then(|block| block.values.get(value_name))
                        .cloned()
                }))
        }
        Some(1) => Ok(None),
        _ => Err(command_exit_message(
            "reg.exe",
            capture.status_code,
            &capture.stdout,
            &capture.stderr,
        )),
    }
}
