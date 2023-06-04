use std::{
    fs::File,
    io::{Read, Write},
};

use crate::todo::ToDo;

const DATABASE: &str = "todo-list.txt";

pub fn store_item(todo: ToDo) {
    let mut database: File = File::options()
        .append(true)
        .create(true)
        .open(DATABASE)
        .expect("Error finding the database");

    database
        .write(todo.serialise().as_bytes())
        .expect("Error storing ToDo item");

    println!("Item added to the database");
}

pub fn read_items() -> Vec<ToDo> {
    let mut database: File = File::options()
        .read(true)
        .open(DATABASE)
        .expect("Error openning the database");

    let mut todo_items: Vec<ToDo> = vec![];
    let mut db_content: String = String::new();

    database
        .read_to_string(&mut db_content)
        .expect("Error reading the database content");

    for todo_serialised in db_content.lines() {
        todo_items.push(ToDo::deserialise(todo_serialised));
    }

    todo_items
}
