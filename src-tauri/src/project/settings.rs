//! `settings.json` 读写（D6）：与 `projects.json` 同机制
//!（`version` 字段、临时文件替换、损坏可诊断）。
//! v3 新增 `appearance` 域（主题工坊）；v1 / v2 文件加载时自动迁移为当前版本并重写。

use super::types::{StoreError, Versioned};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CURRENT_VERSION: u32 = 3;

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

/// 内置风格预设包 id（与前端 appearance.ts 的 STYLE_PRESET_IDS 一致）。
const STYLE_PRESETS: [&str; 4] = ["windy", "cloud", "ink", "midnight"];
const MAX_RADIUS: u32 = 20;
const MIN_FONT_SIZE: u32 = 13;
const MAX_FONT_SIZE: u32 = 16;
const MAX_FONT_STACK_LEN: usize = 200;

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

/// 密度档（settings v3 主题工坊）：紧凑缩放 0.85，舒适 1。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Density {
    Compact,
    #[default]
    Comfortable,
}

/// 单套中性色（settings v3）：亮 / 暗各一份，全部 #RRGGBB。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeutralSet {
    #[serde(default)]
    pub bg: String,
    #[serde(default)]
    pub surface: String,
    #[serde(default)]
    pub sunken: String,
    #[serde(default)]
    pub line: String,
    #[serde(default)]
    pub ink: String,
    #[serde(default)]
    pub muted: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceNeutrals {
    #[serde(default = "default_light_neutrals")]
    pub light: NeutralSet,
    #[serde(default = "default_dark_neutrals")]
    pub dark: NeutralSet,
}

/// Settings v3 `appearance` 域（主题工坊）：风格预设包、圆角、字号、密度、正文字体与中性色。
/// 默认值即 Windy 预设（与前端 appearance.ts DEFAULT_APPEARANCE 一致）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceSettings {
    /// Some(预设 id) = 选中内置包；None = 自定义主题。
    #[serde(default)]
    pub style_preset: Option<String>,
    #[serde(default = "default_radius")]
    pub radius: u32,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default)]
    pub density: Density,
    /// "" = 使用样式表默认系统栈。
    #[serde(default)]
    pub font_family: String,
    #[serde(default = "default_appearance_neutrals")]
    pub neutrals: AppearanceNeutrals,
}

/// Settings v3：颜色模式、强调色、外观（主题工坊）与编辑器配置。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub color_mode: ColorMode,
    #[serde(default)]
    pub accent_color: AccentColor,
    #[serde(default)]
    pub appearance: AppearanceSettings,
    #[serde(default)]
    pub editor: EditorProfile,
}

/// Settings v2 载荷（迁移输入）：无 appearance 域。
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SettingsV2 {
    #[serde(default)]
    color_mode: ColorMode,
    #[serde(default)]
    accent_color: AccentColor,
    #[serde(default)]
    editor: EditorProfile,
}

impl From<SettingsV2> for Settings {
    fn from(v2: SettingsV2) -> Self {
        Self {
            color_mode: v2.color_mode,
            accent_color: v2.accent_color,
            appearance: AppearanceSettings::default(),
            editor: v2.editor,
        }
    }
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

fn default_radius() -> u32 {
    10
}

fn default_font_size() -> u32 {
    14
}

/// Windy 默认亮色中性色（与 App.css :root / 前端 WINDY_NEUTRALS 一致）。
fn default_light_neutrals() -> NeutralSet {
    NeutralSet {
        bg: "#f5f6f8".to_string(),
        surface: "#ffffff".to_string(),
        sunken: "#eceef2".to_string(),
        line: "#dfe3e8".to_string(),
        ink: "#17191f".to_string(),
        muted: "#6a7280".to_string(),
    }
}

/// Windy 默认暗色中性色（与 :root[data-theme="dark"] / 前端 WINDY_NEUTRALS 一致）。
fn default_dark_neutrals() -> NeutralSet {
    NeutralSet {
        bg: "#121417".to_string(),
        surface: "#1a1d22".to_string(),
        sunken: "#23272e".to_string(),
        line: "#2e343c".to_string(),
        ink: "#e8eaee".to_string(),
        muted: "#949ca8".to_string(),
    }
}

fn default_appearance_neutrals() -> AppearanceNeutrals {
    AppearanceNeutrals {
        light: default_light_neutrals(),
        dark: default_dark_neutrals(),
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            color_mode: ColorMode::System,
            accent_color: default_accent_color(),
            appearance: AppearanceSettings::default(),
            editor: EditorProfile::default(),
        }
    }
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            style_preset: Some("windy".to_string()),
            radius: default_radius(),
            font_size: default_font_size(),
            density: Density::Comfortable,
            font_family: String::new(),
            neutrals: default_appearance_neutrals(),
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
        self.appearance.validate()?;
        self.editor.validate()?;
        Ok(())
    }

    /// 从文件加载。文件不存在 → 默认设置；文件为空或损坏 → 可诊断错误。
    /// v1 / v2 文件自动迁移为当前版本并重写（沿用 v1→v2 迁移模式）。
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
            2 => {
                let versioned: Versioned<SettingsV2> =
                    serde_json::from_str(&raw).map_err(|e| corrupted(path, e.to_string()))?;
                let settings = Settings::from(versioned.payload);
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
            appearance: AppearanceSettings::default(),
            editor: EditorProfile {
                executable: legacy.editor_command,
                arguments: default_editor_arguments(),
            },
        }
    }
}

