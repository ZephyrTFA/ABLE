use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Book {
    id: i32,
    title: String,
    author: String,
    publication_year: i32,
    isbn: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct BookResponse {
    book: Book,
}

impl From<&Book> for BookResponse {
    fn from(book: &Book) -> Self {
        Self { book: book.clone() }
    }
}

#[derive(Serialize, Debug)]
pub struct BooksResponse {
    books: Vec<Book>,
}

impl From<Vec<Book>> for BooksResponse {
    fn from(books: Vec<Book>) -> Self {
        Self { books }
    }
}

impl Book {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn publication_year(&self) -> &i32 {
        &self.publication_year
    }

    pub fn isbn(&self) -> &String {
        &self.isbn
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn set_created_at(&mut self, created_at: DateTime<Utc>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
        self.updated_at = updated_at;
    }
}

#[test]
fn test_serialization() {
    let utc_now = Utc::now();
    let book = Book {
        id: 42,
        title: "Hitch Hiker's Guide to the Galaxy".to_string(),
        author: "Douglas Adams".to_string(),
        publication_year: 1979,
        isbn: "9780575074842".to_string(),
        created_at: utc_now,
        updated_at: utc_now,
    };

    let expected = format!(
        "{{\"id\":42,\"title\":\"Hitch Hiker's Guide to the Galaxy\",\"author\":\"Douglas Adams\",\"publication_year\":1979,\"isbn\":\"9780575074842\",\"created_at\":{},\"updated_at\":{}}}",
        serde_json::to_string(&utc_now).expect("failed to serialize datetime"),
        serde_json::to_string(&utc_now).unwrap(),
    );
    let actual = serde_json::to_string(&book).expect("failed to serialize book");
    assert_eq!(expected, actual);
}

#[test]
fn test_deserialization() {
    let utc_now = Utc::now();
    let expected = Book {
        id: 42,
        title: "Hitch Hiker's Guide to the Galaxy".to_string(),
        author: "Douglas Adams".to_string(),
        publication_year: 1979,
        isbn: "9780575074842".to_string(),
        created_at: utc_now,
        updated_at: utc_now,
    };

    let actual = serde_json::from_str(format!(
        "{{\"id\":42,\"title\":\"Hitch Hiker's Guide to the Galaxy\",\"author\":\"Douglas Adams\",\"publication_year\":1979,\"isbn\":\"9780575074842\",\"created_at\":{},\"updated_at\":{}}}",
        serde_json::to_string(&utc_now).expect("failed to serialize datetime"),
        serde_json::to_string(&utc_now).unwrap(),
    ).as_str()).expect("failed to deserialize book json");
    assert_eq!(expected, actual);
}
