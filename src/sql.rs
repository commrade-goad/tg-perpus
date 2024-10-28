use crate::book::{self, Tag};
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

enum AllTable {
    Book,
    AllTags,
    BookTags,
}

lazy_static::lazy_static! {
    static ref SORT_MODE_ASC: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

fn get_sort_mode_val() -> String {
    let locked_buf = SORT_MODE_ASC.lock().unwrap();
    if locked_buf.clone() == true {
        return "ASC".to_string();
    }
    return "DESC".to_string();
}

pub fn set_sort_mode_to_asc(val: bool) -> Result<(), >{
    let mut locked_path = SORT_MODE_ASC.lock().unwrap();
    *locked_path = val;
    return Ok(())
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
        }
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
        }
    }
    Ok(())
}

fn check_table_existance(conn: &Connection, table_name: &str, sql_table: AllTable) -> Result<()> {
    let statement: &str = &format!(
        "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='{}'",
        table_name
    );
    let table_exists: Result<bool> = conn.query_row(statement, [], |row| row.get(0));
    if table_exists.is_err() || !table_exists.unwrap() {
        create_tables(&conn, sql_table)?;
    }
    Ok(())
}

fn check_all_table(conn: &Connection) -> Result<()> {
    check_table_existance(&conn, "book", AllTable::Book)?;
    check_table_existance(&conn, "book_tags", AllTable::BookTags)?;
    check_table_existance(&conn, "all_tags", AllTable::AllTags)?;
    return Ok(());
}

pub async fn sql_read_tags(from: i32, range: i32) -> Result<Vec<book::Tag>, ()> {
    tokio::task::spawn_blocking(move || {
        let mut res: Vec<book::Tag> = Vec::new();
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);
        let mut stmt = conn
            .prepare(&format!(
                "SELECT tags_id, name FROM all_tags limit {} offset {}",
                range, from
            ))
            .unwrap();
        let tags_iter = stmt
            .query_map([], |row| {
                Ok(book::Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .unwrap();
        for tag in tags_iter {
            res.push(tag.unwrap());
        }
        return Ok::<Vec<book::Tag>, ()>(res);
    })
    .await
    .unwrap()
}

pub async fn sql_read_specified_tagged_book(
    tag_id: i32,
    lim: i32,
    off: i32,
) -> Result<Vec<book::Book>, ()> {
    tokio::task::spawn_blocking(move || {
        let mut res: Vec<book::Book> = Vec::new();
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);

        // Get all books with their details
        let mut stmt = conn
            .prepare(&format!(
                "SELECT b.book_id, b.title, b.author, b.desc, b.year, b.cover
            FROM book b
            JOIN book_tags bt ON b.book_id = bt.book_id
            JOIN all_tags at ON bt.tags_id = at.tags_id
            WHERE at.tags_id = {} limit {} offset {}",
                tag_id, lim, off
            ))
            .unwrap();
        let books_iter = stmt
            .query_map([], |row| {
                Ok(book::Book {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    author: row.get(2)?,
                    desc: row.get(3)?,
                    tags: vec![], // Placeholder for tags, will fill this later
                    year: row.get(4)?,
                    cover: row.get(5)?,
                })
            })
            .unwrap();

        // Loop through each book and fetch tags
        for book in books_iter {
            let mut book_data = book.unwrap();

            // Fetch tags for the current book_id
            let mut tag_stmt = conn
                .prepare(
                    "
                SELECT at.name, at.tags_id 
                FROM book_tags bt 
                JOIN all_tags at ON bt.tags_id = at.tags_id 
                WHERE bt.book_id = ?
                ",
                )
                .unwrap();
            let tag_iter = tag_stmt
                .query_map(params![book_data.id], |row| {
                    let tag_name: String = row.get(0)?;
                    let tag_id: i32 = row.get(1)?;
                    let tmp_res: Tag = Tag {
                        id: tag_id,
                        name: tag_name,
                    };
                    Ok(tmp_res) // Convert tags_id to String
                })
                .unwrap();

            // Collect tags into the book struct
            for tag in tag_iter {
                book_data.tags.push(tag.unwrap());
            }

            res.push(book_data);
        }
        Ok(res)
    })
    .await
    .unwrap()
}

pub fn sql_read_book() -> Result<Vec<book::Book>> {
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
        let mut tag_stmt = conn.prepare(
            "
            SELECT at.name, at.tags_id 
            FROM book_tags bt 
            JOIN all_tags at ON bt.tags_id = at.tags_id 
            WHERE bt.book_id = ?
        ",
        )?;
        let tag_iter = tag_stmt.query_map(params![book_data.id], |row| {
            let tag_name: String = row.get(0)?;
            let tag_id: i32 = row.get(1)?;
            let tmp_res: Tag = Tag {
                id: tag_id,
                name: tag_name,
            };
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

pub async fn sql_get_book_info(book_id: i32) -> Result<book::Book, ()> {
    tokio::task::spawn_blocking(move || {
        let res: book::Book;
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);

        // Get all books with their details
        let mut stmt = conn
            .prepare(&format!(
                "SELECT book_id, title, author, desc, year, cover FROM book where book_id = {}",
                book_id
            ))
            .unwrap();
        let books_iter = stmt
            .query_map([], |row| {
                Ok(book::Book {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    author: row.get(2).unwrap(),
                    desc: row.get(3).unwrap(),
                    tags: vec![], // Placeholder for tags, will fill this later
                    year: row.get(4).unwrap(),
                    cover: row.get(5).unwrap(),
                })
            })
            .unwrap();

        // Loop through each book and fetch tags
        for book in books_iter {
            let mut book_data = book.unwrap();

            // Fetch tags for the current book_id
            let mut tag_stmt = conn
                .prepare(&format!(
                    "
                    SELECT at.name, at.tags_id 
                    FROM book_tags bt 
                    JOIN all_tags at ON bt.tags_id = at.tags_id 
                    WHERE bt.book_id = {}",
                    book_id
                ))
                .unwrap();
            let tag_iter = tag_stmt
                .query_map([], |row| {
                    let tag_name: String = row.get(0).unwrap();
                    let tag_id: i32 = row.get(1).unwrap();
                    let tmp_res: Tag = Tag {
                        id: tag_id,
                        name: tag_name,
                    };
                    Ok(tmp_res) // Convert tags_id to String
                })
                .unwrap();

            // Collect tags into the book struct
            for tag in tag_iter {
                book_data.tags.push(tag.unwrap());
            }

            res = book_data;
            return Ok(res);
        }
        return Err(());
    })
    .await
    .unwrap()
}

pub async fn sql_del_book_from_id(book_id: i32) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        let conn = Connection::open(get_sql_path_val())?;
        let _ = check_all_table(&conn);
        conn.execute(
            "
                DELETE FROM book WHERE book_id = ?
        ",
            [book_id],
        )?;
        conn.execute(
            "
            DELETE FROM book_tags where book_id = ?
        ",
            [book_id],
        )?;
        return Ok(());
    })
    .await
    .unwrap()
}

