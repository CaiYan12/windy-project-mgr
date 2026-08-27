//! `settings.json` 读写（D6）：与 `projects.json` 同机制
//!（`version` 字段、临时文件替换、损坏可诊断）。

use super::types::{StoreError, Versioned};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CURRENT_VERSION: u32 = 2;

const DEFAULT_PATH_PLACEHOLDER: &str = "{path}";
const DEFAULT_ACCENT_PRESET: &str = "windy-teal";
const ACCENT_PRESETS: [&str; 6] = [
    "windy-teal",
    "ocean-blue",
    "violet",
    "amber",
    "coral",
    "rose",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub enum AccentColor {
    Preset { value: String },
    Windows {},
    Custom { value: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorProfile {
    #[serde(default)]
    pub executable: String,
    #[serde(default = "default_editor_arguments")]
    pub arguments: Vec<String>,
}

/// Settings v2：颜色模式、强调色与编辑器配置。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub color_mode: ColorMode,
    #[serde(default)]
    pub accent_color: AccentColor,
    #[serde(default)]
    pub editor: EditorProfile,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacySettings {
    #[serde(default)]
    editor_command: String,
    #[serde(default)]
    theme: LegacyTheme,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum LegacyTheme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Deserialize)]
struct VersionOnly {
    version: u32,
}

fn default_accent_color() -> AccentColor {
    AccentColor::Preset {
        value: DEFAULT_ACCENT_PRESET.to_string(),
    }
}

fn default_editor_arguments() -> Vec<String> {
    vec![DEFAULT_PATH_PLACEHOLDER.to_string()]
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            color_mode: ColorMode::System,
            accent_color: default_accent_color(),
            editor: EditorProfile::default(),
        }
    }
}

impl Default for AccentColor {
    fn default() -> Self {
        default_accent_color()
    }
}

impl Default for EditorProfile {
    fn default() -> Self {
        Self {
            executable: String::new(),
            arguments: default_editor_arguments(),
        }
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), StoreError> {
        self.accent_color.validate()?;
        self.editor.validate()?;
        Ok(())
    }

    /// 从文件加载。文件不存在 → 默认设置；文件为空或损坏 → 可诊断错误。
    pub fn load(path: &Path) -> Result<Settings, StoreError> {
        let raw = match std::fs::read_to_string(path) {
            Ok(raw) => raw,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Settings::default());
            }
            Err(e) => return Err(StoreError::Io(e)),
        };
        if raw.trim().is_empty() {
            return Err(StoreError::Corrupted {
                path: path.to_path_buf(),
                detail: "file is empty".to_string(),
            });
        }
        let version = serde_json::from_str::<VersionOnly>(&raw)
            .map_err(|e| corrupted(path, e.to_string()))?
            .version;

        match version {
            1 => {
                let versioned: Versioned<LegacySettings> =
                    serde_json::from_str(&raw).map_err(|e| corrupted(path, e.to_string()))?;
                let settings = Settings::from_legacy(versioned.payload);
                settings
                    .validate()
                    .map_err(|e| invalid_loaded_settings(path, e))?;
                settings.save(path)?;
                Ok(settings)
            }
            CURRENT_VERSION => {
                let versioned: Versioned<Settings> =
                    serde_json::from_str(&raw).map_err(|e| corrupted(path, e.to_string()))?;
                let settings = versioned.payload;
                settings
                    .validate()
                    .map_err(|e| invalid_loaded_settings(path, e))?;
                Ok(settings)
            }
            found => Err(StoreError::VersionMismatch {
                found,
                expected: CURRENT_VERSION,
            }),
        }
    }

    /// 原子保存：写临时文件、同步、重命名替换；失败不破坏原文件。
    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        self.validate()?;
        let versioned = Versioned {
            version: CURRENT_VERSION,
            payload: self.clone(),
        };
        let json = serde_json::to_string_pretty(&versioned)
            .map_err(|e| StoreError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        super::store::write_atomic(path, &json)
    }

    fn from_legacy(legacy: LegacySettings) -> Self {
        Self {
            color_mode: legacy.theme.into(),
            accent_color: AccentColor::default(),
            editor: EditorProfile {
                executable: legacy.editor_command,
                arguments: default_editor_arguments(),
            },
        }
    }
}

impl AccentColor {
    fn validate(&self) -> Result<(), StoreError> {
        match self {
            AccentColor::Preset { value } => {
                if ACCENT_PRESETS.contains(&value.as_str()) {
                    Ok(())
                } else {
                    Err(StoreError::Validation {
                        detail: format!(
                            "accentColor preset must be one of {}",
                            ACCENT_PRESETS.join(", ")
                        ),
                    })
                }
            }
            AccentColor::Windows {} => Ok(()),
            AccentColor::Custom { value } => {
                if is_css_hex_color(value) {
                    Ok(())
                } else {
                    Err(StoreError::Validation {
                        detail: "accentColor custom value must be #RRGGBB".to_string(),
                    })
                }
            }
        }
    }
}

impl EditorProfile {
    fn validate(&self) -> Result<(), StoreError> {
        if self.executable.trim().is_empty() {
            return Ok(());
        }

        let placeholder_count: usize = self
            .arguments
            .iter()
            .map(|arg| arg.matches(DEFAULT_PATH_PLACEHOLDER).count())
            .sum();
        if placeholder_count == 1 {
            if is_batch_executable(&self.executable)
                && self.arguments.iter().any(|argument| argument.contains('"'))
            {
                return Err(StoreError::Validation {
                    detail: "cmd.exe batch arguments cannot contain the double quote character"
                        .to_string(),
                });
            }
            Ok(())
        } else {
            Err(StoreError::Validation {
                detail: "editor.arguments must contain exactly one {path} placeholder when editor.executable is configured".to_string(),
            })
        }
    }
}

pub fn is_batch_executable(executable: &str) -> bool {
    let executable = executable.trim().to_ascii_lowercase();
    executable.ends_with(".cmd") || executable.ends_with(".bat")
}

impl From<LegacyTheme> for ColorMode {
    fn from(value: LegacyTheme) -> Self {
        match value {
            LegacyTheme::System => Self::System,
            LegacyTheme::Light => Self::Light,
            LegacyTheme::Dark => Self::Dark,
        }
    }
}

fn is_css_hex_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value.chars().skip(1).all(|ch| ch.is_ascii_hexdigit())
}

fn corrupted(path: &Path, detail: String) -> StoreError {
    StoreError::Corrupted {
        path: path.to_path_buf(),
        detail,
    }
}

fn invalid_loaded_settings(path: &Path, err: StoreError) -> StoreError {
    let detail = match err {
        StoreError::Validation { detail } => detail,
        other => other.to_string(),
    };
    corrupted(path, detail)
}
