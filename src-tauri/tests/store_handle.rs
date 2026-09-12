//! JSON 句柄的并发集成测试（ADR 0006）：证明单一写者消除了丢失更新。

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use windy_project_mgr_lib::project::handle::JsonHandle;
use windy_project_mgr_lib::project::{Project, Store};

static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "windy-handle-{}-{}-{n}",
        label,
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn sample_project(index: usize) -> Project {
    Project {
        id: format!("p{index}"),
        name: format!("name-{index}"),
        path: format!("D:\\projects\\p{index}"),
        description: None,
        tags: vec![],
        run_command: None,
        build_command: None,
        created_at: "2026-09-11T00:00:00Z".to_string(),
    }
}

#[test]
fn concurrent_writes_do_not_lose_updates() {
    let dir = temp_dir("concurrent");
    let file = dir.join("projects.json");
    let handle = Arc::new(JsonHandle::<Store>::new(file.clone()));

    let threads: Vec<_> = (0..16)
        .map(|i| {
            let handle = Arc::clone(&handle);
            std::thread::spawn(move || {
                handle
                    .write(|store| {
                        store.projects.push(sample_project(i));
                        Ok(())
                    })
                    .expect("write must succeed");
            })
        })
        .collect();

    for thread in threads {
        thread.join().expect("thread must not panic");
    }

    let loaded = Store::load(&file).expect("reload persisted store");
    assert_eq!(
        loaded.projects.len(),
        16,
        "every concurrent write must survive"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn failed_write_leaves_persisted_and_cached_state_unchanged() {
    let dir = temp_dir("rollback");
    let file = dir.join("projects.json");
    let handle = JsonHandle::<Store>::new(file.clone());

    handle
        .write(|store| {
            store.projects.push(sample_project(0));
            Ok(())
        })
        .expect("initial write");

    // 闭包返回错误：既不落盘，也不提交回内存。
    let outcome: Result<(), _> = handle.write(|store| {
        store.projects.push(sample_project(1));
        Err(windy_project_mgr_lib::project::StoreError::Validation {
            detail: "boom".to_string(),
        })
    });
    assert!(outcome.is_err());

    let cached = handle.read(|store| store.projects.len()).expect("read");
    assert_eq!(cached, 1, "cached state must not include the failed mutation");

    let loaded = Store::load(&file).expect("reload");
    assert_eq!(loaded.projects.len(), 1, "disk must not include the failed mutation");
    let _ = std::fs::remove_dir_all(&dir);
}
