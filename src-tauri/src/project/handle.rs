//! 进程内共享的 JSON 句柄（A1）：把「读-改-写」串行化到单一写者，
//! 消除并发命令之间的丢失更新。首次访问时从磁盘加载，之后缓存在内存中。

use super::settings::Settings;
use super::store::Store;
use super::types::StoreError;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

/// 可被 [`JsonHandle`] 持久化的类型：提供加载与原子保存。
pub trait Persisted: Sized {
    fn load(path: &Path) -> Result<Self, StoreError>;
    fn save(&self, path: &Path) -> Result<(), StoreError>;
}

impl Persisted for Store {
    fn load(path: &Path) -> Result<Self, StoreError> {
        Store::load(path)
    }

    fn save(&self, path: &Path) -> Result<(), StoreError> {
        Store::save(self, path)
    }
}

impl Persisted for Settings {
    fn load(path: &Path) -> Result<Self, StoreError> {
        Settings::load(path)
    }

    fn save(&self, path: &Path) -> Result<(), StoreError> {
        Settings::save(self, path)
    }
}

/// 单一写者句柄：所有读写经同一把锁；首次访问时加载并缓存。
///
/// 锁中毒时取回内部值继续使用（一次 panic 不应让整个应用不可用）。
pub struct JsonHandle<T> {
    path: PathBuf,
    inner: Mutex<Option<T>>,
}

impl<T: Persisted + Clone> JsonHandle<T> {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            inner: Mutex::new(None),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Option<T>> {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// 只读访问：确保已加载后以 `&T` 调用 `f`。
    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R, StoreError> {
        let mut guard = self.lock();
        ensure_loaded(&mut guard, &self.path)?;
        let value = guard.as_ref().expect("just loaded");
        Ok(f(value))
    }

    /// 写访问：确保已加载 → 在副本上调用 `f` → 保存成功后提交回内存。
    /// 闭包或保存任一步失败，内存状态保持不变（事务语义）。
    pub fn write<R>(&self, f: impl FnOnce(&mut T) -> Result<R, StoreError>) -> Result<R, StoreError> {
        let mut guard = self.lock();
        ensure_loaded(&mut guard, &self.path)?;
        let mut draft = guard.as_ref().expect("just loaded").clone();
        let output = f(&mut draft)?;
        draft.save(&self.path)?;
        *guard = Some(draft);
        Ok(output)
    }
}

fn ensure_loaded<T: Persisted>(slot: &mut Option<T>, path: &Path) -> Result<(), StoreError> {
    if slot.is_none() {
        *slot = Some(T::load(path)?);
    }
    Ok(())
}
