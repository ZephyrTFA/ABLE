use serde::{Deserialize, Serialize};

use crate::orm::book::Book;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BooksResponse {
    pub books: Vec<Book>,
}
