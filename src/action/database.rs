use std::{fs::File, io::Write};

use crate::todo::ToDo;

const DATABASE: &str = "todo-list.txt";

pub fn store_item(todo: ToDo) {
    let mut database: File = File::options()
        .append(true)
        .create(true)
        .open(DATABASE)
        .expect("Error finding the database");

    database
        .write(&format!("{todo}\n").as_bytes())
        .expect("Error storing ToDo item");
}
