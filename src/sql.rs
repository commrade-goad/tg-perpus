use crate::book;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

enum AllTable {
    Book,
    AllTags,
    BookTags
}

thread_local!(static SQL_PATH: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new())));

fn get_sql_path_val() -> String {
    SQL_PATH.with(|path| {
        let locked_path = path.lock().unwrap();
        locked_path.clone() // Return a cloned String
    })
}

pub fn set_sql_path_val(path: &str) {
    SQL_PATH.with(|sql_path| {
        let mut locked_path = sql_path.lock().unwrap();
        *locked_path = path.to_string(); // Set the new path
    });
}

fn create_tables(conn: &Connection, mode: AllTable) -> Result<()> {
    match mode {
        AllTable::Book => {
            conn.execute(
                "CREATE TABLE book (
                book_id INTEGER PRIMARY KEY,
                title TEXT,
                author TEXT,
                desc TEXT,
                year TEXT,
                cover TEXT
                )",
                [],
            )?;
        },
        AllTable::BookTags => {
            conn.execute(
                "CREATE TABLE book_tags (
                    btag_id TEXT PRIMARY KEY,
                    book_id INTEGER,
                    tags_id INTEGER,
                    FOREIGN KEY (book_id) REFERENCES book(book_id),
                    FOREIGN KEY (tags_id) REFERENCES all_tags(tags_id)
                )",
                [],
            )?;
        }
        AllTable::AllTags => {
            conn.execute(
                "CREATE TABLE all_tags (
                    tags_id INTEGER PRIMARY KEY,
                    name TEXT,
                    img TEXT
                )",
                [],
            )?;
        },
    }
    Ok(())
}

fn check_table_existance(conn: &Connection, table_name: &str, sql_table: AllTable) -> Result<()>{
    let statement: &str = &format!("SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='{}'", table_name);
    let table_exists: Result<bool> = conn
        .query_row(
            statement,
            [],
            |row| row.get(0),
        );
    if table_exists.is_err() || !table_exists.unwrap() {
        create_tables(&conn, sql_table)?;
    }
    Ok(())
}

pub fn read_book() -> Result<Vec<book::Book>> {
    let conn = Connection::open(get_sql_path_val())?;
    let _ = check_table_existance(&conn, "book", AllTable::Book);
    let _ = check_table_existance(&conn, "book_tags", AllTable::BookTags);
    let _ = check_table_existance(&conn, "all_tags", AllTable::AllTags);
    return Ok(Vec::new());
}
