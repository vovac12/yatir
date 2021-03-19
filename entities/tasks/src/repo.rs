use crate::Task;
use async_trait::async_trait;
use yatir_core::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTask {
    pub parent: Index,
    #[serde(default = "crate::default_todo_name")]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTask {
    pub id: Index,
    pub parent: Option<Index>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
pub trait TasksRepo: Send + Sync {
    async fn get(&self, id: Index) -> CoreResult<Task>;
    async fn create(&self, folder: CreateTask) -> CoreResult<Task>;
    async fn update(&self, folder: UpdateTask) -> CoreResult<Task>;
}
