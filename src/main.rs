mod book;
mod serve;
mod search;
mod sql;
use axum::{routing::get, Router};
use serve::*;
use sql::set_sql_path_val;
use std::env;

struct ProgArgs {
    port: String,
    sql_path: String
}

impl ProgArgs {
    fn default_value() -> ProgArgs {
        return ProgArgs { port: "8081".to_string(), sql_path: "./db.sqlite".to_string() };
    }
}

fn get_args() -> Vec<String> {
    let ret: Vec<String> = env::args().collect();
    return ret;
}

fn parse_args(args: Vec<String>) -> Option<ProgArgs> {
    let mut res: ProgArgs = ProgArgs::default_value();
    let mut idx = 1;
    while idx < args.len() {
        let current_arg = &args[idx];
        match &current_arg[..] {
            "-p" | "--port" => {
                if idx + 1 <= args.len() -1 {
                    res.port = args[idx+1].clone();
                }
                idx += 1;
            }
            "-d" | "--databse" => {
                if idx + 1 <= args.len() -1 {
                    res.sql_path = args[idx+1].clone();
                }
                idx += 1;
            }
            _ => {}
        }
        idx += 1;
    }
    return Some(res);
}

#[tokio::main]
async fn main() {
    let args = get_args();
    let parsed: ProgArgs = parse_args(args).unwrap_or(ProgArgs::default_value());
    let ip: &str = "0.0.0.0";
    let port: &str = &parsed.port;

    set_sql_path_val(&parsed.sql_path);

    let combine: &str = &format!("{}:{}", ip, port);

    let app = Router::new()
        .route("/get_tag", get(get_tag))
        .route("/search", get(search_book))
        .route("/get_book_info", get(get_book_info))
        .route("/get_book_from_tag", get(get_book_from_tag));

    let addr = tokio::net::TcpListener::bind(combine).await.unwrap();

    println!("Server running at http://{}:{}", ip, port);

    axum::serve(addr, app).await.unwrap();
}
