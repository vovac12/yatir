pub mod error;

use serde::{Deserialize, Serialize};

pub type Index = i64;

const DEFAULT_OFFSET: Index = 0;
const DEFAULT_LIMIT: Index = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOptions {
    offset: Option<Index>,
    limit: Option<Index>,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            offset: Some(DEFAULT_OFFSET),
            limit: Some(DEFAULT_LIMIT),
        }
    }
}

impl QueryOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(&self) -> Index {
        self.limit.unwrap_or(DEFAULT_LIMIT)
    }

    pub fn offset(&self) -> Index {
        self.offset.unwrap_or(DEFAULT_OFFSET)
    }

    pub fn with_offset(self, offset: Option<Index>) -> Self {
        Self { offset, ..self }
    }

    pub fn with_limit(self, limit: Option<Index>) -> Self {
        Self { limit, ..self }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paging<T> {
    total: Index,
    items: Vec<T>,
}

impl<T> Paging<T> {
    pub fn new(total: Index, items: Vec<T>) -> Self {
        Self { total, items }
    }

    pub fn total(&self) -> Index {
        self.total
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn into_inner(self) -> Vec<T> {
        self.items
    }
}

pub type CoreResult<T> = Result<T, error::CoreError>;

pub mod prelude {
    pub use super::{error::CoreError, CoreResult, Index, Paging, QueryOptions};
    pub use serde::{Deserialize, Serialize};
}
