//! Model Layer
//!
//! Design:
//!
//! - The Model layer normatlizes the application's data type
//!   structrues and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g, `Task`, `Project`).
//!   (`Bmc` is short for Banckend Model Controller).
//! - In framewords like Axum, Tauri, `ModelManager` ar typically
//!   used as App State.
//! - ModelManager are designed to be pass as an arbument
//!   to all Model Controllers functions.

// region:    --- Modules

mod base;
mod error;
mod store;
pub mod task;

use store::{new_db_pool, Db};

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        // FIXME - TBC
        Ok(ModelManager { db })
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
