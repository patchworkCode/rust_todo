use std::io;
use clap::{App, Arg};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use todo::{create_item,retrieve_list, Item};
use rusqlite::{params, Connection, Result};

const COMPLETE: char = '';
const INCOMPLETE: char = '';

fn main() -> Result<()> {
    let conn = Connection::open("todo.db")?;
    conn.execute(
        "
            create table if not exists todo (
                id integer primary key,
                body text not null unique,
                complete integer not null
             )
        ",
        [],
    )?;
    
    //let test_item = Item::new(String::from("second item"));
    //create_item(&conn, test_item)

    let todo = retrieve_list(&conn).unwrap();
    
    for item in todo {
        match item.complete {
            true => println!("{} {}", COMPLETE, item.content),
            _ => println!("{} {}", INCOMPLETE, item.content)
        }
    }

    Ok(())

}
