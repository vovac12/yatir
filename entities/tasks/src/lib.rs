extern crate serde;

pub mod repo;

use yatir_core::prelude::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Task {
    pub parent: Index,
    pub id: Index,
    pub name: String,
    pub description: String,
}

pub(crate) fn default_task_name() -> String {
    "New folder".to_string()
}

pub mod prelude {
    pub use super::{
        repo::{CreateTask, TasksRepo, UpdateTask},
        Task,
    };
}
