//! D3 路径查重测试：重复路径 / 大小写变体 / 末尾分隔符变体。

use windy_project_mgr_lib::project::dedup::{is_same_path, normalize_path};
use windy_project_mgr_lib::project::{Project, Store};

fn project(id: &str, path: &str) -> Project {
    Project {
        id: id.to_string(),
        name: format!("name-{id}"),
        path: path.to_string(),
        description: None,
        tags: vec![],
        run_command: None,
        build_command: None,
        created_at: "2026-08-25T00:00:00Z".to_string(),
    }
}

// ---------- normalize / 相等 ----------

#[test]
fn normalize_trims_trailing_separators() {
    assert_eq!(normalize_path("D:\\projects\\alpha\\"), "D:\\projects\\alpha");
    assert_eq!(normalize_path("D:\\projects\\alpha/"), "D:\\projects\\alpha");
    assert_eq!(normalize_path("D:\\projects\\alpha"), "D:\\projects\\alpha");
}

#[test]
fn normalize_unifies_forward_slashes() {
    assert_eq!(normalize_path("D:/projects/alpha"), "D:\\projects\\alpha");
}

#[test]
fn normalize_resolves_dot_segments_lexically() {
    assert_eq!(normalize_path("D:\\a\\b\\..\\c"), "D:\\a\\c");
    assert_eq!(normalize_path("D:\\a\\.\\b"), "D:\\a\\b");
    assert_eq!(normalize_path("D:\\a\\..\\..\\b"), "D:\\b");
}

#[test]
fn normalize_keeps_unc_share_root() {
    assert_eq!(
        normalize_path("\\\\server\\share\\x\\..\\y"),
        "\\\\server\\share\\y"
    );
}

#[test]
fn normalize_makes_relative_path_absolute() {
    let normalized = normalize_path("foo\\bar");
    assert!(
        normalized.ends_with("\\foo\\bar"),
        "relative path must be resolved against cwd: {normalized}"
    );
    assert!(
        normalized.len() >= 3 && &normalized[1..3] == ":\\",
        "normalized path must be absolute (drive-rooted): {normalized}"
    );
}

#[test]
fn duplicate_exact_path_detected() {
    assert!(is_same_path(
        "D:\\projects\\alpha",
        "D:\\projects\\alpha"
    ));
}

#[test]
fn duplicate_case_variant_detected() {
    assert!(is_same_path(
        "d:\\PROJECTS\\Alpha",
        "D:\\projects\\alpha"
    ));
}

#[test]
fn duplicate_trailing_separator_variant_detected() {
    assert!(is_same_path(
        "D:\\projects\\alpha\\",
        "D:\\projects\\alpha"
    ));
    assert!(is_same_path(
        "D:\\projects\\alpha/",
        "D:\\projects\\alpha"
    ));
}

#[test]
fn different_paths_are_not_duplicates() {
    assert!(!is_same_path(
        "D:\\projects\\alpha",
        "D:\\projects\\beta"
    ));
    assert!(!is_same_path(
        "D:\\projects\\alpha",
        "D:\\projects\\alphabet"
    ));
}

// ---------- Store 查找 ----------

#[test]
fn store_finds_duplicate_by_path_variant() {
    let mut store = Store::default();
    store.create(project("p1", "D:\\projects\\alpha")).expect("create");
    let dup = store
        .find_duplicate_by_path("d:\\Projects\\ALPHA\\", None)
        .expect("case + trailing separator variant must hit");
    assert_eq!(dup.id, "p1");
}

#[test]
fn store_excludes_self_when_checking_update() {
    let mut store = Store::default();
    store.create(project("p1", "D:\\projects\\alpha")).expect("create");
    assert!(
        store
            .find_duplicate_by_path("D:\\projects\\alpha", Some("p1"))
            .is_none(),
        "record must not duplicate against itself"
    );
}

#[test]
fn store_reports_no_duplicate_for_new_path() {
    let mut store = Store::default();
    store.create(project("p1", "D:\\projects\\alpha")).expect("create");
    assert!(store.find_duplicate_by_path("D:\\projects\\beta", None).is_none());
}
