use serde_derive::{Deserialize, Serialize};

// Data model for books and tags
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub desc: String,
    pub tags: Vec<Tag>,
    pub year: String,
    pub cover: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

// Sample book data
/* pub fn sample_books() -> Vec<Book> {
    vec![
        Book {
            id: 0,
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik and Carol Nichols".to_string(),
            tags: vec!["rust".to_string(), "programming".to_string()],
            year: "2025".to_string(),
            cover: "".to_string()
        },
        Book {
            id: 1,
            title: "The Pragmatic Programmer".to_string(),
            author: "Andrew Hunt and David Thomas".to_string(),
            tags: vec!["programming".to_string(), "pragmatism".to_string()],
            year: "2024".to_string(),
            cover: "".to_string()
        },
        Book {
            id: 2,
            title: "The C Programming Language".to_string(),
            author: "Linus Torvald".to_string(),
            tags: vec!["c".to_string(), "programming".to_string(), "linus".to_string()],
            year: "2023".to_string(),
            cover: "".to_string()
        },
    ]
} */
