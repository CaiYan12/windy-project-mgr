//! 注册表文本解析与 `reg.exe` 查询（B1：自 `commands/system.rs` 拆出）。

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::exec::{command_exit_message, run_command_capture};

pub(crate) const UNINSTALL_ROOTS: [&str; 3] = [
    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall",
    r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall",
    r"HKLM\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryQueryBlock {
    pub key_path: String,
    pub values: BTreeMap<String, String>,
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

pub(crate) fn query_uninstall_registry_blocks() -> (Vec<RegistryQueryBlock>, bool, Vec<String>) {
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

pub(crate) fn query_registry_value(
    key_path: &str,
    value_name: &str,
) -> Result<Option<String>, String> {
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

pub(crate) fn parse_registry_path(raw: &str) -> Option<PathBuf> {
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
