use serde::Serialize;

use super::book::Book;

#[derive(Serialize, Debug)]
pub struct GetBooksResponse<'a> {
    books: Vec<&'a Book>,
}

impl<'a> GetBooksResponse<'a> {
    pub fn new(books: &'a [Book]) -> Self {
        Self {
            books: books.iter().collect(),
        }
    }
}
