extern crate serde;

pub mod repo;

use yatir_core::prelude::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToDo {
    pub parent: Index,
    pub id: Index,
    pub name: String,
    pub description: String,
}

pub(crate) fn default_todo_name() -> String {
    "New folder".to_string()
}

pub mod prelude {
    pub use super::{
        repo::{CreateToDo, ToDoRepo, UpdateToDo},
        ToDo,
    };
}
