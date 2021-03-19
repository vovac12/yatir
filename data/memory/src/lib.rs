use async_std::sync::RwLock;
use async_trait::async_trait;
use std::collections::HashMap;
use yatir_core::prelude::*;
use yatir_folders::prelude::*;

#[derive(Debug, Default)]
pub struct MemoryDatabase {
    folders: RwLock<HashMap<Index, Folder>>,
    folders_index: RwLock<HashMap<Index, Vec<Index>>>,
}

impl MemoryDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl FoldersRepo for MemoryDatabase {
    async fn get(&self, id: Index) -> CoreResult<Folder> {
        self.folders
            .read()
            .await
            .get(&id)
            .map(|x| x.clone())
            .ok_or(CoreError::not_found(format!(
                "Folder with id {} not exists",
                id
            )))
    }

    async fn get_children(&self, id: Index) -> CoreResult<Paging<Folder>> {
        let folders = self.folders.read().await;
        self.folders_index
            .read()
            .await
            .get(&id)
            .map(|childs| {
                Paging::new(
                    childs.len() as Index,
                    childs
                        .iter()
                        .flat_map(|child| folders.get(child).map(|x| x.clone()))
                        .collect(),
                )
            })
            .ok_or(CoreError::not_found(format!(
                "Folder with id {} not exists",
                id
            )))
    }

    async fn create(&self, folder: CreateFolder) -> CoreResult<Folder> {
        let mut folders = self.folders.write().await;
        let mut folders_index = self.folders_index.write().await;
        let id = folders
            .keys()
            .max()
            .unwrap_or(&(folders.len() as Index))
            .wrapping_add(1);
        if let Some(parent) = folder.parent {
            folders_index
                .get_mut(&parent)
                .ok_or_else(|| {
                    CoreError::not_found(format!("Parent folder with id {} not found", parent))
                })?
                .push(id);
        }
        let folder = Folder {
            id,
            name: folder.name,
            description: folder.description.unwrap_or_default(),
            parent: folder.parent.unwrap_or(0),
        };

        folders.insert(id, folder.clone());
        folders_index.insert(id, Default::default());
        Ok(folder)
    }

    async fn update(&self, folder: UpdateFolder) -> CoreResult<Folder> {
        let mut folders = self.folders.write().await;
        let actual_folder = folders
            .get_mut(&folder.id)
            .ok_or(CoreError::not_found(format!(
                "Folder with id {} not exists",
                &folder.id
            )))?;
        match folder.parent {
            Some(parent) if parent != actual_folder.parent => {
                let mut folders_index = self.folders_index.write().await;
                let childs =
                    folders_index
                        .get_mut(&actual_folder.parent)
                        .ok_or(CoreError::not_found(format!(
                            "Folder with id {} not exists",
                            &actual_folder.parent
                        )))?;
                let mut idx = None;
                for (i, &child) in childs.iter().enumerate() {
                    if child == folder.id {
                        idx = Some(i);
                    }
                }
                if let Some(idx) = idx {
                    childs.remove(idx);
                }
            }
            _ => {}
        };
        if let Some(name) = folder.name {
            actual_folder.name = name;
        }

        if let Some(description) = folder.description {
            actual_folder.description = description;
        }
        Ok(actual_folder.clone())
    }
}
