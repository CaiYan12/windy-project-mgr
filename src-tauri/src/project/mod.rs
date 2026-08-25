//! 项目记录与存储层：`Project` 模型与 `projects.json` / `settings.json` 读写。

pub mod settings;
pub mod store;
pub mod types;

pub use settings::Settings;
pub use store::Store;
pub use types::{Project, StoreError};
