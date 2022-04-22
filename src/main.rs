use std::io;
use clap::{ArgGroup, Command, arg};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use todo::{create_item, begin_connection, retrieve_list, Item};
use rusqlite::{params, Connection};

const COMPLETE: char = '';
const INCOMPLETE: char = '';

fn main() -> Result<(), String> {
    let args = Command::new("rust-todo")
    .arg(arg!(-a --add <item> "list item to be added"))
    .arg(arg!(-l --list "lists items in the the todo list"))
    .group(ArgGroup::new("CRUD")
        .args(&["add", "list"])
        .required(true))
    .get_matches();


    let conn = begin_connection().unwrap(); 
    //let test_item = Item::new(String::from("second item"));
    //create_item(&conn, test_item)
    //let stdin = io::stdin();
    //let reader = stdin.lock();


    let todo = retrieve_list(&conn).unwrap();
    
    for item in todo {
        match item.complete {
            true => println!("{} {}", COMPLETE, item.content),
            false => println!("{} {}", INCOMPLETE, item.content)
        }
    }

    Ok(())

}
