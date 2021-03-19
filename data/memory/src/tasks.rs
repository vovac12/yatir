use crate::MemoryDatabase;
use async_trait::async_trait;
use yatir_core::prelude::*;
use yatir_folders::prelude::FoldersRepo;
use yatir_tasks::prelude::*;

#[async_trait]
impl TasksRepo for MemoryDatabase {
    async fn get(&self, id: Index) -> CoreResult<Task> {
        self.tasks
            .read()
            .await
            .get(&id)
            .map(|x| x.clone())
            .ok_or(CoreError::not_found(format!(
                "Task with id {} not exists",
                id
            )))
    }

    async fn create(&self, task: CreateTask) -> CoreResult<Task> {
        let mut tasks = self.tasks.write().await;
        let mut tasks_in_folders = self.tasks_in_folders.write().await;
        FoldersRepo::get(self, task.parent).await?;
        let id = tasks
            .keys()
            .max()
            .unwrap_or(&(tasks.len() as Index))
            .wrapping_add(1);
        let task = Task {
            id,
            name: task.name,
            description: task.description.unwrap_or_default(),
            parent: task.parent,
        };

        tasks_in_folders
            .entry(task.parent)
            .or_insert(Default::default())
            .push(id);
        tasks.insert(id, task.clone());
        Ok(task)
    }

    async fn update(&self, task: UpdateTask) -> CoreResult<Task> {
        let mut tasks = self.tasks.write().await;
        let actual_task = tasks.get_mut(&task.id).ok_or(CoreError::not_found(format!(
            "Folder with id {} not exists",
            &task.id
        )))?;
        match task.parent {
            Some(parent) if parent != actual_task.parent => {
                let mut tasks_in_folders = self.tasks_in_folders.write().await;
                let childs =
                    tasks_in_folders
                        .get_mut(&actual_task.parent)
                        .ok_or(CoreError::not_found(format!(
                            "Folder with id {} not exists",
                            &actual_task.parent
                        )))?;
                let mut idx = None;
                for (i, &child) in childs.iter().enumerate() {
                    if child == task.id {
                        idx = Some(i);
                    }
                }
                if let Some(idx) = idx {
                    childs.remove(idx);
                }
            }
            _ => {}
        };
        if let Some(name) = task.name {
            actual_task.name = name;
        }

        if let Some(description) = task.description {
            actual_task.description = description;
        }
        Ok(actual_task.clone())
    }
    async fn get_by_parent(&self, id: Index) -> CoreResult<Paging<Task>> {
        let tasks = self.tasks.read().await;
        self.tasks_in_folders
            .read()
            .await
            .get(&id)
            .map(|childs| {
                Paging::new(
                    childs.len() as Index,
                    childs
                        .iter()
                        .flat_map(|child| tasks.get(child).map(|x| x.clone()))
                        .collect(),
                )
            })
            .ok_or(CoreError::not_found(format!(
                "Parent with id {} not exists",
                id
            )))
    }
}
