//! `projects.json` Store 集成测试：真实临时目录，不用 Mock（D12）。

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use windy_project_mgr_lib::project::{Project, Store, StoreError};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

/// 每个测试一个独立临时目录，避免相互污染。
fn temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-p4-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

fn sample_project(id: &str, path: &str) -> Project {
    Project {
        id: id.to_string(),
        name: format!("name-{id}"),
        path: path.to_string(),
        description: Some(format!("desc-{id}")),
        tags: vec!["tag-a".to_string()],
        run_command: Some("pnpm dev".to_string()),
        build_command: None,
        created_at: "2026-08-25T00:00:00Z".to_string(),
    }
}

// ---------- Save ----------

#[test]
fn save_writes_versioned_json_and_roundtrips() {
    let dir = temp_dir("save-roundtrip");
    let file = dir.join("projects.json");
    let mut store = Store::default();
    store.projects.push(sample_project("p1", "D:\\projects\\alpha"));
    store.projects.push(sample_project("p2", "D:\\projects\\beta"));
    store.save(&file).expect("save must succeed");

    let raw = std::fs::read_to_string(&file).expect("file must exist");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    assert_eq!(json["version"], 1);

    let loaded = Store::load(&file).expect("reload");
    assert_eq!(loaded, store);
    cleanup(&dir);
}

#[test]
fn save_creates_missing_parent_directory() {
    let dir = temp_dir("save-mkdir");
    let file = dir.join("nested").join("deep").join("projects.json");
    let store = Store::default();
    store.save(&file).expect("save must create parent dirs");
    assert!(file.exists());
    cleanup(&dir);
}

#[test]
fn save_failure_does_not_corrupt_existing_file() {
    let dir = temp_dir("save-failure");
    let file = dir.join("projects.json");
    let mut store = Store::default();
    store.projects.push(sample_project("p1", "D:\\projects\\alpha"));
    store.save(&file).expect("initial save");
    let before = std::fs::read_to_string(&file).expect("read");

    // 令目标路径成为一个目录，使重命名替换必然失败。
    std::fs::remove_file(&file).expect("remove");
    std::fs::create_dir(&file).expect("make target a dir");

    store.save(&file).expect_err("save must fail when replace is impossible");

    // 恢复为文件后原数据完好。
    std::fs::remove_dir_all(&file).expect("cleanup dir");
    std::fs::write(&file, &before).expect("restore");
    let loaded = Store::load(&file).expect("reload after failed save");
    assert_eq!(loaded, store);
    cleanup(&dir);
}

// ---------- CRUD ----------

#[test]
fn create_adds_project() {
    let mut store = Store::default();
    store.create(sample_project("p1", "D:\\projects\\alpha")).expect("create");
    assert_eq!(store.projects.len(), 1);
    assert_eq!(store.get("p1").expect("get").path, "D:\\projects\\alpha");
}

#[test]
fn create_rejects_duplicate_id() {
    let mut store = Store::default();
    store.create(sample_project("p1", "D:\\projects\\alpha")).expect("create");
    let err = store
        .create(sample_project("p1", "D:\\projects\\beta"))
        .expect_err("duplicate id must be rejected");
    assert!(matches!(err, StoreError::DuplicateId { .. }), "got: {err:?}");
    assert_eq!(store.projects.len(), 1);
    assert_eq!(store.get("p1").expect("get").path, "D:\\projects\\alpha");
}

#[test]
fn update_replaces_existing_project() {
    let mut store = Store::default();
    store.create(sample_project("p1", "D:\\projects\\alpha")).expect("create");
    let mut changed = sample_project("p1", "D:\\projects\\alpha");
    changed.name = "renamed".to_string();
    changed.tags = vec![];
    store.update(changed.clone()).expect("update");
    assert_eq!(store.projects.len(), 1);
    assert_eq!(store.get("p1").expect("get"), &changed);
}

#[test]
fn update_missing_returns_not_found() {
    let mut store = Store::default();
    let err = store
        .update(sample_project("ghost", "D:\\x"))
        .expect_err("update of missing id must fail");
    assert!(matches!(err, StoreError::NotFound { .. }), "got: {err:?}");
}

#[test]
fn delete_removes_record_but_never_the_project_directory() {
    let dir = temp_dir("delete-dir");
    let project_dir = dir.join("real-project");
    std::fs::create_dir_all(&project_dir).expect("create project dir");
    std::fs::write(project_dir.join("main.py"), "print(1)").expect("seed file");

    let mut store = Store::default();
    store
        .create(sample_project("p1", project_dir.to_str().expect("utf8 path")))
        .expect("create");
    let removed = store.delete("p1").expect("delete");
    assert_eq!(removed.id, "p1");
    assert!(store.projects.is_empty());
    assert!(project_dir.exists(), "project directory must survive record deletion");
    assert!(project_dir.join("main.py").exists());
    cleanup(&dir);
}

#[test]
fn delete_missing_returns_not_found() {
    let mut store = Store::default();
    let err = store.delete("ghost").expect_err("delete of missing id must fail");
    assert!(matches!(err, StoreError::NotFound { .. }), "got: {err:?}");
}

#[test]
fn get_missing_returns_none() {
    let store = Store::default();
    assert!(store.get("ghost").is_none());
}

// ---------- Load ----------

#[test]
fn load_missing_file_returns_empty_store() {
    let dir = temp_dir("load-missing");
    let file = dir.join("projects.json");
    let store = Store::load(&file).expect("missing file should load as empty");
    assert!(store.projects.is_empty());
    cleanup(&dir);
}

#[test]
fn load_empty_file_returns_corrupted_error() {
    let dir = temp_dir("load-empty");
    let file = dir.join("projects.json");
    std::fs::write(&file, "").expect("write empty file");
    let err = Store::load(&file).expect_err("empty file must be diagnosable");
    assert!(matches!(err, StoreError::Corrupted { .. }), "got: {err:?}");
    cleanup(&dir);
}

#[test]
fn load_invalid_json_returns_corrupted_error() {
    let dir = temp_dir("load-badjson");
    let file = dir.join("projects.json");
    std::fs::write(&file, "{ not valid json !!!").expect("write");
    let err = Store::load(&file).expect_err("invalid JSON must be diagnosable");
    assert!(matches!(err, StoreError::Corrupted { .. }), "got: {err:?}");
    cleanup(&dir);
}

#[test]
fn load_wrong_version_returns_version_mismatch() {
    let dir = temp_dir("load-version");
    let file = dir.join("projects.json");
    std::fs::write(&file, r#"{ "version": 99, "projects": [] }"#).expect("write");
    let err = Store::load(&file).expect_err("unknown version must be rejected");
    assert!(
        matches!(err, StoreError::VersionMismatch { found: 99, expected: 1 }),
        "got: {err:?}"
    );
    cleanup(&dir);
}

#[test]
fn load_valid_file_returns_projects() {
    let dir = temp_dir("load-valid");
    let file = dir.join("projects.json");
    let json = r#"{
        "version": 1,
        "projects": [
            {
                "id": "p1",
                "name": "alpha",
                "path": "D:\\projects\\alpha",
                "tags": ["web"],
                "createdAt": "2026-08-25T00:00:00Z"
            }
        ]
    }"#;
    std::fs::write(&file, json).expect("write");
    let store = Store::load(&file).expect("valid file must load");
    assert_eq!(store.projects.len(), 1);
    assert_eq!(store.projects[0].id, "p1");
    assert_eq!(store.projects[0].name, "alpha");
    assert_eq!(store.projects[0].description, None);
    assert_eq!(store.projects[0].run_command, None);
    cleanup(&dir);
}
