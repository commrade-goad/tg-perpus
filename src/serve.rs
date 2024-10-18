use crate::book::*;
use crate::search::*;
use serde_derive::Deserialize;
use axum::{extract::Query, response::IntoResponse, Json};

// `/get_tag?s={startat}&e={endat}`
#[derive(Deserialize)]
pub struct GetTagParams {
    s: i32,
    e: i32,
}
pub async fn get_tag(Query(params): Query<GetTagParams>) -> impl IntoResponse {
    Json("NOT IMPLEMENTED YET")
}

// `/search?q={query}`
#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}
pub async fn search_book(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let res = s_search_book(&params.q);
    Json(res)
}

// `/get_book_info?id={id}`
#[derive(Deserialize)]
pub struct BookInfoParams{
    id: String,
}
pub async fn get_book_info(Query(params): Query<BookInfoParams>) -> impl IntoResponse {
    Json("NOT IMPLEMENTED YET")
}

// `/get_book_with_tags?t={tag}`
#[derive(Deserialize)]
pub struct GetBookListFromTagParams{
    t: String,
}
pub async fn get_book_from_tag(Query(params): Query<GetBookListFromTagParams>) -> impl IntoResponse {
    Json("NOT IMPLEMENTED YET")
}

// `/add_book?t={title}&a={author}&tg={tag} {tag}&im={image blob}`
#[derive(Deserialize)]
pub struct AddBookParams{
    t: String,
    a: String,
    tg: String,
    im: String,
}
pub async fn add_new_book(Query(params): Query<AddBookParams>) -> impl IntoResponse {
    Json("NOT IMPLEMENTED YET")
}

// `/add_tag?n={name}&im={image blob}`
#[derive(Deserialize)]
pub struct AddTagParams{
    n: String,
    im: String,
}
pub async fn add_new_tag(Query(params): Query<AddTagParams>) -> impl IntoResponse {
    Json("NOT IMPLEMENTED YET")
}

// `/del_book?id={book_id}`
#[derive(Deserialize)]
pub struct DelBookParams{
    id: String,
}
pub async fn del_new_book(Query(params): Query<DelBookParams>) -> impl IntoResponse {
    Json("NOT IMPLEMENTED YET")
}

// `/del_tag?id={tag_id}`
#[derive(Deserialize)]
pub struct DelTagParams{
    id: String,
}
pub async fn del_new_tag(Query(params): Query<DelTagParams>) -> impl IntoResponse {
    Json("NOT IMPLEMENTED YET")
}
