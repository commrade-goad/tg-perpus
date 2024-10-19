use crate::book::{self, Tag};
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

enum AllTable {
    Book,
    AllTags,
    BookTags
}

lazy_static::lazy_static! {
    static ref SQL_PATH: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

fn get_sql_path_val() -> String {
    let locked_path = SQL_PATH.lock().unwrap();
    locked_path.clone() // Return a cloned String
}

pub fn set_sql_path_val(path: &str) {
    let mut locked_path = SQL_PATH.lock().unwrap();
    *locked_path = path.to_string(); // Set the new path
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

fn check_all_table(conn: &Connection) -> Result<()> {
    check_table_existance(&conn, "book", AllTable::Book)?;
    check_table_existance(&conn, "book_tags", AllTable::BookTags)?;
    check_table_existance(&conn, "all_tags", AllTable::AllTags)?;
    return Ok(())
}

pub fn read_tags() -> Result<Vec<book::Tag>> {
    let mut res: Vec<book::Tag> = Vec::new();
    let conn = Connection::open(get_sql_path_val())?;
    let _ = check_all_table(&conn);
    let mut stmt = conn.prepare("SELECT tags_id, name FROM all_tags")?;
    let tags_iter = stmt.query_map([], |row| {
        Ok(book::Tag {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;
    for tag in tags_iter {
        res.push(tag.unwrap());
    }
    return Ok(res)
}

pub fn read_book() -> Result<Vec<book::Book>> {
    let mut res: Vec<book::Book> = Vec::new();
    let conn = Connection::open(get_sql_path_val())?;
    let _ = check_all_table(&conn);

    // Get all books with their details
    let mut stmt = conn.prepare("SELECT book_id, title, author, desc, year, cover FROM book")?;
    let books_iter = stmt.query_map([], |row| {
        Ok(book::Book {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            desc: row.get(3)?,
            tags: vec![], // Placeholder for tags, will fill this later
            year: row.get(4)?,
            cover: row.get(5)?,
        })
    })?;

    // Loop through each book and fetch tags
    for book in books_iter {
        let mut book_data = book?;
        
        // Fetch tags for the current book_id
        let mut tag_stmt = conn.prepare("
            SELECT at.name, at.tags_id 
            FROM book_tags bt 
            JOIN all_tags at ON bt.tags_id = at.tags_id 
            WHERE bt.book_id = ?
        ")?;
        let tag_iter = tag_stmt.query_map(params![book_data.id], |row| {
            let tag_name: String = row.get(0)?;
            let tag_id: i32 = row.get(1)?;
            let tmp_res: Tag = Tag { id: tag_id, name: tag_name };
            Ok(tmp_res) // Convert tags_id to String
        })?;
        
        // Collect tags into the book struct
        for tag in tag_iter {
            book_data.tags.push(tag?);
        }

        res.push(book_data);
    }
    Ok(res)
}
