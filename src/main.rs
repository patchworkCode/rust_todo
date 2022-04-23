use clap::{arg, ArgGroup, Command};
use rusqlite::Result;
use todo::*;

const COMPLETE: char = '';
const INCOMPLETE: char = '';

fn main() -> Result<()> {
    let args = Command::new("rust-todo")
        .arg(arg!(-a --add "list item to be added"))
        .arg(arg!(-l --list "lists items in the the todo list"))
        .arg(arg!(-d --delete <index> "remove a todo item"))
        .arg(arg!(-c --complete <index> "complete a list item"))
        .group(
            ArgGroup::new("CRUD")
                .args(&["add", "list", "delete", "complete"])
                .required(true),
        )
        .get_matches();

    let conn = begin_connection()?;

    if args.is_present("add") {
        match process_add(&conn) {
            Ok(_) => println!("Item succesfully added."),
            Err(error) => println!("There was an error: {:#?}", error),
        }
    } else if args.is_present("list") {
        let todo = retrieve_list(&conn)?;
        match todo.is_empty() {
            true => println!("Todo list is empty, please add an item."),
            false => {
                for item in todo {
                    match item.complete {
                        true => println!("{} {}", COMPLETE, item.content),
                        false => println!("{} {}", INCOMPLETE, item.content),
                    }
                }
            }
        }
    } else if args.is_present("delete") {
        let index: i8 = args.value_of("delete").unwrap().parse().unwrap();
        let result = delete_item(&conn, index)?;
        println!("{}", result)
    } else if args.is_present("complete") {
        let index: i8 = args.value_of("complete").unwrap().parse().unwrap();
        let result = complete_item(&conn, index)?;
        println!("{}", result)
    }

    Ok(())
}
