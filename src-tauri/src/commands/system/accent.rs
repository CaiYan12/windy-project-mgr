//! Windows 强调色解析（B1：自 `commands/system.rs` 拆出）。

use super::registry::parse_registry_dword;

pub(crate) const WINDOWS_ACCENT_NOT_FOUND: &str =
    "Windows accent color was not found in the registry";

pub fn windows_bgr_dword_to_css_hex(value: u32) -> String {
    let red = (value & 0xFF) as u8;
    let green = ((value >> 8) & 0xFF) as u8;
    let blue = ((value >> 16) & 0xFF) as u8;
    format!("#{red:02X}{green:02X}{blue:02X}")
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
