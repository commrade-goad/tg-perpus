use rusqlite::{params, Connection, Result};
use std::cell::RefCell;

thread_local!(static SQL_PATH: RefCell<String> = RefCell::new(String::new()));

fn get_sql_path_val() -> String {
    let mut ret:String = String::new();
    SQL_PATH.with(|text| {
        ret.push_str(&text.borrow().clone());
    });
    return ret
}

pub fn set_sql_path_val(str: &str) -> () {
    SQL_PATH.with(|text| {
        *text.borrow_mut() = str.to_string();
    });
    return
}

pub fn test() -> Result<()> {
    println!("{}", get_sql_path_val());
    Ok(())
}
