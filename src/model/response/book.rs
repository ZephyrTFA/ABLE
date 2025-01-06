use chrono::{DateTime, Utc};

#[allow(dead_code)]
struct Book {
    id: u32,
    title: String,
    author: String,
    publication_year: i32,
    isbn: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
