use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Book {
    id: u32,
    title: String,
    author: String,
    publication_year: i32,
    isbn: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
