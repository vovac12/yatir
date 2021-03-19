use crate::Folder;
use async_trait::async_trait;
use yatir_core::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFolder {
    pub parent: Option<Index>,
    #[serde(default = "crate::default_folder_name")]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFolder {
    pub id: Index,
    pub parent: Option<Index>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
pub trait FoldersRepo: Send + Sync {
    async fn get(&self, id: Index) -> CoreResult<Folder>;
    async fn get_children(&self, id: Index) -> CoreResult<Paging<Folder>>;
    async fn create(&self, folder: CreateFolder) -> CoreResult<Folder>;
    async fn update(&self, folder: UpdateFolder) -> CoreResult<Folder>;
}
