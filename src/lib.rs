use rusqlite::{params, Error, Connection, Result};

#[derive()]
pub struct Item {
    pub content: String,
    pub complete: bool
}

impl Item {
    pub fn new(content: String, complete: bool) -> Self{
        Self{
            content,
            complete,
        }
    }

    fn complete(&mut self) {
        self.complete = true;
    }
}

pub fn create_item(conn: &Connection, item: Item) -> Result<()> {

    conn.execute("INSERT INTO todo (body, complete) VALUES (?1, ?2)",
                params![item.content, 0]
    )?;

    Ok(())
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

pub fn retrieve_item(conn: &Connection) {
    todo!()
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
            _ => Err(rusqlite::Error::InvalidQuery)//needs to be improved upon
        }
    })?;

    for item in todo_iter {
        match item {
            Ok(item) =>  all_items.push(item),
            Err(e) => return Err(e),
        }
    }

    Ok(all_items)
}


//pub struct Todo {
//    list: Vec<Item>
//}
//
//impl Todo {
//    fn new(list: Vec<Item>) -> Self {
//        Self{
//            list
//        }
//    }
//    
//    fn add(&mut self, item: Item)  {
//        self.list.push(item);
//    }
//
//    fn delete(&mut self, index: usize) {
//        self.list.remove(index);
//    }
//
//    fn complete(&mut self, index: usize) {
//        self.list[index].complete();
//    }
//}

