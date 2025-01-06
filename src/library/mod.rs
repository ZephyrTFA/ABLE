use std::{
    collections::{hash_map::Values, HashMap},
    fmt::Display,
};

use chrono::Utc;
use log::{trace, warn};

use crate::model::response::book::Book;

#[derive(Default, Debug)]
pub struct Library {
    books: HashMap<String, Book>,
}

pub enum LibraryErrorStatus {
    IsbnExists,
    IsbnMismatch,
    IdNotFound,
}

impl Display for LibraryErrorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IsbnExists => f.write_str("isbn exists"),
            Self::IsbnMismatch => f.write_str("isbn mismatch"),
            Self::IdNotFound => f.write_str("id not found"),
        }
    }
}

impl Library {
    pub fn sync(&self) {
        trace!("syncing");
        todo!()
    }

    pub fn add_book(&mut self, book: Book) -> Result<(), LibraryErrorStatus> {
        let isbn = book.isbn();
        if self.books.contains_key(isbn) {
            warn!("refusing to add book with conflicting isbn: {isbn}");
            return Err(LibraryErrorStatus::IsbnExists);
        }

        trace!("adding book with isbn: {}", isbn);
        self.books.insert(isbn.to_string(), book);
        self.sync();
        Ok(())
    }

    pub fn get_books(&self) -> Values<String, Book> {
        self.books.values()
    }

    pub fn get_book_by_isbn(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }

    pub fn get_book_by_id(&self, id: &i32) -> Option<&Book> {
        self.get_books().find(|book| book.id() == id)
    }

    pub fn update_book(&mut self, mut book: Book) -> Result<(), LibraryErrorStatus> {
        let old_book = self.get_book_by_id(book.id());
        if old_book.is_none() {
            warn!(
                "refusing to update book id not found in library: {}",
                book.id()
            );
            return Err(LibraryErrorStatus::IdNotFound);
        }
        let old_book = old_book.unwrap();

        let isbn = book.isbn().clone();
        let old_isbn = old_book.isbn();
        if &isbn != old_isbn {
            warn!(
                "refusing to update book with isbn mismatch: {} != {}",
                isbn, old_isbn
            );
            return Err(LibraryErrorStatus::IsbnMismatch);
        }

        trace!("updating book with isbn: {isbn}");
        // They can pass whatever timestamp they want; we overwrite it with what the actual time of the transaction.
        book.set_created_at(*old_book.created_at());
        book.set_updated_at(Utc::now());
        self.books.insert(isbn, book);
        self.sync();
        Ok(())
    }

    pub fn drop_book(&mut self, id: &i32) -> Result<(), LibraryErrorStatus> {
        let book = self.get_book_by_id(id);
        if book.is_none() {
            warn!("failed to drop book id not found in library: {}", id);
            return Err(LibraryErrorStatus::IdNotFound);
        }

        let isbn = book.unwrap().isbn().clone();
        trace!("dropping book: {isbn}");
        self.books.remove(&isbn);
        self.sync();
        Ok(())
    }
}
