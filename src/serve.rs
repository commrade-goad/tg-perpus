use crate::book::*;
use crate::search::*;
use serde_derive::Deserialize;
use axum::{extract::Query, response::IntoResponse, Json};

#[derive(Deserialize)]
pub struct GetTagParams {
    s: i32,
    e: i32,
}
// Handler for `/get_tag?s={startat}&e={endat}`
pub async fn get_tag(Query(params): Query<GetTagParams>) -> impl IntoResponse {
    let books = sample_books();
    let mut tags: Vec<String> = books.iter().flat_map(|book| book.tags.clone()).collect();
    tags.sort();
    tags.dedup();

    Json(tags)
}

// Query parameters for `/search?q={query}`
#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

// Handler for `/search?q={query}`
pub async fn search(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let books = sample_books();
    Json(books)
}

