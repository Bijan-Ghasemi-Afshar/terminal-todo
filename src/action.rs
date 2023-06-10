use crate::todo::ToDo;
use std::fmt::Display;
use std::io::{self, stdout, Write};

pub mod database;

const CREATE: &'static str = "create";
const LIST: &'static str = "list";
const EDIT: &'static str = "edit";
const DONE: &'static str = "done";
const UNDONE: &'static str = "undone";
const DELETE: &'static str = "delete";

#[derive(PartialEq, Debug)]
pub struct Action {
    pub name: &'static str,
    pub requires_arguments: bool,
    pub arguments: Vec<String>,
    pub execute: fn(Vec<String>) -> Result<(), &'static str>,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Action: {}\nRequired Arguments: {:?}\nArguments{:?}",
            self.name, self.requires_arguments, self.arguments
        )
    }
}

fn list(_: Vec<String>) -> Result<(), &'static str> {
    println!("Printing all ToDo items");
    let todos: Vec<ToDo> = database::read_items()?;

    todos.iter().enumerate().for_each(|(index, todo)| {
        println!(
            "===============\n# {}\n{todo}\n===============\n",
            index + 1
        )
    });

    Ok(())
}

fn create(_: Vec<String>) -> Result<(), &'static str> {
    println!("Creating a ToDo item");

    print!("Title: ");
    stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    print!("Description: ");
    stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    let todo: ToDo = ToDo::new(title, description);

    database::store_item(todo)?;

    Ok(())
}

fn get_item_index_arg(args: Vec<String>) -> Result<usize, &'static str> {
    match args.get(0) {
        Some(arg) => match arg.parse::<usize>() {
            Ok(index) => {
                if index == 0 {
                    return Err("Given argument should be a positive number");
                }
                return Ok(index - 1);
            }
            Err(_) => return Err("Given argument should be a positive number"),
        },
        None => return Err("Could not get passed argument"),
    };
}

fn edit(args: Vec<String>) -> Result<(), &'static str> {
    let item_index: usize = get_item_index_arg(args)?;

    println!("Editing #{} ToDo item", item_index + 1);
    let mut todos: Vec<ToDo> = database::read_items()?;

    let mut edit_todo = match todos.get_mut(item_index) {
        Some(todo) => todo,
        None => return Err("Given ToDo Item index is wrong"),
    };

    println!("{edit_todo}");

    print!("New Title: ");
    stdout().flush().unwrap();
    let mut new_title = String::new();
    io::stdin().read_line(&mut new_title).unwrap();

    print!("New Description: ");
    stdout().flush().unwrap();
    let mut new_description = String::new();
    io::stdin().read_line(&mut new_description).unwrap();

    if new_title.len() > 1 {
        edit_todo.title = new_title;
    }

    if new_description.len() > 1 {
        edit_todo.description = new_description;
    }

    database::store_existing_items(todos)?;

    Ok(())
}

fn update_todo_status(args: Vec<String>, status: String) -> Result<(), &'static str> {
    let item_index: usize = get_item_index_arg(args)?;

    println!("Editing #{} ToDo item", item_index + 1);
    let mut todos: Vec<ToDo> = database::read_items()?;

    let mut edit_todo = match todos.get_mut(item_index) {
        Some(todo) => todo,
        None => return Err("Given ToDo Item index is wrong"),
    };

    edit_todo.done = status;

    database::store_existing_items(todos)?;

    Ok(())
}

fn done(args: Vec<String>) -> Result<(), &'static str> {
    update_todo_status(args, "✅".into())?;
    Ok(())
}

fn undone(args: Vec<String>) -> Result<(), &'static str> {
    update_todo_status(args, "❌".into())?;
    Ok(())
}

fn delete(args: Vec<String>) -> Result<(), &'static str> {
    let item_index: usize = get_item_index_arg(args)?;

    let mut todos: Vec<ToDo> = database::read_items()?;

    if item_index >= todos.len() || item_index <= 0 {
        return Err("Given item index is wrong");
    }

    println!("Deleting #{} ToDo item", item_index + 1);

    todos.remove(item_index);

    database::store_existing_items(todos)?;

    Ok(())
}

pub const ACTIONS: [Action; 6] = [
    Action {
        name: CREATE,
        requires_arguments: false,
        arguments: vec![],
        execute: create,
    },
    Action {
        name: LIST,
        requires_arguments: false,
        arguments: vec![],
        execute: list,
    },
    Action {
        name: EDIT,
        requires_arguments: true,
        arguments: vec![],
        execute: edit,
    },
    Action {
        name: DONE,
        requires_arguments: true,
        arguments: vec![],
        execute: done,
    },
    Action {
        name: UNDONE,
        requires_arguments: true,
        arguments: vec![],
        execute: undone,
    },
    Action {
        name: DELETE,
        requires_arguments: true,
        arguments: vec![],
        execute: delete,
    },
];
