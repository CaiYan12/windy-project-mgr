//! 项目记录与存储层：`Project` 模型与 `projects.json` / `settings.json` 读写。

pub mod dedup;
pub mod handle;
pub mod settings;
pub mod store;
pub mod types;

pub use handle::JsonHandle;
pub use settings::Settings;
pub use store::Store;
pub use types::{Project, StoreError};