pub async fn sql_del_tag_from_id(tag_id: i32) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        let conn = Connection::open(get_sql_path_val())?;
        let _ = check_all_table(&conn);
        conn.execute(
            "
                DELETE FROM all_tags WHERE tags_id = ?
            ",
            [tag_id],
        )?;
        conn.execute(
            "
                DELETE FROM book_tags where tags_id = ?
            ",
            [tag_id],
        )?;
        return Ok(());
    })
    .await
    .unwrap()
}

pub async fn sql_search_title(title: &str) -> Result<Vec<book::Book>, ()> {
    let title_str: String = title.to_string();
    tokio::task::spawn_blocking(move || {
        let mut res: Vec<book::Book> = Vec::new();
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);

        // Get all books with their details
        let mut stmt =
            conn.prepare(&format!("SELECT book_id, title, author, desc, year, cover FROM book WHERE title = {}", title_str)).unwrap();
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
        }).unwrap();

        // Loop through each book and fetch tags
        for book in books_iter {
            let mut book_data = book.unwrap();

            // Fetch tags for the current book_id
            let mut tag_stmt = conn.prepare(
                "
                SELECT at.name, at.tags_id 
                FROM book_tags bt 
                JOIN all_tags at ON bt.tags_id = at.tags_id 
                WHERE bt.book_id = ?
                ",
            ).unwrap();
            let tag_iter = tag_stmt.query_map(params![book_data.id], |row| {
                let tag_name: String = row.get(0)?;
                let tag_id: i32 = row.get(1)?;
                let tmp_res: Tag = Tag {
                    id: tag_id,
                    name: tag_name,
                };
                Ok(tmp_res) // Convert tags_id to String
            }).unwrap();

            // Collect tags into the book struct
            for tag in tag_iter {
                book_data.tags.push(tag.unwrap());
            }

            res.push(book_data);
        }
        return Ok::<Vec<book::Book>, ()>(res);
    });
    return Err(());
}

pub async fn sql_search_author(author: &str) -> Result<Vec<book::Book>, ()> {
    let author_str: String = author.to_string();
    tokio::task::spawn_blocking(move || {
        let mut res: Vec<book::Book> = Vec::new();
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);

        // Get all books with their details
        let mut stmt =
            conn.prepare(&format!("SELECT book_id, title, author, desc, year, cover FROM book WHERE title = {}", author_str)).unwrap();
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
        }).unwrap();

        // Loop through each book and fetch tags
        for book in books_iter {
            let mut book_data = book.unwrap();

            // Fetch tags for the current book_id
            let mut tag_stmt = conn.prepare(
                "
                SELECT at.name, at.tags_id 
                FROM book_tags bt 
                JOIN all_tags at ON bt.tags_id = at.tags_id 
                WHERE bt.book_id = ?
                ",
            ).unwrap();
            let tag_iter = tag_stmt.query_map(params![book_data.id], |row| {
                let tag_name: String = row.get(0)?;
                let tag_id: i32 = row.get(1)?;
                let tmp_res: Tag = Tag {
                    id: tag_id,
                    name: tag_name,
                };
                Ok(tmp_res) // Convert tags_id to String
            }).unwrap();

            // Collect tags into the book struct
            for tag in tag_iter {
                book_data.tags.push(tag.unwrap());
            }

            res.push(book_data);
        }
        return Ok::<Vec<book::Book>, ()>(res);
    });
    return Err(());
}
