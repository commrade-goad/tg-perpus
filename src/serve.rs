use crate::search::s_search_book;
use crate::sql::*;
use axum::{extract::Query, response::IntoResponse, Json};
use serde_derive::Deserialize;

// `/get_tag?s={startat}&e={endat}`
#[derive(Deserialize)]
pub struct GetTagParams {
    f: i32,
    r: i32,
    sort: Option<String>,
}
pub async fn get_tag(Query(params): Query<GetTagParams>) -> impl IntoResponse {
    let sort_parse: String = params.sort.as_deref().unwrap_or("ASC").to_string();
    let sorting_mode: String;
    if !is_valid_sort(&sort_parse){
        sorting_mode = "ASC".to_string();
    } else {
        sorting_mode = sort_parse;
    }
    match sql_read_tags(params.f, params.r, sorting_mode).await {
        Ok(val) => {
            if val.len() <= 0 {
                return Json(None);
            }
            return Json(Some(val));
        }
        Err(_) => return Json(None),
    }
}

// `/search?q={query}`
#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
    sort: Option<String>,
}
pub async fn search_book(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let sort_parse: String = params.sort.as_deref().unwrap_or("ASC").to_string();
    let sorting_mode: String;
    if !is_valid_sort(&sort_parse){
        sorting_mode = "ASC".to_string();
    } else {
        sorting_mode = sort_parse;
    }
    let res = s_search_book(&params.q, sorting_mode).await;
    if res.len() <= 0 {
        return Json(None);
    }
    Json(Some(res))
}

// `/get_book_info?id={id}`
#[derive(Deserialize)]
pub struct BookInfoParams {
    id: i32,
    sort: Option<String>,
}
pub async fn get_book_info(Query(params): Query<BookInfoParams>) -> impl IntoResponse {
    let sort_parse: String = params.sort.as_deref().unwrap_or("ASC").to_string();
    let sorting_mode: String;
    if !is_valid_sort(&sort_parse){
        sorting_mode = "ASC".to_string();
    } else {
        sorting_mode = sort_parse;
    }
    match sql_get_book_info(params.id, sorting_mode).await {
        Ok(val) => Json(Some(val)),
        Err(_) => Json(None),
    }
}

// `/get_book_from_tag?id={tag}&f={from}&r={range}`
#[derive(Deserialize)]
pub struct GetBookListFromTagParams {
    f: i32,
    r: i32,
    id: i32,
    sort: Option<String>,
}
pub async fn get_book_from_tag(
    Query(params): Query<GetBookListFromTagParams>,
) -> impl IntoResponse {
    let sort_parse: String = params.sort.as_deref().unwrap_or("ASC").to_string();
    let sorting_mode: String;
    if !is_valid_sort(&sort_parse){
        sorting_mode = "ASC".to_string();
    } else {
        sorting_mode = sort_parse;
    }
    match sql_read_specified_tagged_book(params.id, params.r, params.f, sorting_mode).await {
        Ok(val) => {
            if val.len() <= 0 {
                return Json(None);
            }
            return Json(Some(val));
        }
        Err(_) => return Json(None),
    }
}

// `/add_book?t={title}&a={author}&tg={tag} {tag}&im={path}`
#[derive(Deserialize)]
pub struct AddBookParams {
    title: String,
    author: String,
    tagid: String,
    imgp: String,
    year: String,
    desc: String
}
pub async fn add_new_book(Query(params): Query<AddBookParams>) -> impl IntoResponse {
    match sql_add_new_book(&params.title, &params.author, &params.tagid, &params.year, &params.desc, &params.imgp).await {
        Ok(_) => Json(Some("SUCCESS")),
        Err(_) => return Json(None),
    }
}

// `/add_tag?n={name}&im={image blob}`
#[derive(Deserialize)]
pub struct AddTagParams {
    name: String,
    imgp: String,
}
pub async fn add_new_tag(Query(params): Query<AddTagParams>) -> impl IntoResponse {
    match sql_add_new_tag(&params.name, &params.imgp).await {
        Ok(_) => Json(Some("SUCCESS")),
        Err(_) => return Json(None),
    }
}

// `/del_book?id={book_id}`
#[derive(Deserialize)]
pub struct DelBookParams {
    id: i32,
}
pub async fn del_book(Query(params): Query<DelBookParams>) -> impl IntoResponse {
    match sql_del_book_from_id(params.id).await {
        Ok(_) => Json(Some("SUCCESS")),
        Err(_) => return Json(None),
    }
}

// `/del_tag?id={tag_id}`
#[derive(Deserialize)]
pub struct DelTagParams {
    id: i32,
}
pub async fn del_tag(Query(params): Query<DelTagParams>) -> impl IntoResponse {
    match sql_del_tag_from_id(params.id).await {
        Ok(_) => Json(Some("SUCCESS")),
        Err(_) => return Json(None),
    }
}
