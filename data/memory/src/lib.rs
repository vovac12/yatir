mod folders;
mod tasks;
use async_std::sync::RwLock;
use std::collections::HashMap;
use yatir_core::prelude::*;
use yatir_folders::prelude::*;
use yatir_tasks::prelude::*;

#[derive(Debug, Default)]
pub struct MemoryDatabase {
    folders: RwLock<HashMap<Index, Folder>>,
    folders_index: RwLock<HashMap<Index, Vec<Index>>>,
    tasks: RwLock<HashMap<Index, Task>>,
    tasks_in_folders: RwLock<HashMap<Index, Vec<Index>>>,
}

impl MemoryDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
