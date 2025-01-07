use crate::orm::book::Book;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookResponse {
    pub book: Book,
}

#[cfg(test)]
use chrono::Utc;

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
