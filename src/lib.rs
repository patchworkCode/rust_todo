use rusqlite::{params, Connection, Result};
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum AddCliError {
    Io(io::Error),
    Insert(rusqlite::Error),
    NullItem,
}

#[derive(Debug)]
pub struct Item {
    pub content: String,
    pub complete: bool,
}

impl Item {
    pub fn new(content: String, complete: bool) -> Self {
        Self { content, complete }
    }
}

pub fn begin_connection() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;
    conn.execute(
        "
            CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY,
                body TEXT NOT NULL UNIQUE,
                complete INTEGER NOT NULL
             )
        ",
        [],
    )?;

    Ok(conn)
}

pub fn create_item(conn: &Connection, item: Item) -> Result<()> {
    conn.execute(
        "INSERT INTO todo (body, complete) VALUES (?1, ?2)",
        params![item.content, 0],
    )?;

    Ok(())
}

pub fn retrieve_item(conn: &Connection, index: i8) -> Result<Item, rusqlite::Error> {
    let item = conn.query_row(
        "SELECT item FROM todo WHERE id=?",
        [index.to_string()],
        |row| {
            let content = row.get(1)?;
            let complete: i8 = row.get(2)?;
            match complete {
                0 => Ok(Item::new(content, false)),
                1 => Ok(Item::new(content, true)),
                _ => Err(rusqlite::Error::InvalidQuery),
            }
        },
    )?;
    Ok(item)
}

pub fn does_exist_item(conn: &Connection, index: i8) -> Result<i8> {
    conn.query_row(
        "SELECT EXISTS (SELECT * FROM todo LIMIT 1 OFFSET ?)",
        [index - 1],
        |row| {
            let eval: i8 = row.get(0)?;
            return Ok(eval);
        },
    )
}

pub fn delete_item(conn: &Connection, index: i8) -> Result<String> {
    match does_exist_item(conn, index)? {
        1 => {
            conn.execute(
                "DELETE FROM todo WHERE id in (SELECT id FROM todo LIMIT 1 OFFSET ?)",
                [index - 1],
            )?;
            Ok("Item successfully deleted.".to_string())
        }
        _ => Ok("Item does not exist.".to_string()),
    }
}

pub fn complete_item(conn: &Connection, index: i8) -> Result<String> {
    match does_exist_item(conn, index)? {
        1 => {
            conn.execute(
                "UPDATE todo SET complete = 1 WHERE id in (SELECT id FROM todo LIMIT 1 OFFSET ?)",
                [index - 1],
            )?;
            Ok("Item successfully completed.".to_string())
        }
        _ => Ok("Item does not exist.".to_string()),
    }
}

pub fn retrieve_list(conn: &Connection) -> Result<Vec<Item>> {
    let mut all_items: Vec<Item> = vec![];
    let mut stmt = conn.prepare("SELECT id, body, complete FROM todo")?;

    let todo_iter = stmt.query_map([], |row| {
        let content = row.get(1)?;
        let complete: i8 = row.get(2)?;
        match complete {
            0 => Ok(Item::new(content, false)),
            1 => Ok(Item::new(content, true)),
            _ => Err(rusqlite::Error::InvalidQuery), //needs to be improved upon
        }
    })?;

    for item in todo_iter {
        match item {
            Ok(item) => all_items.push(item),
            Err(e) => return Err(e),
        }
    }

    Ok(all_items)
}

pub fn process_add(conn: &Connection) -> Result<(), AddCliError> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut content = String::new();
    match reader.read_line(&mut content) {
        Ok(n) => match n {
            1 => Err(AddCliError::NullItem),
            _ => {
                let item = Item::new(content.trim().to_string(), false);
                match create_item(conn, item) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(AddCliError::Insert(error)),
                }
            }
        },
        Err(error) => Err(AddCliError::Io(error)),
    }
}
