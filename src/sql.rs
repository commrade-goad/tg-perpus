use crate::book::{self, Tag};
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

enum AllTable {
    Book,
    AllTags,
    BookTags,
}

pub fn is_valid_sort(sort: &str) -> bool {
    let new_str: &str = &sort.to_uppercase();
    match &new_str.to_uppercase()[..] {
        "ASC" => return true,
        "DESC" => return true,
        _ => return false,
    }
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

pub async fn sql_read_tags(from: i32, range: i32, sort_mode: String) -> Result<Vec<book::Tag>, ()> {
    tokio::task::spawn_blocking(move || {
        let mut res: Vec<book::Tag> = Vec::new();
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);
        let mut stmt = conn
            .prepare(&format!(
                "SELECT tags_id, name FROM all_tags ORDER BY name {} limit {} offset {}",
                sort_mode, range, from,
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
    sort_mode: String,
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
                WHERE at.tags_id = {} ORDER BY b.title {} limit {} offset {}",
                tag_id, sort_mode, lim, off,
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
                .prepare(&format!(
                    "
                SELECT at.name, at.tags_id 
                FROM book_tags bt 
                JOIN all_tags at ON bt.tags_id = at.tags_id 
                WHERE bt.book_id = ? ORDER BY at.name {}
                ",
                    sort_mode
                ))
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

pub fn sql_read_book(sort_mode: String) -> Result<Vec<book::Book>> {
    let mut res: Vec<book::Book> = Vec::new();
    let conn = Connection::open(get_sql_path_val())?;
    let _ = check_all_table(&conn);

    // Get all books with their details
    let mut stmt = conn.prepare(&format!(
        "SELECT book_id, title, author, desc, year, cover FROM book ORDER BY title {}",
        sort_mode
    ))?;
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
        let mut tag_stmt = conn.prepare(&format!(
            "
            SELECT at.name, at.tags_id 
            FROM book_tags bt 
            JOIN all_tags at ON bt.tags_id = at.tags_id 
            WHERE bt.book_id = ? ORDER BY at.name {}",
            sort_mode
        ))?;
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
    return Ok(res);
}

pub async fn sql_get_book_info(book_id: i32, sort_mode: String) -> Result<book::Book, ()> {
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
                    WHERE bt.book_id = {} ORDER BY at.name {}",
                    book_id, sort_mode
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

pub async fn sql_search_title(title: &str, sort_mode: String) -> Result<Vec<book::Book>, ()> {
    let title_str: String = title.to_string();
    tokio::task::spawn_blocking(move || {
        let mut res: Vec<book::Book> = Vec::new();
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);

        // Get all books with their details
        let mut stmt =
            conn.prepare(&format!("SELECT book_id, title, author, desc, year, cover FROM book WHERE title = {} ORDER BY title {}", title_str, sort_mode)).unwrap();
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
                .prepare(&format!(
                    "
                SELECT at.name, at.tags_id 
                FROM book_tags bt 
                JOIN all_tags at ON bt.tags_id = at.tags_id 
                WHERE bt.book_id = ? ORDER BY at.name {}
                ",
                    sort_mode
                ))
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
        return Ok::<Vec<book::Book>, ()>(res);
    });
    return Err(());
}

pub async fn sql_search_author(author: &str, sort_mode: String) -> Result<Vec<book::Book>, ()> {
    let author_str: String = author.to_string();
    tokio::task::spawn_blocking(move || {
        let mut res: Vec<book::Book> = Vec::new();
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);

        // Get all books with their details
        let mut stmt =
            conn.prepare(&format!("SELECT book_id, title, author, desc, year, cover FROM book WHERE title = {} ORDER BY title {}", author_str, sort_mode)).unwrap();
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
                .prepare(&format!(
                    "
                SELECT at.name, at.tags_id 
                FROM book_tags bt 
                JOIN all_tags at ON bt.tags_id = at.tags_id 
                WHERE bt.book_id = ? ORDER BY at.name {}
                ",
                    sort_mode
                ))
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
        return Ok::<Vec<book::Book>, ()>(res);
    });
    return Err(());
}

pub async fn sql_add_new_tag(tag_name: &str, img: &str) -> Result<usize, ()> {
    let tag_name = tag_name.replace("'", "''");
    let img = img.to_string();

    tokio::task::spawn_blocking(move || {
        let conn = Connection::open(get_sql_path_val()).map_err(|_| ())?;
        check_all_table(&conn).map_err(|_| ())?;

        // Get the count of existing tags
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM all_tags", [], |row| row.get(0))
            .map_err(|_| ())?;

        // Insert the new tag using parameterized query
        conn.execute(
            "INSERT INTO all_tags (tags_id, name, img) VALUES (?, ?, ?)",
            params![count + 1, tag_name, img],
        )
        .map_err(|_| ())
    })
    .await
    .map_err(|_| ())?
}

pub async fn sql_add_new_book(book_name: &str, author: &str, tags_id: &str, year: &str, desc: &str,img: &str) -> Result<usize, ()> {
    let title = book_name.replace("'", "''");
    let auth = author.to_string();
    let y = year.to_string();
    let d = desc.replace("'", "''");
    let img = img.to_string();
    let tags_arr:Vec<String> = tags_id.split_whitespace().map(|s| s.to_string()).collect();


    tokio::task::spawn_blocking(move || {
        let conn = Connection::open(get_sql_path_val()).unwrap();
        let _ = check_all_table(&conn);

        // Get the count of existing tags
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM book", [], |row| row.get(0))
            .map_err(|_| ())?;

        // Insert the new tag using parameterized query
        let _ = conn.execute(
            "INSERT INTO book (book_id, title, author, desc, year, cover) VALUES (?, ?, ?, ?, ?, ?)",
            params![count + 1, title, auth, d, y, img],
        )
        .map_err(|_| ());

        let mut idx = 0;
        for tag in tags_arr {
            let tag_int: i32 = tag.trim().parse().map_err(|_| ())?;
            let btag: String = format!("{}-{}", count + 1, idx);

            let result = conn.execute(
                "INSERT INTO book_tags (btag_id, book_id, tags_id) VALUES (?, ?, ?)",
                params![btag, count + 1, tag_int],
            );

            if let Err(e) = result {
                println!("Error inserting into book_tags: {:?}", e);
                return Err(()); // Adjust as needed
            }
            idx += 1;
        }
        Ok(1)
    })
    .await
    .map_err(|_| ())?
}
