use crate::book;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

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

pub fn read_book() -> Result<Vec<book::Book>> {
    let conn = Connection::open(get_sql_path_val())?;
    let table_exists: Result<bool> = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='book'",
            [],
            |row| row.get(0),
        );

    // If the table does not exist, create it
    /* if table_exists.is_err() || !table_exists.unwrap() {
        conn.execute(
            "CREATE TABLE book (
                book_id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                desc TEXT NOT NULL,
            )",
            [], // No parameters
        )?;
    } */
    return Ok(Vec::new());
}
