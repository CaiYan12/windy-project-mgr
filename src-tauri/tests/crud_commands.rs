//! Project CRUD command 核心函数集成测试：真实临时数据目录（D12）。

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::commands::project::{
    check_path_available_in, create_project_in, delete_project_in, get_project_in,
    get_projects_in, update_project_in, portable_data_dir, CreateProjectInput, PathAvailability,
};
use windy_project_mgr_lib::project::{Project, StoreError};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_data_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p5-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

fn input(name: &str, path: &str) -> CreateProjectInput {
    CreateProjectInput {
        name: name.to_string(),
        path: path.to_string(),
        description: None,
        tags: vec![],
        run_command: None,
        build_command: None,
    }
}

#[test]
fn portable_data_dir_is_sibling_of_executable() {
    let executable = PathBuf::from(r"C:\Apps\Windy\windy-project-mgr.exe");

    let data_dir = portable_data_dir(&executable).expect("executable has parent");

    assert_eq!(data_dir, PathBuf::from(r"C:\Apps\Windy\data"));
}

#[test]
fn portable_data_dir_preserves_spaces_in_parent_path() {
    let executable = PathBuf::from(r"D:\Portable Apps\Windy Project Manager\windy-project-mgr.exe");

    let data_dir = portable_data_dir(&executable).expect("executable has parent");

    assert_eq!(
        data_dir,
        PathBuf::from(r"D:\Portable Apps\Windy Project Manager\data")
    );
}

#[test]
fn portable_data_dir_rejects_parentless_path() {
    let error = portable_data_dir(PathBuf::from("windy-project-mgr.exe").as_path())
        .expect_err("parentless executable path must fail");

    assert!(error.to_string().contains("executable directory is unavailable"));
}

// ---------- Create / Read ----------

#[test]
fn create_persists_project_with_generated_id_and_timestamp() {
    let dir = temp_data_dir("create");
    let created = create_project_in(&dir, input("alpha", "D:\\projects\\alpha\\"))
        .expect("create must succeed");
    assert!(!created.id.is_empty());
    assert!(created.created_at.ends_with('Z'));
    assert_eq!(created.path, "D:\\projects\\alpha", "path must be normalized");

    let all = get_projects_in(&dir).expect("list");
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, created.id);
    cleanup(&dir);
}

#[test]
fn get_project_returns_by_id_and_not_found_for_missing() {
    let dir = temp_data_dir("get");
    let created = create_project_in(&dir, input("alpha", "D:\\projects\\alpha")).expect("create");
    let fetched = get_project_in(&dir, &created.id).expect("get by id");
    assert_eq!(fetched.name, "alpha");
    let err = get_project_in(&dir, "ghost").expect_err("missing id");
    assert!(matches!(err, StoreError::NotFound { .. }), "got: {err:?}");
    cleanup(&dir);
}

// ---------- D3 查重拒绝 ----------

#[test]
fn create_rejects_duplicate_path_variants() {
    let dir = temp_data_dir("dedup");
    create_project_in(&dir, input("alpha", "D:\\projects\\alpha")).expect("create");

    for variant in [
        "D:\\projects\\alpha",   // 完全重复
        "d:\\PROJECTS\\Alpha",   // 大小写变体
        "D:\\projects\\alpha\\", // 末尾分隔符变体
        "D:/projects/alpha",     // 正斜杠变体
    ] {
        let err = create_project_in(&dir, input("dup", variant))
            .expect_err("duplicate must be rejected: {variant}");
        assert!(matches!(err, StoreError::DuplicatePath { .. }), "got: {err:?}");
    }

    assert_eq!(get_projects_in(&dir).expect("list").len(), 1, "no record added");
    cleanup(&dir);
}

#[test]
fn create_rejects_relative_path() {
    let dir = temp_data_dir("relative");
    let err = create_project_in(&dir, input("rel", "projects\\alpha"))
        .expect_err("relative path must be rejected");
    assert!(matches!(err, StoreError::Validation { .. }), "got: {err:?}");
    assert!(get_projects_in(&dir).expect("list").is_empty());
    cleanup(&dir);
}

