use crate::ToDo;
use async_trait::async_trait;
use yatir_core::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateToDo {
    pub parent: Index,
    #[serde(default = "crate::default_todo_name")]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateToDo {
    pub id: Index,
    pub parent: Option<Index>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
pub trait ToDoRepo: Send + Sync {
    async fn get(&self, id: Index) -> CoreResult<ToDo>;
    async fn create(&self, folder: CreateToDo) -> CoreResult<ToDo>;
    async fn update(&self, folder: UpdateToDo) -> CoreResult<ToDo>;
}
