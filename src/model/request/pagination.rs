use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Pagination {
    pub fn page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    pub fn per_page(&self) -> usize {
        self.per_page.unwrap_or(50)
    }
}