#[test]
fn check_path_available_reports_conflict_excludes_self_and_flags_relative() {
    let dir = temp_data_dir("check-available");
    let created = create_project_in(&dir, input("alpha", "D:\\projects\\alpha")).expect("create");

    assert_eq!(
        check_path_available_in(&dir, "D:\\projects\\beta", None).expect("query"),
        PathAvailability::Available,
        "unused absolute path must be available"
    );
    assert_eq!(
        check_path_available_in(&dir, "d:/PROJECTS/alpha/", None).expect("query"),
        PathAvailability::Duplicate {
            path: "D:\\projects\\alpha".to_string()
        }
    );
    assert_eq!(
        check_path_available_in(&dir, "D:\\projects\\alpha", Some(created.id.as_str()))
            .expect("query"),
        PathAvailability::Available,
        "editing a record must not conflict with itself"
    );
    assert_eq!(
        check_path_available_in(&dir, "src-tauri\\src", None).expect("query"),
        PathAvailability::NotAbsolute,
        "relative path must be flagged before existence is considered"
    );
    cleanup(&dir);
}

// ---------- Update ----------

#[test]
fn update_persists_changes() {
    let dir = temp_data_dir("update");
    let created = create_project_in(&dir, input("alpha", "D:\\projects\\alpha")).expect("create");
    let mut changed = created.clone();
    changed.name = "renamed".to_string();
    changed.run_command = Some("start.bat".to_string());
    let updated = update_project_in(&dir, changed.clone()).expect("update");
    assert_eq!(updated.name, "renamed");
    let reloaded = get_project_in(&dir, &created.id).expect("reload");
    assert_eq!(reloaded.run_command, Some("start.bat".to_string()));
    cleanup(&dir);
}

#[test]
fn update_rejects_path_of_other_project() {
    let dir = temp_data_dir("update-dedup");
    create_project_in(&dir, input("alpha", "D:\\projects\\alpha")).expect("create");
    let beta = create_project_in(&dir, input("beta", "D:\\projects\\beta")).expect("create");
    let mut changed = beta.clone();
    changed.path = "d:\\Projects\\ALPHA\\".to_string();
    let err = update_project_in(&dir, changed).expect_err("path collision must be rejected");
    assert!(matches!(err, StoreError::DuplicatePath { .. }), "got: {err:?}");
    cleanup(&dir);
}

#[test]
fn update_missing_returns_not_found() {
    let dir = temp_data_dir("update-missing");
    let ghost = Project {
        id: "ghost".to_string(),
        name: "ghost".to_string(),
        path: "D:\\x".to_string(),
        description: None,
        tags: vec![],
        run_command: None,
        build_command: None,
        created_at: "2026-08-25T00:00:00Z".to_string(),
    };
    let err = update_project_in(&dir, ghost).expect_err("missing id");
    assert!(matches!(err, StoreError::NotFound { .. }), "got: {err:?}");
    cleanup(&dir);
}

// ---------- Delete ----------

#[test]
fn delete_removes_record_but_never_the_project_directory() {
    let dir = temp_data_dir("delete");
    let project_dir = dir.join("real-project");
    std::fs::create_dir_all(&project_dir).expect("create project dir");
    std::fs::write(project_dir.join("app.js"), "// seed").expect("seed");

    let created = create_project_in(
        &dir,
        input("real", project_dir.to_str().expect("utf8 path")),
    )
    .expect("create");
    delete_project_in(&dir, &created.id).expect("delete");
    assert!(get_projects_in(&dir).expect("list").is_empty());
    assert!(project_dir.exists(), "project directory must survive");
    assert!(project_dir.join("app.js").exists());
    cleanup(&dir);
}

#[test]
fn delete_missing_returns_not_found() {
    let dir = temp_data_dir("delete-missing");
    let err = delete_project_in(&dir, "ghost").expect_err("missing id");
    assert!(matches!(err, StoreError::NotFound { .. }), "got: {err:?}");
    cleanup(&dir);
}
