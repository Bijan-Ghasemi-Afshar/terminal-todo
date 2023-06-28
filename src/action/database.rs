use std::{
    any::Any,
    env,
    fs::{DirBuilder, File},
    io::{Read, Write},
    vec,
};

use crate::todo::ToDo;

pub trait Database: Any {
    fn read_items(&self) -> Result<Vec<ToDo>, &'static str>;
    fn store_existing_items(&self, todos: Vec<ToDo>) -> Result<(), &'static str>;
    fn store_item(&self, todo: ToDo) -> Result<(), &'static str>;
}

const DATABASE_NAME: &str = "todo-list.txt";

pub struct DatabaseAgent {}

fn get_database() -> Result<String, &'static str> {
    let database_location = env::var("TODO_DB").unwrap_or_else(|_| {
        let db_base_path = if cfg!(target_os = "windows") {
            let mut windows_app_data_path = env::var("AppData").unwrap_or_default();

            windows_app_data_path = windows_app_data_path + "/.terminal-todo";

            let dir_build_res = DirBuilder::new()
                .recursive(true)
                .create(&windows_app_data_path);

            if let Err(_) = dir_build_res {
                return "Could not create DB directory path".into();
            }

            windows_app_data_path
        } else {
            let mut unix_home_path = env::var("HOME").unwrap_or_default();

            unix_home_path = unix_home_path + "/.terminal-todo";

            let dir_build_res = DirBuilder::new().recursive(true).create(&unix_home_path);

            if let Err(_) = dir_build_res {
                return "Could not create DB directory path".into();
            }

            unix_home_path
        };

        db_base_path
    });

    Ok(format!("{}/{}", database_location, DATABASE_NAME))
}

impl Database for DatabaseAgent {
    fn store_item(&self, todo: ToDo) -> Result<(), &'static str> {
        let db = get_database()?;

        let mut database: File = File::options()
            .append(true)
            .create(true)
            .open(db)
            .expect("Error finding the database");

        database
            .write(todo.serialise().as_bytes())
            .expect("Error storing ToDo item");

        println!("Item added to the database");

        Ok(())
    }

    fn store_existing_items(&self, todos: Vec<ToDo>) -> Result<(), &'static str> {
        let db = get_database()?;

        let mut database: File = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(db)
            .expect("Error openning the database");

        let mut serialised_todos = Vec::new();

        todos.iter().for_each(|todo| {
            serialised_todos.push(todo.serialise());
        });

        let serialised_todos = serialised_todos.join("");

        database
            .write(&serialised_todos.as_bytes())
            .expect("Error storing ToDo item");

        Ok(())
    }

    fn read_items(&self) -> Result<Vec<ToDo>, &'static str> {
        let db = get_database()?;

        let mut database: File = File::options()
            .write(true)
            .create(true)
            .read(true)
            .open(db)
            .expect("Error openning the database");

        let mut todo_items: Vec<ToDo> = vec![];
        let mut db_content: String = String::new();

        database
            .read_to_string(&mut db_content)
            .expect("Error reading the database content");

        for todo_serialised in db_content.lines() {
            todo_items.push(ToDo::deserialise(todo_serialised)?);
        }

        Ok(todo_items)
    }
}
