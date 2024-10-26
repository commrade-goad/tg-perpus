use crate::search::s_search_book;
use crate::sql::*;
use axum::{extract::Query, response::IntoResponse, Json};
use serde_derive::Deserialize;

// `/get_tag?s={startat}&e={endat}`
#[derive(Deserialize)]
pub struct GetTagParams {
    f: i32,
    r: i32,
}
pub async fn get_tag(Query(params): Query<GetTagParams>) -> impl IntoResponse {
    match sql_read_tags(params.f, params.r).await {
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
}
pub async fn search_book(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let res = s_search_book(&params.q).await;
    if res.0.len() <= 0 {
        return Json(None);
    }
    Json(Some(res))
}

// `/get_book_info?id={id}`
#[derive(Deserialize)]
pub struct BookInfoParams {
    id: String,
}
pub async fn get_book_info(Query(params): Query<BookInfoParams>) -> impl IntoResponse {
    let convert: i32 = params.id.trim().parse().unwrap_or(-1);
    if convert <= -1 {
        return Json(None);
    }
    match sql_get_book_info(convert).await {
        Ok(val) => Json(Some(val)),
        Err(_) => Json(None),
    }
}

// `/get_book_from_tag?id={tag}&f={from}&r={range}`
#[derive(Deserialize)]
pub struct GetBookListFromTagParams {
    f: i32,
    r: i32,
    id: String,
}
pub async fn get_book_from_tag(
    Query(params): Query<GetBookListFromTagParams>,
) -> impl IntoResponse {
    let convert: i32 = params.id.trim().parse().unwrap_or(-1);
    if convert <= -1 {
        return Json(None);
    }
    match sql_read_specified_tagged_book(convert, params.r, params.f).await {
        Ok(val) => {
            if val.len() <= 0 {
                return Json(None);
            }
            return Json(Some(val));
        }
        Err(_) => return Json(None),
    }
}

// `/add_book?t={title}&a={author}&tg={tag} {tag}&im={image blob}`
#[derive(Deserialize)]
pub struct AddBookParams {
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
pub struct AddTagParams {
    n: String,
    im: String,
}
pub async fn add_new_tag(Query(params): Query<AddTagParams>) -> impl IntoResponse {
    Json("NOT IMPLEMENTED YET")
}

// `/del_book?id={book_id}`
#[derive(Deserialize)]
pub struct DelBookParams {
    id: String,
}
pub async fn del_book(Query(params): Query<DelBookParams>) -> impl IntoResponse {
    let convert: i32 = params.id.trim().parse().unwrap_or(-1);
    if convert == -1 {
        return Json(None);
    }
    match sql_del_book_from_id(convert).await {
        Ok(val) => Json(Some(val)),
        Err(_) => return Json(None),
    }
}

// `/del_tag?id={tag_id}`
#[derive(Deserialize)]
pub struct DelTagParams {
    id: String,
}
pub async fn del_tag(Query(params): Query<DelTagParams>) -> impl IntoResponse {
    let convert: i32 = params.id.trim().parse().unwrap_or(-1);
    if convert == -1 {
        return Json(None);
    }
    match sql_del_tag_from_id(convert).await {
        Ok(val) => Json(Some(val)),
        Err(_) => return Json(None),
    }
}
