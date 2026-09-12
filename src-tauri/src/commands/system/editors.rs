//! 编辑器发现：产品目录、注册表 / PATH / 标准路径三源检测与去重（B1：自 `commands/system.rs` 拆出）。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::exec::run_where;
use super::registry::parse_registry_path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DetectionSource {
    Path,
    Registry,
    Standard,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedEditor {
    pub id: String,
    pub name: String,
    pub executable: String,
    pub source: DetectionSource,
}

const PATH_ALIASES: [&str; 6] = [
    "code",
    "code-insiders",
    "cursor",
    "windsurf",
    "codium",
    "zed",
];

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

pub fn detect_registry_editors_from_blocks<F>(
    blocks: &[super::registry::RegistryQueryBlock],
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

pub(crate) fn detect_standard_editors_in_roots<F>(roots: &[PathBuf], path_exists: F) -> Vec<DetectedEditor>
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

pub(crate) fn query_path_editors() -> (Vec<DetectedEditor>, bool, Vec<String>) {
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

pub(crate) fn standard_search_roots() -> Vec<PathBuf> {
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

pub(crate) fn strip_verbatim_prefix(path: &str) -> String {
    path.strip_prefix(r"\\?\").unwrap_or(path).to_string()
}

fn resolve_registry_editor<F>(
    product: EditorProduct,
    block: &super::registry::RegistryQueryBlock,
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

fn registry_block_matches_product(
    product: EditorProduct,
    block: &super::registry::RegistryQueryBlock,
) -> bool {
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

fn normalize_existing_path(path: &Path) -> String {
    match path.canonicalize() {
        Ok(canonical) => strip_verbatim_prefix(&canonical.display().to_string()),
        Err(_) => strip_verbatim_prefix(&path.display().to_string()),
    }
}

fn normalized_path_key(path: &str) -> String {
    strip_verbatim_prefix(path).to_ascii_lowercase()
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
