use yatir_data_memory::MemoryDatabase;
use yatir_folders::prelude::*;
pub struct AppState {
    repo: MemoryDatabase,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            repo: MemoryDatabase::new(),
        }
    }

    pub fn folders_repo(&self) -> Option<&dyn FoldersRepo> {
        Some(&self.repo)
    }
}
