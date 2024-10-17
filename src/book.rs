use serde_derive::{Deserialize, Serialize};

// Data model for books and tags
#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub tags: Vec<String>,
}

// Sample book data
pub fn sample_books() -> Vec<Book> {
    vec![
        Book {
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik and Carol Nichols".to_string(),
            tags: vec!["rust".to_string(), "programming".to_string()],
        },
        Book {
            title: "The Pragmatic Programmer".to_string(),
            author: "Andrew Hunt and David Thomas".to_string(),
            tags: vec!["programming".to_string(), "pragmatism".to_string()],
        },
        Book {
            title: "The C Programming Language".to_string(),
            author: "Linus Torvald".to_string(),
            tags: vec!["c".to_string(), "programming".to_string(), "linus".to_string()],
        },
    ]
}