impl AppearanceSettings {
    /// 校验口径与前端 parseAppearance 一致（严格 #RRGGBB / 区间 / 控制字符）。
    fn validate(&self) -> Result<(), StoreError> {
        if let Some(preset) = &self.style_preset {
            if !STYLE_PRESETS.contains(&preset.as_str()) {
                return Err(StoreError::Validation {
                    detail: format!(
                        "appearance.stylePreset must be one of {}",
                        STYLE_PRESETS.join(", ")
                    ),
                });
            }
        }
        if self.radius > MAX_RADIUS {
            return Err(StoreError::Validation {
                detail: format!("appearance.radius must be between 0 and {MAX_RADIUS}"),
            });
        }
        if self.font_size < MIN_FONT_SIZE || self.font_size > MAX_FONT_SIZE {
            return Err(StoreError::Validation {
                detail: format!(
                    "appearance.fontSize must be between {MIN_FONT_SIZE} and {MAX_FONT_SIZE}"
                ),
            });
        }
        if self
            .font_family
            .chars()
            .any(|ch| matches!(ch, '\u{0000}'..='\u{001f}' | '\u{007f}'))
        {
            return Err(StoreError::Validation {
                detail: "appearance.fontFamily must not contain control characters".to_string(),
            });
        }
        if self.font_family.chars().count() > MAX_FONT_STACK_LEN {
            return Err(StoreError::Validation {
                detail: format!(
                    "appearance.fontFamily must be at most {MAX_FONT_STACK_LEN} characters"
                ),
            });
        }
        for (mode, set) in [
            ("light", &self.neutrals.light),
            ("dark", &self.neutrals.dark),
        ] {
            for (key, value) in [
                ("bg", &set.bg),
                ("surface", &set.surface),
                ("sunken", &set.sunken),
                ("line", &set.line),
                ("ink", &set.ink),
                ("muted", &set.muted),
            ] {
                if !is_css_hex_color(value) {
                    return Err(StoreError::Validation {
                        detail: format!("appearance.neutrals.{mode}.{key} must be #RRGGBB"),
                    });
                }
            }
        }
        Ok(())
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
        match self.placeholder_rule_violation() {
            Some(detail) => Err(StoreError::Validation { detail }),
            None => Ok(()),
        }
    }

    /// 占位符 / 批处理引号规则的唯一实现：合法返回 `None`，否则返回可诊断详情。
    /// `validate()` 与 `resolve_arguments()` 共用，避免规则在多处漂移（A3 / B2）。
    fn placeholder_rule_violation(&self) -> Option<String> {
        let placeholder_count: usize = self
            .arguments
            .iter()
            .map(|arg| arg.matches(DEFAULT_PATH_PLACEHOLDER).count())
            .sum();
        if placeholder_count != 1 {
            return Some(
                "editor.arguments must contain exactly one {path} placeholder when editor.executable is configured"
                    .to_string(),
            );
        }
        if is_batch_executable(&self.executable) {
            if let Some(detail) = batch_quote_violation(&self.arguments) {
                return Some(detail);
            }
        }
        None
    }

    /// 启动参数解算（B2）：替换唯一 `{path}` 占位符并校验批处理约束。
    /// 这是该规则在后端唯一的执行入口；`launch` 只负责把结果组装成 `LaunchPlan`。
    pub fn resolve_arguments(&self, path: &Path) -> Result<Vec<String>, EditorProfileError> {
        if self.executable.trim().is_empty() {
            return Err(EditorProfileError::NotConfigured);
        }
        if let Some(detail) = self.placeholder_rule_violation() {
            return Err(EditorProfileError::Invalid { detail });
        }
        let path_string = path.display().to_string();
        let substituted: Vec<String> = self
            .arguments
            .iter()
            .map(|argument| argument.replacen(DEFAULT_PATH_PLACEHOLDER, &path_string, 1))
            .collect();
        // 保留旧 launch 的语义：批处理命令的**替换后**参数同样不得含双引号
        //（例如项目路径本身含引号时）。
        if is_batch_executable(&self.executable) {
            if let Some(detail) = batch_quote_violation(&substituted) {
                return Err(EditorProfileError::Invalid { detail });
            }
        }
        Ok(substituted)
    }
}

/// 批处理参数引号规则（B2）：参数中不得含双引号。
/// `validate` 检查原始参数，`resolve_arguments` 检查替换后的参数。
fn batch_quote_violation(arguments: &[String]) -> Option<String> {
    if arguments.iter().any(|argument| argument.contains('"')) {
        Some("cmd.exe batch arguments cannot contain the double quote character".to_string())
    } else {
        None
    }
}

/// 编辑器配置解算错误（B2）：区分「未配置」与「配置非法」。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorProfileError {
    NotConfigured,
    Invalid { detail: String },
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
