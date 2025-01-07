use std::{collections::HashMap, fmt::Display, sync::Arc};

use chrono::Utc;
use log::{info, trace, warn};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    TryIntoModel,
};
use tokio::sync::Mutex;

use crate::orm::book::{self, Book};

#[derive(Debug, Clone, Default)]
pub struct Library {
    books: Arc<Mutex<HashMap<String, Book>>>,
}

#[derive(Debug)]
pub enum LibraryErrorStatus {
    IsbnExists,
    IsbnMismatch,
    IdNotFound,
    DatabaseError,
}

impl Display for LibraryErrorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IsbnExists => f.write_str("isbn exists"),
            Self::IsbnMismatch => f.write_str("isbn mismatch"),
            Self::IdNotFound => f.write_str("id not found"),
            Self::DatabaseError => f.write_str("database error"),
        }
    }
}

impl Library {
    pub async fn full_sync(
        &mut self,
        database: &DatabaseConnection,
    ) -> Result<(), LibraryErrorStatus> {
        info!("Syncronizing library.");

        let db_result = book::Entity::find().all(database).await;
        if let Err(error) = db_result {
            warn!("failed to add book: {}", error.to_string());
            return Err(LibraryErrorStatus::DatabaseError);
        }
        let db_books = db_result.unwrap();
        let db_books_as_isbn = db_books.iter().map(|book| &book.isbn).collect::<Vec<_>>();

        let books = self.books.lock().await;
        let missing = books
            .iter()
            .filter(|(isbn, _)| !db_books_as_isbn.contains(isbn))
            .map(|(_, book)| book.clone().into_active_model())
            .collect::<Vec<_>>();
        drop(books);

        let mut books = db_books
            .into_iter()
            .map(|book| (book.isbn.clone(), book))
            .collect::<HashMap<String, Book>>();
        for book in &missing {
            let book = book
                .clone()
                .try_into_model()
                .expect("failed to convert a known good model to a model.");
            books.insert(book.isbn.clone(), book as Book);
        }

        if !missing.is_empty() {
            let db_result = book::Entity::insert_many(missing).exec(database).await;
            if let Err(error) = db_result {
                warn!("failed to sync library: {}", error.to_string());
                return Err(LibraryErrorStatus::DatabaseError);
            }
        }
        Ok(())
    }

    pub async fn add_book(
        &mut self,
        book: Book,
        database: &DatabaseConnection,
    ) -> Result<(), LibraryErrorStatus> {
        let isbn = &book.isbn;
        let mut books = self.books.lock().await;
        if books.contains_key(isbn) {
            warn!("refusing to add book with conflicting isbn: {isbn}");
            return Err(LibraryErrorStatus::IsbnExists);
        }

        trace!("inserting to db");
        let db_result = book.clone().into_active_model().insert(database).await;
        if let Err(error) = db_result {
            warn!("failed to add book: {}", error.to_string());
            return Err(LibraryErrorStatus::DatabaseError);
        }

        trace!("inserting to local cache");
        books.insert(isbn.to_string(), book);
        Ok(())
    }

    pub async fn get_books(
        &mut self,
        database: &DatabaseConnection,
    ) -> Result<Vec<Book>, LibraryErrorStatus> {
        self.full_sync(database).await?;
        Ok(self.books.lock().await.values().cloned().collect())
    }

    pub async fn get_book_by_isbn(
        &self,
        isbn: &str,
        database: &DatabaseConnection,
    ) -> Result<Book, LibraryErrorStatus> {
        let books = self.books.lock().await;
        let entry = books.get(isbn);
        if let Some(book) = entry {
            return Ok(book.clone());
        }

        trace!("fetching db entry");
        let db_result = book::Entity::find()
            .filter(book::Column::Isbn.eq(isbn))
            .one(database)
            .await;
        if let Err(error) = db_result {
            warn!("failed to fetch from db: {}", error.to_string());
            return Err(LibraryErrorStatus::DatabaseError);
        }

        Ok(db_result.unwrap().unwrap())
    }

    pub async fn get_book_by_id(
        &mut self,
        id: i32,
        database: &DatabaseConnection,
    ) -> Result<Book, LibraryErrorStatus> {
        let entry = self
            .get_books(database)
            .await?
            .into_iter()
            .find(|book| book.id == id);
        if let Some(book) = entry {
            return Ok(book.clone());
        }

        trace!("fetching db entry");
        let db_result = book::Entity::find_by_id(id).one(database).await;
        if let Err(error) = db_result {
            warn!("failed to fetch from db: {}", error.to_string());
            return Err(LibraryErrorStatus::DatabaseError);
        }
        let book = db_result.unwrap();
        if book.is_none() {
            return Err(LibraryErrorStatus::IdNotFound);
        }

        // found it in db but not local cache. add it.
        let book = book.unwrap();
        self.books
            .lock()
            .await
            .insert(book.isbn.clone(), book.clone());

        Ok(book)
    }

    pub async fn update_book(
        &mut self,
        mut book: Book,
        database: &DatabaseConnection,
    ) -> Result<(), LibraryErrorStatus> {
        let old_book = self.get_book_by_id(book.id, database).await?;

        // They can pass whatever timestamp they want; we overwrite it with what the actual time of the transaction.
        book.created_at = old_book.created_at;
        book.updated_at = Utc::now();

        trace!("updating db entry");
        let db_result = book.clone().into_active_model().insert(database).await;
        if let Err(error) = db_result {
            warn!("failed to update db: {}", error.to_string());
            return Err(LibraryErrorStatus::DatabaseError);
        }

        trace!("updating local cache");
        let mut books = self.books.lock().await;
        if book.isbn != old_book.isbn {
            books.remove(&old_book.isbn);
        }
        books.insert(book.isbn.to_string(), book);
        Ok(())
    }

    pub async fn drop_book(
        &mut self,
        id: i32,
        database: &DatabaseConnection,
    ) -> Result<(), LibraryErrorStatus> {
        let db_result = book::Entity::delete_by_id(id).exec(database).await;
        if let Err(error) = db_result {
            warn!("failed to drop book: {}", error.to_string());
            return Err(LibraryErrorStatus::DatabaseError);
        }

        if db_result.unwrap().rows_affected == 0 {
            return Err(LibraryErrorStatus::IdNotFound);
        }

        let book = self.get_book_by_id(id, database).await;
        if book.is_err() {
            warn!("failed to drop book id not found in cache: {}", id);
            return Err(LibraryErrorStatus::IdNotFound);
        }

        let isbn = book.unwrap().isbn.clone();
        trace!("dropping book from cache: {isbn}");
        self.books.lock().await.remove(&isbn);
        Ok(())
    }
}
